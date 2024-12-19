use cgp::prelude::HasErrorType;
use hermes_chain_components::traits::queries::packet_is_received::ReceivedPacketQuerier;
use hermes_chain_components::traits::types::ibc::{
    HasChannelIdType, HasPortIdType, HasSequenceType,
};

pub struct QueryPacketIsReceivedOnStarknet;

impl<Chain, Counterparty> ReceivedPacketQuerier<Chain, Counterparty>
    for QueryPacketIsReceivedOnStarknet
where
    Chain: HasChannelIdType<Counterparty> + HasPortIdType<Counterparty> + HasErrorType,
    Counterparty: HasSequenceType<Chain>,
{
    async fn query_packet_is_received(
        _chain: &Chain,
        _port_id: &Chain::PortId,
        _channel_id: &Chain::ChannelId,
        _sequence: &Counterparty::Sequence,
    ) -> Result<bool, Chain::Error> {
        todo!()
    }
}
