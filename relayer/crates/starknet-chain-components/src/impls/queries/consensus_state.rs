use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::queries::consensus_state::{
    CanQueryConsensusState, ConsensusStateQuerier, ConsensusStateWithProofsQuerier,
};
use hermes_chain_components::traits::types::consensus_state::HasConsensusStateType;
use hermes_chain_components::traits::types::height::{HasHeightFields, HasHeightType};
use hermes_chain_components::traits::types::ibc::HasClientIdType;
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::traits::contract::call::CanCallContract;
use crate::traits::queries::address::CanQueryContractAddress;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;
use crate::types::client_id::ClientId;
use crate::types::commitment_proof::StarknetCommitmentProof;
use crate::types::cosmos::consensus_state::CometConsensusState;
use crate::types::cosmos::height::Height;

#[derive(Debug)]
pub struct ConsensusStateNotFound {
    pub client_id: ClientId,
    pub height: Height,
}

pub struct QueryCometConsensusState;

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
        _query_height: &Chain::Height, // TODO: figure whether we can perform height specific queries on Starknet
    ) -> Result<Counterparty::ConsensusState, Chain::Error> {
        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let height = Height {
            revision_number: Counterparty::revision_number(consensus_height),
            revision_height: Counterparty::revision_height(consensus_height),
        };

        let calldata = encoding
            .encode(&(client_id.sequence, height.clone()))
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(&contract_address, &selector!("consensus_state"), &calldata)
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

impl<Chain, Counterparty> ConsensusStateWithProofsQuerier<Chain, Counterparty>
    for QueryCometConsensusState
where
    Chain: HasClientIdType<Counterparty>
        + HasHeightType<Height = u64>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + CanQueryConsensusState<Counterparty>
        + HasAsyncErrorType,
    Counterparty: HasConsensusStateType<Chain> + HasHeightType,
{
    async fn query_consensus_state_with_proofs(
        chain: &Chain,
        tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Chain::Height,
    ) -> Result<(Counterparty::ConsensusState, Chain::CommitmentProof), Chain::Error> {
        // FIXME: properly fetch consensus state with proofs
        let consensus_state = chain
            .query_consensus_state(tag, client_id, consensus_height, query_height)
            .await?;

        let proof = StarknetCommitmentProof {
            proof_height: *query_height,
            proof_bytes: Vec::new(),
        };

        Ok((consensus_state, proof))
    }
}
