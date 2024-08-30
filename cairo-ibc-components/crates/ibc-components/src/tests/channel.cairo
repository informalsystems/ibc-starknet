use crate::cosmos::impls::ProvideCosmosIbcTypes;
use crate::cosmos::types::Packet;
use crate::impls::channel::CheckPacketFields;

use crate::traits::{ProvideChannelBinding, ProvideChannelPacketApi};

#[derive(Drop)]
pub struct Context {}

pub impl DummyChannelPacketHandler of ProvideChannelPacketApi {
    type Context = Context;

    impl IbcTypes = ProvideCosmosIbcTypes;

    fn handle_receive_packet(
        context: @Self::Context, packet: @Self::IbcTypes::IncomingPacket,
    ) -> Result<Self::IbcTypes::IncomingPacketAck, Self::IbcTypes::Error> {
        panic!("todo")
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

pub impl DummyChannelBinding of ProvideChannelBinding {
    type Context = Context;

    impl IbcTypes = ProvideCosmosIbcTypes;

    fn local_channel_id(context: @Self::Context) -> @Self::IbcTypes::LocalChannelId {
        @'channel-1'
    }

    fn local_port_id(context: @Self::Context) -> @Self::IbcTypes::LocalPortId {
        @'channel-2'
    }

    fn counterparty_channel_id(context: @Self::Context) -> @Self::IbcTypes::CounterpartyChannelId {
        @'transfer'
    }

    fn counterparty_port_id(context: @Self::Context) -> @Self::IbcTypes::CounterpartyPortId {
        @'transfer'
    }
}

pub impl TestPacketHandler =
    CheckPacketFields<
        Context, ProvideCosmosIbcTypes, DummyChannelBinding, DummyChannelPacketHandler
    >;

#[test]
fn test_packet_handler() {
    let context = Context {};

    let packet = Packet {
        src_channel_id: 'channel-1',
        src_port_id: 'transfer',
        dst_channel_id: 'channel-1',
        dst_port_id: 'transfer',
        sequence: 1,
        packet_data: "",
    };

    TestPacketHandler::handle_receive_packet(@context, @packet);
}
