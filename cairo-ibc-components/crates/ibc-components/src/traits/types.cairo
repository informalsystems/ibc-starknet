pub trait ProvideIbcTypes {
    type Error;

    type LocalAddress;

    type CounterpartyAddress;

    type LocalClientId;

    type LocalConnectionId;

    type LocalChannelId;

    type LocalPortId;

    type LocalSequence;

    type CounterpartyClientId;

    type CounterpartyConnectionId;

    type CounterpartyChannelId;

    type CounterpartyPortId;

    type CounterpartySequence;

    type LocalTime;

    type LocalTimeoutTimestamp;

    type CounterpartyTimeoutTimestamp;

    type IncomingPacket;

    type OutgoingPacket;

    type IncomingPacketData;

    type OutgoingPacketData;

    type IncomingPacketAck;

    type OutgoingPacketAck;

    type IncomingPacketHash;

    type OutgoingPacketHash;

    type IncomingPacketAckHash;

    type OutgoingPacketAckHash;

    impl LocalChannelIdEq: PartialEq<Self::LocalChannelId>;

    fn hash_incoming_packet(packet: @Self::IncomingPacket) -> Self::IncomingPacketHash;

    fn hash_outgoing_packet(packet: @Self::IncomingPacket) -> Self::OutgoingPacketHash;

    fn incoming_packet_src_channel_id(
        packet: @Self::IncomingPacket
    ) -> @Self::CounterpartyChannelId;

    fn incoming_packet_dst_channel_id(packet: @Self::IncomingPacket) -> @Self::LocalChannelId;

    fn incoming_packet_src_port_id(packet: @Self::IncomingPacket) -> @Self::CounterpartyPortId;

    fn incoming_packet_dst_port_id(packet: @Self::IncomingPacket) -> @Self::LocalPortId;

    fn outgoing_packet_src_channel_id(packet: @Self::IncomingPacket) -> @Self::LocalChannelId;

    fn outgoing_packet_dst_channel_id(
        packet: @Self::IncomingPacket
    ) -> @Self::CounterpartyChannelId;

    fn outgoing_packet_src_port_id(packet: @Self::IncomingPacket) -> @Self::LocalPortId;

    fn outgoing_packet_dst_port_id(packet: @Self::IncomingPacket) -> @Self::CounterpartyPortId;
}
