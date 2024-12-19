use hermes_chain_components::traits::packet::fields::{
    PacketDstChannelIdGetter, PacketTimeoutHeightGetter,
};
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::HasChannelIdType;
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use ibc::core::channel::types::packet::Packet;
use ibc::core::channel::types::timeout::TimeoutHeight;

use crate::types::channel_id::ChannelId;

pub struct ReadPacketDstStarknetFields;

impl<Chain, Counterparty> PacketDstChannelIdGetter<Chain, Counterparty>
    for ReadPacketDstStarknetFields
where
    Chain: HasOutgoingPacketType<Counterparty, OutgoingPacket = Packet>,
    Counterparty: HasChannelIdType<Chain, ChannelId = ChannelId>,
{
    fn packet_dst_channel_id(packet: &Packet) -> ChannelId {
        ChannelId {
            channel_id: packet.chan_id_on_b.to_string(),
        }
    }
}

impl<Chain, Counterparty> PacketTimeoutHeightGetter<Chain, Counterparty>
    for ReadPacketDstStarknetFields
where
    Chain: HasOutgoingPacketType<Counterparty, OutgoingPacket = Packet>,
    Counterparty: HasHeightType<Height = u64>,
{
    fn packet_timeout_height(packet: &Packet) -> Option<u64> {
        match &packet.timeout_height_on_b {
            TimeoutHeight::Never => None,
            TimeoutHeight::At(h) => Some(h.revision_height()),
        }
    }
}
