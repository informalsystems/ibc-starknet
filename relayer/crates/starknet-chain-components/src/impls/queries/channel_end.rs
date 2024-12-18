use cgp::prelude::HasErrorType;
use hermes_chain_components::traits::queries::channel_end::{
    ChannelEndQuerier, ChannelEndWithProofsQuerier,
};
use hermes_chain_components::traits::types::channel::HasChannelEndType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;

pub struct QueryChannelEndFromStarknet;

impl<Chain, Counterparty> ChannelEndQuerier<Chain, Counterparty> for QueryChannelEndFromStarknet
where
    Chain: HasHeightType
        + HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasChannelEndType<Counterparty>
        + HasErrorType,
{
    async fn query_channel_end(
        _chain: &Chain,
        _channel_id: &Chain::ChannelId,
        _port_id: &Chain::PortId,
        _height: &Chain::Height,
    ) -> Result<Chain::ChannelEnd, Chain::Error> {
        todo!()
    }
}

impl<Chain, Counterparty> ChannelEndWithProofsQuerier<Chain, Counterparty>
    for QueryChannelEndFromStarknet
where
    Chain: HasHeightType
        + HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasChannelEndType<Counterparty>
        + HasCommitmentProofType
        + HasErrorType,
{
    async fn query_channel_end_with_proofs(
        _chain: &Chain,
        _channel_id: &Chain::ChannelId,
        _port_id: &Chain::PortId,
        _height: &Chain::Height,
    ) -> Result<(Chain::ChannelEnd, Chain::CommitmentProof), Chain::Error> {
        todo!()
    }
}
