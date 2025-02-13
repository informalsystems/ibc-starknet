use cgp::prelude::*;
use hermes_chain_components::traits::queries::ack_packets::AckPacketsQuerier;
use hermes_chain_components::traits::queries::packet_acknowledgements::PacketAcknowledgementsQuerier;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{
    HasChannelIdType, HasPortIdType, HasSequenceType,
};
use hermes_chain_components::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_cosmos_chain_components::components::client::{
    AckPacketsQuerierComponent, PacketAcknowledgementsQuerierComponent,
};

pub struct QueryStarknetAckPackets;

#[cgp_provider(AckPacketsQuerierComponent)]
impl<Chain, Counterparty> AckPacketsQuerier<Chain, Counterparty> for QueryStarknetAckPackets
where
    Chain: HasHeightType
        + HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasWriteAckEvent<Counterparty>
        + HasAsyncErrorType,
    Counterparty: HasOutgoingPacketType<Chain>
        + HasChannelIdType<Chain>
        + HasPortIdType<Chain>
        + HasSequenceType<Chain>,
{
    async fn query_ack_packets_from_sequences(
        _chain: &Chain,
        _channel_id: &Counterparty::ChannelId,
        _port_id: &Counterparty::PortId,
        _counterparty_channel_id: &Chain::ChannelId,
        _counterparty_port_id: &Chain::PortId,
        _sequences: &[Counterparty::Sequence],
    ) -> Result<Vec<(Counterparty::OutgoingPacket, Chain::WriteAckEvent)>, Chain::Error> {
        todo!()
    }
}

#[cgp_provider(PacketAcknowledgementsQuerierComponent)]
impl<Chain, Counterparty> PacketAcknowledgementsQuerier<Chain, Counterparty>
    for QueryStarknetAckPackets
where
    Chain: HasHeightType
        + HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasAsyncErrorType,
    Counterparty: HasSequenceType<Chain>,
{
    async fn query_packet_acknowlegements(
        _chain: &Chain,
        _channel_id: &Chain::ChannelId,
        _port_id: &Chain::PortId,
        _sequences: &[Counterparty::Sequence],
    ) -> Result<Option<Vec<Counterparty::Sequence>>, Chain::Error> {
        todo!()
    }
}
