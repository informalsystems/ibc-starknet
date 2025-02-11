use cgp::prelude::*;
use hermes_chain_components::traits::queries::send_packets::{
    SendPacketQuerier, SendPacketsQuerier,
};
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{
    HasChannelIdType, HasPortIdType, HasSequenceType,
};
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_cosmos_chain_components::components::client::{
    SendPacketQuerierComponent, SendPacketsQuerierComponent,
};

pub struct QueryStarknetSendPacket;

#[cgp_provider(SendPacketsQuerierComponent)]
impl<Chain, Counterparty> SendPacketsQuerier<Chain, Counterparty> for QueryStarknetSendPacket
where
    Chain: HasHeightType
        + HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasSequenceType<Counterparty>
        + HasOutgoingPacketType<Counterparty>
        + HasAsyncErrorType,
    Counterparty: HasChannelIdType<Chain> + HasPortIdType<Chain>,
{
    async fn query_send_packets_from_sequences(
        _chain: &Chain,
        _channel_id: &Chain::ChannelId,
        _port_id: &Chain::PortId,
        _counterparty_channel_id: &Counterparty::ChannelId,
        _counterparty_port_id: &Counterparty::PortId,
        _sequences: &[Chain::Sequence],
        _height: &Chain::Height,
    ) -> Result<Vec<Chain::OutgoingPacket>, Chain::Error> {
        todo!()
    }
}

#[cgp_provider(SendPacketQuerierComponent)]
impl<Chain, Counterparty> SendPacketQuerier<Chain, Counterparty> for QueryStarknetSendPacket
where
    Chain: HasHeightType
        + HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasSequenceType<Counterparty>
        + HasOutgoingPacketType<Counterparty>
        + HasAsyncErrorType,
    Counterparty: HasChannelIdType<Chain> + HasPortIdType<Chain>,
{
    async fn query_send_packet_from_sequence(
        _chain: &Chain,
        _channel_id: &Chain::ChannelId,
        _port_id: &Chain::PortId,
        _counterparty_channel_id: &Counterparty::ChannelId,
        _counterparty_port_id: &Counterparty::PortId,
        _sequence: &Chain::Sequence,
        _height: &Chain::Height,
    ) -> Result<Chain::OutgoingPacket, Chain::Error> {
        todo!()
    }
}
