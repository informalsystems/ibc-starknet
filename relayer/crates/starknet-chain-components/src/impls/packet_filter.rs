use cgp::prelude::HasErrorType;
use hermes_chain_components::traits::packet::filter::{IncomingPacketFilter, OutgoingPacketFilter};
use hermes_chain_components::traits::types::packet::{
    HasIncomingPacketType, HasOutgoingPacketType,
};

pub struct FilterStarknetPackets;

impl<Chain, Counterparty> OutgoingPacketFilter<Chain, Counterparty> for FilterStarknetPackets
where
    Chain: HasOutgoingPacketType<Counterparty> + HasErrorType,
{
    async fn should_relay_outgoing_packet(
        _chain: &Chain,
        _packet: &Chain::OutgoingPacket,
    ) -> Result<bool, Chain::Error> {
        Ok(true)
    }
}

impl<Chain, Counterparty> IncomingPacketFilter<Chain, Counterparty> for FilterStarknetPackets
where
    Chain: HasIncomingPacketType<Counterparty> + HasErrorType,
{
    async fn should_relay_incoming_packet(
        _chain: &Chain,
        _packet: &Chain::IncomingPacket,
    ) -> Result<bool, Chain::Error> {
        Ok(true)
    }
}
