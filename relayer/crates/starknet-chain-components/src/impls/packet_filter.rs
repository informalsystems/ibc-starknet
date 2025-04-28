use cgp::prelude::*;
use hermes_core::chain_components::traits::{
    HasIncomingPacketType, HasOutgoingPacketType, IncomingPacketFilter,
    IncomingPacketFilterComponent, OutgoingPacketFilter, OutgoingPacketFilterComponent,
};

pub struct FilterStarknetPackets;

#[cgp_provider(OutgoingPacketFilterComponent)]
impl<Chain, Counterparty> OutgoingPacketFilter<Chain, Counterparty> for FilterStarknetPackets
where
    Chain: HasOutgoingPacketType<Counterparty> + HasAsyncErrorType,
{
    async fn should_relay_outgoing_packet(
        _chain: &Chain,
        _packet: &Chain::OutgoingPacket,
    ) -> Result<bool, Chain::Error> {
        Ok(true)
    }
}

#[cgp_provider(IncomingPacketFilterComponent)]
impl<Chain, Counterparty> IncomingPacketFilter<Chain, Counterparty> for FilterStarknetPackets
where
    Chain: HasIncomingPacketType<Counterparty> + HasAsyncErrorType,
{
    async fn should_relay_incoming_packet(
        _chain: &Chain,
        _packet: &Chain::IncomingPacket,
    ) -> Result<bool, Chain::Error> {
        Ok(true)
    }
}
