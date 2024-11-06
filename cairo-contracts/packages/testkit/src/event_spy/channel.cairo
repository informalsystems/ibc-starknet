use openzeppelin_testing::events::{EventSpyExt, EventSpyExtImpl};
use snforge_std::EventSpy;
use starknet::ContractAddress;
use starknet_ibc_core::channel::ChannelEventEmitterComponent::{
    Event, SendPacketEvent, ReceivePacketEvent, AcknowledgePacketEvent, TimeoutPacketEvent
};
use starknet_ibc_core::channel::{Packet, ChannelOrdering};

#[generate_trait]
pub impl ChannelEventSpyExtImpl of ChannelEventSpyExt {
    fn assert_send_packet_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        channel_ordering: ChannelOrdering,
        packet: Packet,
    ) {
        let expected = Event::SendPacketEvent(
            SendPacketEvent {
                sequence_on_a: packet.seq_on_a,
                port_id_on_a: packet.port_id_on_a,
                channel_id_on_a: packet.chan_id_on_a,
                port_id_on_b: packet.port_id_on_b,
                channel_id_on_b: packet.chan_id_on_b,
                timeout_height_on_b: packet.timeout_height_on_b,
                timeout_timestamp_on_b: packet.timeout_timestamp_on_b,
                channel_ordering,
                packet_data: packet.data,
            }
        );
        self.assert_emitted_single(contract_address, expected);
    }

    fn assert_recv_packet_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        channel_ordering: ChannelOrdering,
        packet: Packet,
    ) {
        let expected = Event::ReceivePacketEvent(
            ReceivePacketEvent {
                sequence_on_a: packet.seq_on_a,
                port_id_on_a: packet.port_id_on_a,
                channel_id_on_a: packet.chan_id_on_a,
                port_id_on_b: packet.port_id_on_b,
                channel_id_on_b: packet.chan_id_on_b,
                timeout_height_on_b: packet.timeout_height_on_b,
                timeout_timestamp_on_b: packet.timeout_timestamp_on_b,
                channel_ordering,
                packet_data: packet.data,
            }
        );
        self.assert_emitted_single(contract_address, expected);
    }

    fn assert_ack_packet_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        channel_ordering: ChannelOrdering,
        packet: Packet,
    ) {
        let expected = Event::AcknowledgePacketEvent(
            AcknowledgePacketEvent {
                sequence_on_a: packet.seq_on_a,
                port_id_on_a: packet.port_id_on_a,
                channel_id_on_a: packet.chan_id_on_a,
                port_id_on_b: packet.port_id_on_b,
                channel_id_on_b: packet.chan_id_on_b,
                timeout_height_on_b: packet.timeout_height_on_b,
                timeout_timestamp_on_b: packet.timeout_timestamp_on_b,
                channel_ordering,
            }
        );
        self.assert_emitted_single(contract_address, expected);
    }

    fn assert_timeout_packet_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        channel_ordering: ChannelOrdering,
        packet: Packet,
    ) {
        let expected = Event::TimeoutPacketEvent(
            TimeoutPacketEvent {
                sequence_on_a: packet.seq_on_a,
                port_id_on_a: packet.port_id_on_a,
                channel_id_on_a: packet.chan_id_on_a,
                port_id_on_b: packet.port_id_on_b,
                channel_id_on_b: packet.chan_id_on_b,
                timeout_height_on_b: packet.timeout_height_on_b,
                timeout_timestamp_on_b: packet.timeout_timestamp_on_b,
                channel_ordering,
            }
        );
        self.assert_emitted_single(contract_address, expected);
    }
}
