use cgp::prelude::*;
use hermes_chain_components::traits::packet::fields::{
    PacketSrcChannelIdGetter, PacketSrcChannelIdGetterComponent,
};
use hermes_chain_components::traits::types::ibc::HasChannelIdType;
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use ibc::core::channel::types::packet::Packet;

use crate::types::channel_id::ChannelId;

pub struct ReadPacketSrcStarknetFields;

#[cgp_provider(PacketSrcChannelIdGetterComponent)]
impl<Chain, Counterparty> PacketSrcChannelIdGetter<Chain, Counterparty>
    for ReadPacketSrcStarknetFields
where
    Chain: HasOutgoingPacketType<Counterparty, OutgoingPacket = Packet>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>,
{
    fn packet_src_channel_id(packet: &Packet) -> ChannelId {
        packet.chan_id_on_a.clone()
    }
}
