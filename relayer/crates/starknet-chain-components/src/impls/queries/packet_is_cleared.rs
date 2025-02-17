use cgp::prelude::*;
use hermes_chain_components::traits::queries::packet_is_cleared::PacketIsClearedQuerier;
use hermes_chain_components::traits::types::ibc::{
    HasChannelIdType, HasPortIdType, HasSequenceType,
};
use hermes_cosmos_chain_components::components::client::PacketIsClearedQuerierComponent;

#[new_cgp_provider(PacketIsClearedQuerierComponent)]
impl<Chain, Counterparty> PacketIsClearedQuerier<Chain, Counterparty>
    for QueryStarknetPacketIsCleared
where
    Chain: HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasSequenceType<Counterparty>
        + HasAsyncErrorType,
{
    async fn query_packet_is_cleared(
        _chain: &Chain,
        _port_id: &Chain::PortId,
        _channel_id: &Chain::ChannelId,
        _sequence: &Chain::Sequence,
    ) -> Result<bool, Chain::Error> {
        Ok(false) // stub
    }
}
