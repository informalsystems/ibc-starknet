use core::metaprogramming::TypeEqual;

use crate::traits::{ProvideChannelBinding, ProvideChannelPacketApi, ProvideIbcTypes};

pub impl CheckPacketFields<
    Context,
    impl IbcTypes: ProvideIbcTypes,
    impl Provider: ProvideChannelBinding,
    impl Inner: ProvideChannelPacketApi,
    +TypeEqual<Inner::Context, Context>,
    +TypeEqual<Provider::Context, Context>,
    +TypeEqual<Provider::IbcTypes::LocalChannelId, IbcTypes::LocalChannelId>,
    +TypeEqual<Inner::IbcTypes::Error, IbcTypes::Error>,
    +TypeEqual<Inner::IbcTypes::IncomingPacket, IbcTypes::IncomingPacket>,
    +TypeEqual<Inner::IbcTypes::IncomingPacketAck, IbcTypes::IncomingPacketAck>,
> of ProvideChannelPacketApi {
    type Context = Context;

    impl IbcTypes = IbcTypes;

    fn handle_receive_packet(
        context: @Self::Context, packet: @Self::IbcTypes::IncomingPacket,
    ) -> Result<Self::IbcTypes::IncomingPacketAck, Self::IbcTypes::Error> {
        let channel_id = Provider::local_channel_id(context);

        let packet_channel_id = Self::IbcTypes::incoming_packet_dst_channel_id(packet);

        if channel_id != packet_channel_id {
            panic!("mismatch channel id");
        }

        Inner::handle_receive_packet(context, packet)
    }

    fn handle_send_packet(
        context: @Self::Context, packet: @Self::IbcTypes::OutgoingPacketData,
    ) -> Result<Self::IbcTypes::OutgoingPacket, Self::IbcTypes::Error> {
        panic!("todo")
    }

    fn handle_receive_ack_packet(
        context: @Self::Context,
        packet: @Self::IbcTypes::OutgoingPacket,
        packet_ack: @Self::IbcTypes::OutgoingPacketAck,
    ) -> Result<(), Self::IbcTypes::Error> {
        panic!("todo")
    }
}
