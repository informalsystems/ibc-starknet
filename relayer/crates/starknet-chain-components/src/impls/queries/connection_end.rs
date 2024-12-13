use cgp::prelude::HasErrorType;
use hermes_chain_components::traits::queries::connection_end::{
    ConnectionEndQuerier, ConnectionEndWithProofsQuerier,
};
use hermes_chain_components::traits::types::connection::HasConnectionEndType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::HasConnectionIdType;
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;

pub struct QueryConnectionEndFromStarknet;

impl<Chain, Counterparty> ConnectionEndQuerier<Chain, Counterparty>
    for QueryConnectionEndFromStarknet
where
    Chain: HasHeightType
        + HasConnectionIdType<Counterparty>
        + HasConnectionEndType<Counterparty>
        + HasErrorType,
{
    async fn query_connection_end(
        _chain: &Chain,
        _connection_id: &Chain::ConnectionId,
        _height: &Chain::Height,
    ) -> Result<Chain::ConnectionEnd, Chain::Error> {
        todo!()
    }
}

impl<Chain, Counterparty> ConnectionEndWithProofsQuerier<Chain, Counterparty>
    for QueryConnectionEndFromStarknet
where
    Chain: HasHeightType
        + HasCommitmentProofType
        + HasConnectionIdType<Counterparty>
        + HasConnectionEndType<Counterparty>
        + HasErrorType,
{
    async fn query_connection_end_with_proofs(
        _chain: &Chain,
        _connection_id: &Chain::ConnectionId,
        _height: &Chain::Height,
    ) -> Result<(Chain::ConnectionEnd, Chain::CommitmentProof), Chain::Error> {
        todo!()
    }
}
