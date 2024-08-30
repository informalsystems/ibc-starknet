use starknet_ibc_components::traits::ProvideIbcTypes;

pub trait ProvideChannelPacketApi<Context, impl IbcTypes: ProvideIbcTypes> {
    fn handle_receive_packet(
        context: @Context, packet: @IbcTypes::IncomingPacket,
    ) -> Result<IbcTypes::IncomingPacketAck, IbcTypes::Error>;

    fn handle_send_packet(
        context: @Context, packet: @IbcTypes::OutgoingPacketData,
    ) -> Result<IbcTypes::OutgoingPacket, IbcTypes::Error>;

    fn handle_receive_ack_packet(
        context: @Context,
        packet: @IbcTypes::OutgoingPacket,
        packet_ack: @IbcTypes::OutgoingPacketAck,
    ) -> Result<(), IbcTypes::Error>;
}

pub trait ProvideChannelBinding<Context, impl IbcTypes: ProvideIbcTypes> {
    fn local_channel_id(context: @Context) -> @IbcTypes::LocalChannelId;

    fn local_port_id(context: @Context) -> @IbcTypes::LocalPortId;

    fn counterparty_channel_id(context: @Context) -> @IbcTypes::CounterpartyChannelId;

    fn counterparty_port_id(context: @Context) -> @IbcTypes::CounterpartyPortId;
}

pub trait ProvideChannelStorage<Context, impl IbcTypes: ProvideIbcTypes> {
    fn commit_receive_packet(
        context: @Context,
        sequence: @IbcTypes::CounterpartySequence,
        packet_hash: @IbcTypes::IncomingPacketHash,
        ack_hash: @IbcTypes::IncomingPacketAckHash,
    ) -> Result<(), IbcTypes::Error>;

    fn commit_send_packet(
        context: @Context,
        sequence: @IbcTypes::LocalSequence,
        packet_hash: @IbcTypes::OutgoingPacketHash,
    ) -> Result<(), IbcTypes::Error>;
}
