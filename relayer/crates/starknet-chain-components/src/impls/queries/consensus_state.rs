use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CanQueryBlock, CanQueryConsensusState, ConsensusStateQuerier, ConsensusStateQuerierComponent,
    ConsensusStateWithProofsQuerier, ConsensusStateWithProofsQuerierComponent, HasClientIdType,
    HasCommitmentProofType, HasConsensusStateType, HasHeightFields, HasHeightType,
    HasIbcCommitmentPrefix,
};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_cosmos_core::chain_components::types::Secp256k1KeyPair;
use hermes_prelude::*;
use ibc::core::client::types::Height as IbcHeight;
use ibc::core::host::types::path::{ClientConsensusStatePath, Path};
use starknet::core::types::Felt;
use starknet::macros::selector;
use starknet_crypto_lib::StarknetCryptoLib;
use starknet_storage_verifier::ibc::ibc_path_to_storage_key;
use starknet_v14::core::types::StorageProof;

use crate::traits::{
    CanCallContract, CanQueryContractAddress, CanQueryStorageProof, HasBlobType, HasSelectorType,
    HasStarknetProofSigner, HasStorageKeyType, HasStorageProofType,
};
use crate::types::{
    ClientId, CometConsensusState, Height, StarknetChainStatus, StarknetCommitmentProof,
};
#[derive(Debug)]
pub struct ConsensusStateNotFound {
    pub client_id: ClientId,
    pub height: Height,
}

pub struct QueryCometConsensusState;

#[cgp_provider(ConsensusStateQuerierComponent)]
impl<Chain, Counterparty, Encoding> ConsensusStateQuerier<Chain, Counterparty>
    for QueryCometConsensusState
where
    Chain: HasClientIdType<Counterparty, ClientId = ClientId>
        + HasHeightType
        + CanCallContract
        + HasSelectorType<Selector = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_client_contract_address")>
        + CanRaiseAsyncError<ConsensusStateNotFound>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty:
        HasConsensusStateType<Chain, ConsensusState = CometConsensusState> + HasHeightFields,
    Encoding: CanEncode<ViaCairo, (u64, Height)>
        + CanDecode<ViaCairo, Vec<Felt>>
        + CanDecode<ViaCairo, CometConsensusState>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_consensus_state(
        chain: &Chain,
        _tag: PhantomData<Counterparty>,
        client_id: &ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Chain::Height,
    ) -> Result<Counterparty::ConsensusState, Chain::Error> {
        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let height = Height {
            revision_number: Counterparty::revision_number(consensus_height),
            revision_height: Counterparty::revision_height(consensus_height),
        };

        let client_id_seq = client_id
            .as_str()
            .rsplit_once('-')
            .ok_or_else(|| Chain::raise_error("invalid client id"))?
            .1
            .parse::<u64>()
            .map_err(|_| Chain::raise_error("invalid sequence"))?;

        let calldata = encoding
            .encode(&(client_id_seq, height.clone()))
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("consensus_state"),
                &calldata,
                Some(query_height),
            )
            .await?;

        let raw_consensus_state: Vec<Felt> =
            encoding.decode(&output).map_err(Chain::raise_error)?;

        let consensus_state: CometConsensusState = encoding
            .decode(&raw_consensus_state)
            .map_err(Chain::raise_error)?;

        // FIXME: Temporary workaround, as the current Cairo contract returns
        // default value when the entry is not found.
        if consensus_state.root.is_empty() {
            return Err(Chain::raise_error(ConsensusStateNotFound {
                client_id: client_id.clone(),
                height,
            }));
        }

        Ok(consensus_state)
    }
}

#[cgp_provider(ConsensusStateWithProofsQuerierComponent)]
impl<Chain, Counterparty> ConsensusStateWithProofsQuerier<Chain, Counterparty>
    for QueryCometConsensusState
where
    Chain: HasClientIdType<Counterparty, ClientId = ClientId>
        + CanQueryStorageProof
        + HasStorageKeyType<StorageKey = Felt>
        + HasStorageProofType<StorageProof = StorageProof>
        + HasHeightType<Height = u64>
        + CanQueryBlock<Block = StarknetChainStatus>
        + HasIbcCommitmentPrefix<CommitmentPrefix = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + CanQueryConsensusState<Counterparty>
        + HasStarknetProofSigner<ProofSigner = Secp256k1KeyPair>
        + CanQueryContractAddress<symbol!("ibc_client_contract_address")>
        + CanRaiseAsyncError<String>
        + HasAsyncErrorType,
    Counterparty: HasConsensusStateType<Chain, ConsensusState = CometConsensusState>
        + HasHeightType<Height = IbcHeight>,
{
    async fn query_consensus_state_with_proofs(
        chain: &Chain,
        tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Chain::Height,
    ) -> Result<(Counterparty::ConsensusState, Chain::CommitmentProof), Chain::Error> {
        let contract_address = chain.query_contract_address(PhantomData).await?;

        let consensus_state = chain
            .query_consensus_state(tag, client_id, consensus_height, query_height)
            .await?;

        let block = chain.query_block(query_height).await?;

        let ibc_path = Path::ClientConsensusState(ClientConsensusStatePath::new(
            client_id.clone(),
            consensus_height.revision_number(),
            consensus_height.revision_height(),
        ));

        let felt_path: Felt = ibc_path_to_storage_key::<StarknetCryptoLib>(ibc_path);

        // key == path
        let storage_proof: StorageProof = chain
            .query_storage_proof(query_height, &contract_address, &[felt_path])
            .await
            .unwrap();

        let storage_proof_bytes = serde_json::to_vec(&storage_proof).unwrap();

        let proof = StarknetCommitmentProof {
            proof_height: block.height,
            proof_bytes: storage_proof_bytes,
        };

        Ok((consensus_state, proof))
    }
}
