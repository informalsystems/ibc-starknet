use crate::traits::{ProvideChannelBinding, ProvideChannelPacketApi, ProvideIbcTypes};

pub impl CheckPacketFields<
    Context,
    impl IbcTypes: ProvideIbcTypes,
    impl Provider: ProvideChannelBinding<Context, IbcTypes>,
    impl Inner: ProvideChannelPacketApi<Context, IbcTypes>,
    impl EqChannelId: PartialEq<IbcTypes::LocalChannelId>,
> of ProvideChannelPacketApi<Context, IbcTypes> {
    fn handle_receive_packet(
        context: @Context, packet: @IbcTypes::IncomingPacket,
    ) -> Result<IbcTypes::IncomingPacketAck, IbcTypes::Error> {
        let channel_id = Provider::local_channel_id(context);

        let packet_channel_id = IbcTypes::incoming_packet_dst_channel_id(packet);

        if channel_id != packet_channel_id {
            panic!("mismatch channel id");
        }

        Inner::handle_receive_packet(context, packet)
    }

    fn handle_send_packet(
        context: @Context, packet: @IbcTypes::OutgoingPacketData,
    ) -> Result<IbcTypes::OutgoingPacket, IbcTypes::Error> {
        panic!("todo")
    }

    fn handle_receive_ack_packet(
        context: @Context,
        packet: @IbcTypes::OutgoingPacket,
        packet_ack: @IbcTypes::OutgoingPacketAck,
    ) -> Result<(), IbcTypes::Error> {
        panic!("todo")
    }
}
