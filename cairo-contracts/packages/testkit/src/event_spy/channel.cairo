use openzeppelin_testing::events::{EventSpyExt, EventSpyExtImpl};
use snforge_std::EventSpy;
use starknet::ContractAddress;
use starknet_ibc_core::channel::ChannelEventEmitterComponent::{
    Event, ChanOpenInitEvent, ChanOpenTryEvent, ChanOpenAckEvent, ChanOpenConfirmEvent,
    SendPacketEvent, ReceivePacketEvent, AcknowledgePacketEvent, TimeoutPacketEvent
};
use starknet_ibc_core::channel::{
    MsgChanOpenInit, MsgChanOpenTry, MsgChanOpenAck, MsgChanOpenConfirm, Packet, ChannelOrdering,
    AppVersion
};
use starknet_ibc_core::host::{ChannelId, ConnectionId, PortId};

#[generate_trait]
pub impl ChannelEventSpyExtImpl of ChannelEventSpyExt {
    fn assert_chan_open_init_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        channel_id_on_a: ChannelId,
        version_on_a: AppVersion,
        msg: MsgChanOpenInit
    ) {
        let expected = Event::ChanOpenInitEvent(
            ChanOpenInitEvent {
                port_id_on_a: msg.port_id_on_a,
                channel_id_on_a,
                port_id_on_b: msg.port_id_on_b,
                connection_id_on_a: msg.conn_id_on_a,
                version_on_a,
            }
        );
        self.assert_emitted_single(contract_address, expected);
    }

    fn assert_chan_open_try_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        channel_id_on_b: ChannelId,
        version_on_b: AppVersion,
        msg: MsgChanOpenTry
    ) {
        let expected = Event::ChanOpenTryEvent(
            ChanOpenTryEvent {
                port_id_on_b: msg.port_id_on_b,
                channel_id_on_b,
                port_id_on_a: msg.port_id_on_a,
                channel_id_on_a: msg.chan_id_on_a,
                connection_id_on_b: msg.conn_id_on_b,
                version_on_b,
            }
        );
        self.assert_emitted_single(contract_address, expected);
    }

    fn assert_chan_open_ack_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        port_id_on_b: PortId,
        connection_id_on_a: ConnectionId,
        msg: MsgChanOpenAck
    ) {
        let expected = Event::ChanOpenAckEvent(
            ChanOpenAckEvent {
                port_id_on_a: msg.port_id_on_a,
                channel_id_on_a: msg.chan_id_on_a,
                port_id_on_b,
                channel_id_on_b: msg.chan_id_on_b,
                connection_id_on_a,
            }
        );
        self.assert_emitted_single(contract_address, expected);
    }

    fn assert_chan_open_confirm_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        port_id_on_a: PortId,
        channel_id_on_a: ChannelId,
        connection_id_on_b: ConnectionId,
        msg: MsgChanOpenConfirm
    ) {
        let expected = Event::ChanOpenConfirmEvent(
            ChanOpenConfirmEvent {
                port_id_on_b: msg.port_id_on_b,
                channel_id_on_b: msg.chan_id_on_b,
                port_id_on_a,
                channel_id_on_a,
                connection_id_on_b,
            }
        );
        self.assert_emitted_single(contract_address, expected);
    }

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
