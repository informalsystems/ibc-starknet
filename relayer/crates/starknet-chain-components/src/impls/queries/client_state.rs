use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CanQueryBlock, CanQueryClientState, ClientStateQuerier, ClientStateQuerierComponent,
    ClientStateWithProofsQuerier, ClientStateWithProofsQuerierComponent, HasClientIdType,
    HasClientStateType, HasCommitmentProofType, HasHeightType, HasIbcCommitmentPrefix,
};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_prelude::*;
use ibc::core::host::types::path::{ClientStatePath, Path};
use starknet::core::types::Felt;
use starknet::macros::selector;
use starknet_crypto_lib::StarknetCryptoLib;
use starknet_storage_verifier::ibc::ibc_path_to_storage_key;
use starknet_v14::core::types::StorageProof;

use crate::traits::{
    CanCallContract, CanQueryContractAddress, CanQueryStorageProof, HasBlobType, HasSelectorType,
    HasStorageKeyType, HasStorageProofType,
};
use crate::types::{ClientId, CometClientState, StarknetChainStatus, StarknetCommitmentProof};

pub struct QueryCometClientState;

#[cgp_provider(ClientStateQuerierComponent)]
impl<Chain, Counterparty, Encoding> ClientStateQuerier<Chain, Counterparty>
    for QueryCometClientState
where
    Chain: HasClientIdType<Counterparty, ClientId = ClientId>
        + HasHeightType
        + CanCallContract
        + HasSelectorType<Selector = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_client_contract_address")>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasClientStateType<Chain, ClientState = CometClientState>,
    Encoding: CanEncode<ViaCairo, u64>
        + CanDecode<ViaCairo, Vec<Felt>>
        + CanDecode<ViaCairo, CometClientState>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_client_state(
        chain: &Chain,
        _tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        height: &Chain::Height,
    ) -> Result<Counterparty::ClientState, Chain::Error> {
        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let client_id_seq = client_id
            .as_str()
            .rsplit_once('-')
            .ok_or_else(|| Chain::raise_error("invalid client id"))?
            .1
            .parse::<u64>()
            .map_err(|_| Chain::raise_error("invalid sequence"))?;

        let calldata = encoding
            .encode(&client_id_seq)
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("client_state"),
                &calldata,
                Some(height),
            )
            .await?;

        let raw_client_state: Vec<Felt> = encoding.decode(&output).map_err(Chain::raise_error)?;

        let client_state: CometClientState = encoding
            .decode(&raw_client_state)
            .map_err(Chain::raise_error)?;

        Ok(client_state)
    }
}

#[cgp_provider(ClientStateWithProofsQuerierComponent)]
impl<Chain, Counterparty> ClientStateWithProofsQuerier<Chain, Counterparty>
    for QueryCometClientState
where
    Chain: HasClientIdType<Counterparty, ClientId = ClientId>
        + CanQueryStorageProof
        + HasStorageKeyType<StorageKey = Felt>
        + HasStorageProofType<StorageProof = StorageProof>
        + HasHeightType<Height = u64>
        + CanQueryBlock<Block = StarknetChainStatus>
        + HasIbcCommitmentPrefix<CommitmentPrefix = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + CanQueryClientState<Counterparty>
        + CanQueryContractAddress<symbol!("ibc_client_contract_address")>
        + CanRaiseAsyncError<serde_json::Error>,
    Counterparty: HasClientStateType<Chain, ClientState = CometClientState> + HasHeightType,
{
    async fn query_client_state_with_proofs(
        chain: &Chain,
        tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        query_height: &Chain::Height,
    ) -> Result<(Counterparty::ClientState, Chain::CommitmentProof), Chain::Error> {
        let contract_address = chain.query_contract_address(PhantomData).await?;

        let client_state = chain
            .query_client_state(tag, client_id, query_height)
            .await?;

        let block = chain.query_block(query_height).await?;

        let ibc_path = Path::ClientState(ClientStatePath::new(client_id.clone()));

        let felt_path: Felt = ibc_path_to_storage_key(&StarknetCryptoLib, ibc_path);

        // key == path
        let storage_proof: StorageProof = chain
            .query_storage_proof(query_height, &contract_address, &[felt_path])
            .await?;

        let storage_proof_bytes = serde_json::to_vec(&storage_proof).map_err(Chain::raise_error)?;

        let proof = StarknetCommitmentProof {
            proof_height: block.height,
            proof_bytes: storage_proof_bytes,
        };

        Ok((client_state, proof))
    }
}
