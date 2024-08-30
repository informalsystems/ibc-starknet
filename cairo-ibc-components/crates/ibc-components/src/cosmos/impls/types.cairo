use starknet_ibc_components::cosmos::types::Packet;

use starknet_ibc_components::traits::ProvideIbcTypes;

impl ProvideCosmosIbcTypes of ProvideIbcTypes {
    type LocalAddress = felt252;

    type CounterpartyAddress = ByteArray;

    type LocalClientId = felt252;

    type LocalConnectionId = felt252;

    type LocalChannelId = felt252;

    type LocalPortId = felt252;

    type LocalSequence = u64;

    type CounterpartyClientId = felt252;

    type CounterpartyConnectionId = felt252;

    type CounterpartyChannelId = felt252;

    type CounterpartyPortId = felt252;

    type CounterpartySequence = u64;

    type LocalTime = u128;

    type LocalTimeoutTimestamp = u128;

    type CounterpartyTimeoutTimestamp = u128;

    type IncomingPacket = Packet;

    type OutgoingPacket = ByteArray;

    type IncomingPacketData = ByteArray;

    type OutgoingPacketData = ByteArray;

    fn incoming_packet_src_channel_id(
        packet: @Self::IncomingPacket
    ) -> @Self::CounterpartyChannelId {
        packet.src_channel_id
    }

    fn incoming_packet_dst_channel_id(packet: @Self::IncomingPacket) -> @Self::LocalChannelId {
        packet.dst_channel_id
    }

    fn incoming_packet_src_port_id(packet: @Self::IncomingPacket) -> @Self::CounterpartyPortId {
        packet.src_port_id
    }

    fn incoming_packet_dst_port_id(packet: @Self::IncomingPacket) -> @Self::LocalPortId {
        packet.dst_port_id
    }

    fn outgoing_packet_src_channel_id(packet: @Self::IncomingPacket) -> @Self::LocalChannelId {
        packet.src_channel_id
    }

    fn outgoing_packet_dst_channel_id(
        packet: @Self::IncomingPacket
    ) -> @Self::CounterpartyChannelId {
        packet.dst_channel_id
    }

    fn outgoing_packet_src_port_id(packet: @Self::IncomingPacket) -> @Self::LocalPortId {
        packet.src_port_id
    }

    fn outgoing_packet_dst_port_id(packet: @Self::IncomingPacket) -> @Self::CounterpartyPortId {
        packet.dst_port_id
    }
}
