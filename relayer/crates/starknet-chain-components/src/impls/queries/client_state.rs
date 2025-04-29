use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CanQueryBlock, CanQueryClientState, ClientStateQuerier, ClientStateQuerierComponent,
    ClientStateWithProofsQuerier, ClientStateWithProofsQuerierComponent, HasClientIdType,
    HasClientStateType, HasCommitmentProofType, HasHeightType, HasIbcCommitmentPrefix,
};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_cosmos_core::chain_components::types::Secp256k1KeyPair;
use hermes_prelude::*;
use ibc::core::host::types::path::{ClientStatePath, Path};
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::traits::contract::call::CanCallContract;
use crate::traits::proof_signer::HasStarknetProofSigner;
use crate::traits::queries::contract_address::CanQueryContractAddress;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;
use crate::types::client_id::ClientId;
use crate::types::commitment_proof::StarknetCommitmentProof;
use crate::types::cosmos::client_state::CometClientState;
use crate::types::membership_proof_signer::MembershipVerifierContainer;
use crate::types::status::StarknetChainStatus;

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
        + HasHeightType<Height = u64>
        + CanQueryBlock<Block = StarknetChainStatus>
        + HasIbcCommitmentPrefix<CommitmentPrefix = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + CanQueryClientState<Counterparty>
        + HasStarknetProofSigner<ProofSigner = Secp256k1KeyPair>
        + CanRaiseAsyncError<String>
        + HasAsyncErrorType,
    Counterparty: HasClientStateType<Chain, ClientState = CometClientState> + HasHeightType,
{
    async fn query_client_state_with_proofs(
        chain: &Chain,
        tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        query_height: &Chain::Height,
    ) -> Result<(Counterparty::ClientState, Chain::CommitmentProof), Chain::Error> {
        // FIXME: properly fetch client state with proofs
        let client_state = chain
            .query_client_state(tag, client_id, query_height)
            .await?;

        let block = chain.query_block(query_height).await?;

        // FIXME: CometClientState can't be encoded to protobuf
        let protobuf_encoded_client_state = Vec::new();
        // let protobuf_encoded_client_state = Counterparty::default_encoding()
        //     .encode(&client_state)
        //     .map_err(Chain::raise_error)?;

        let unsigned_membership_proof_bytes = MembershipVerifierContainer {
            state_root: block.block_hash.to_bytes_be().to_vec(),
            prefix: chain.ibc_commitment_prefix().clone(),
            path: Path::ClientState(ClientStatePath::new(client_id.clone()))
                .to_string()
                .into(),
            value: Some(protobuf_encoded_client_state),
        }
        .canonical_bytes();

        let signed_bytes = chain
            .proof_signer()
            .sign(&unsigned_membership_proof_bytes)
            .map_err(Chain::raise_error)?;

        let proof = StarknetCommitmentProof {
            proof_height: block.height,
            proof_bytes: signed_bytes,
        };

        Ok((client_state, proof))
    }
}
