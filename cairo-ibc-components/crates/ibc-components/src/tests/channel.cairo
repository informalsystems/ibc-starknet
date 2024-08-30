use crate::cosmos::impls::ProvideCosmosIbcTypes;
use crate::cosmos::types::Packet;
use crate::impls::channel::CheckPacketFields;

use crate::traits::{ProvideChannelBinding, ProvideChannelPacketApi};

#[derive(Drop)]
pub struct Context {}

pub impl DummyChannelPacketHandler of ProvideChannelPacketApi<Context, ProvideCosmosIbcTypes> {
    fn handle_receive_packet(
        context: @Context, packet: @ProvideCosmosIbcTypes::IncomingPacket,
    ) -> Result<ProvideCosmosIbcTypes::IncomingPacketAck, ProvideCosmosIbcTypes::Error> {
        panic!("todo")
    }

    fn handle_send_packet(
        context: @Context, packet: @ProvideCosmosIbcTypes::OutgoingPacketData,
    ) -> Result<ProvideCosmosIbcTypes::OutgoingPacket, ProvideCosmosIbcTypes::Error> {
        panic!("todo")
    }

    fn handle_receive_ack_packet(
        context: @Context,
        packet: @ProvideCosmosIbcTypes::OutgoingPacket,
        packet_ack: @ProvideCosmosIbcTypes::OutgoingPacketAck,
    ) -> Result<(), ProvideCosmosIbcTypes::Error> {
        panic!("todo")
    }
}

pub impl DummyChannelBinding of ProvideChannelBinding<Context, ProvideCosmosIbcTypes> {
    fn local_channel_id(context: @Context) -> @ProvideCosmosIbcTypes::LocalChannelId {
        @'channel-1'
    }

    fn local_port_id(context: @Context) -> @ProvideCosmosIbcTypes::LocalPortId {
        @'channel-2'
    }

    fn counterparty_channel_id(context: @Context) -> @ProvideCosmosIbcTypes::CounterpartyChannelId {
        @'transfer'
    }

    fn counterparty_port_id(context: @Context) -> @ProvideCosmosIbcTypes::CounterpartyPortId {
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
