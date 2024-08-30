use starknet_ibc_components::traits::ProvideIbcTypes;

pub trait ProvideChannelPacketApi {
    type Context;

    impl IbcTypes: ProvideIbcTypes;

    fn handle_receive_packet(
        context: @Self::Context, packet: @Self::IbcTypes::IncomingPacket,
    ) -> Result<Self::IbcTypes::IncomingPacketAck, Self::IbcTypes::Error>;

    fn handle_send_packet(
        context: @Self::Context, packet: @Self::IbcTypes::OutgoingPacketData,
    ) -> Result<Self::IbcTypes::OutgoingPacket, Self::IbcTypes::Error>;

    fn handle_receive_ack_packet(
        context: @Self::Context,
        packet: @Self::IbcTypes::OutgoingPacket,
        packet_ack: @Self::IbcTypes::OutgoingPacketAck,
    ) -> Result<(), Self::IbcTypes::Error>;
}

pub trait ProvideChannelBinding {
    type Context;

    impl IbcTypes: ProvideIbcTypes;

    fn local_channel_id(context: @Self::Context) -> @Self::IbcTypes::LocalChannelId;

    fn local_port_id(context: @Self::Context) -> @Self::IbcTypes::LocalPortId;

    fn counterparty_channel_id(context: @Self::Context) -> @Self::IbcTypes::CounterpartyChannelId;

    fn counterparty_port_id(context: @Self::Context) -> @Self::IbcTypes::CounterpartyPortId;
}

pub trait ProvideChannelStorage {
    type Context;

    impl IbcTypes: ProvideIbcTypes;

    fn commit_receive_packet(
        context: @Self::Context,
        sequence: @Self::IbcTypes::CounterpartySequence,
        packet_hash: @Self::IbcTypes::IncomingPacketHash,
        ack_hash: @Self::IbcTypes::IncomingPacketAckHash,
    ) -> Result<(), Self::IbcTypes::Error>;

    fn commit_send_packet(
        context: @Self::Context,
        sequence: @Self::IbcTypes::LocalSequence,
        packet_hash: @Self::IbcTypes::OutgoingPacketHash,
    ) -> Result<(), Self::IbcTypes::Error>;
}
