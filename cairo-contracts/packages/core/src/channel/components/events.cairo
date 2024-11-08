#[starknet::component]
pub mod ChannelEventEmitterComponent {
    use starknet_ibc_core::channel::{Packet, ChannelOrdering, AppVersion, Acknowledgement};
    use starknet_ibc_core::client::{Height, Timestamp};
    use starknet_ibc_core::host::{ConnectionId, PortId, ChannelId, Sequence};

    #[storage]
    pub struct Storage {}

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        ChanOpenInitEvent: ChanOpenInitEvent,
        ChanOpenTryEvent: ChanOpenTryEvent,
        SendPacketEvent: SendPacketEvent,
        ReceivePacketEvent: ReceivePacketEvent,
        WriteAcknowledgementEvent: WriteAcknowledgementEvent,
        AcknowledgePacketEvent: AcknowledgePacketEvent,
        TimeoutPacketEvent: TimeoutPacketEvent,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct ChanOpenInitEvent {
        #[key]
        pub port_id_on_a: PortId,
        #[key]
        pub channel_id_on_a: ChannelId,
        #[key]
        pub port_id_on_b: PortId,
        #[key]
        pub connection_id_on_a: ConnectionId,
        #[key]
        pub version_on_a: AppVersion,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct ChanOpenTryEvent {
        #[key]
        pub port_id_on_b: PortId,
        #[key]
        pub channel_id_on_b: ChannelId,
        #[key]
        pub port_id_on_a: PortId,
        #[key]
        pub channel_id_on_a: ChannelId,
        #[key]
        pub connection_id_on_b: ConnectionId,
        #[key]
        pub version_on_b: AppVersion,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct SendPacketEvent {
        #[key]
        pub sequence_on_a: Sequence,
        #[key]
        pub port_id_on_a: PortId,
        #[key]
        pub channel_id_on_a: ChannelId,
        #[key]
        pub port_id_on_b: PortId,
        #[key]
        pub channel_id_on_b: ChannelId,
        #[key]
        pub timeout_height_on_b: Height,
        #[key]
        pub timeout_timestamp_on_b: Timestamp,
        #[key]
        pub channel_ordering: ChannelOrdering,
        pub packet_data: Array<felt252>,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct ReceivePacketEvent {
        #[key]
        pub sequence_on_a: Sequence,
        #[key]
        pub port_id_on_a: PortId,
        #[key]
        pub channel_id_on_a: ChannelId,
        #[key]
        pub port_id_on_b: PortId,
        #[key]
        pub channel_id_on_b: ChannelId,
        #[key]
        pub timeout_height_on_b: Height,
        #[key]
        pub timeout_timestamp_on_b: Timestamp,
        #[key]
        pub channel_ordering: ChannelOrdering,
        pub packet_data: Array<felt252>,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct WriteAcknowledgementEvent {
        #[key]
        pub sequence_on_a: Sequence,
        #[key]
        pub port_id_on_a: PortId,
        #[key]
        pub channel_id_on_a: ChannelId,
        #[key]
        pub port_id_on_b: PortId,
        #[key]
        pub channel_id_on_b: ChannelId,
        pub packet_data: Array<felt252>,
        pub acknowledgement: Acknowledgement,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct AcknowledgePacketEvent {
        #[key]
        pub sequence_on_a: Sequence,
        #[key]
        pub port_id_on_a: PortId,
        #[key]
        pub channel_id_on_a: ChannelId,
        #[key]
        pub port_id_on_b: PortId,
        #[key]
        pub channel_id_on_b: ChannelId,
        #[key]
        pub timeout_height_on_b: Height,
        #[key]
        pub timeout_timestamp_on_b: Timestamp,
        #[key]
        pub channel_ordering: ChannelOrdering,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct TimeoutPacketEvent {
        #[key]
        pub sequence_on_a: Sequence,
        #[key]
        pub port_id_on_a: PortId,
        #[key]
        pub channel_id_on_a: ChannelId,
        #[key]
        pub port_id_on_b: PortId,
        #[key]
        pub channel_id_on_b: ChannelId,
        #[key]
        pub timeout_height_on_b: Height,
        #[key]
        pub timeout_timestamp_on_b: Timestamp,
        #[key]
        pub channel_ordering: ChannelOrdering,
    }

    #[generate_trait]
    pub impl ChannelEventEmitterImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ChannelEventEmitterTrait<TContractState> {
        fn emit_chan_open_init_event(
            ref self: ComponentState<TContractState>,
            port_id_on_a: PortId,
            channel_id_on_a: ChannelId,
            port_id_on_b: PortId,
            connection_id_on_a: ConnectionId,
            version_on_a: AppVersion,
        ) {
            self
                .emit(
                    ChanOpenInitEvent {
                        port_id_on_a,
                        channel_id_on_a,
                        port_id_on_b,
                        connection_id_on_a,
                        version_on_a,
                    }
                );
        }

        fn emit_chan_open_try_event(
            ref self: ComponentState<TContractState>,
            port_id_on_b: PortId,
            channel_id_on_b: ChannelId,
            port_id_on_a: PortId,
            channel_id_on_a: ChannelId,
            connection_id_on_b: ConnectionId,
            version_on_b: AppVersion,
        ) {
            self
                .emit(
                    ChanOpenTryEvent {
                        port_id_on_b,
                        channel_id_on_b,
                        port_id_on_a,
                        channel_id_on_a,
                        connection_id_on_b,
                        version_on_b,
                    }
                );
        }

        fn emit_send_packet_event(
            ref self: ComponentState<TContractState>, packet: Packet, ordering: ChannelOrdering,
        ) {
            self
                .emit(
                    SendPacketEvent {
                        sequence_on_a: packet.seq_on_a,
                        port_id_on_a: packet.port_id_on_a,
                        channel_id_on_a: packet.chan_id_on_a,
                        port_id_on_b: packet.port_id_on_b,
                        channel_id_on_b: packet.chan_id_on_b,
                        timeout_height_on_b: packet.timeout_height_on_b,
                        timeout_timestamp_on_b: packet.timeout_timestamp_on_b,
                        channel_ordering: ordering,
                        packet_data: packet.data,
                    }
                );
        }

        fn emit_recv_packet_event(
            ref self: ComponentState<TContractState>, packet: Packet, ordering: ChannelOrdering,
        ) {
            self
                .emit(
                    ReceivePacketEvent {
                        sequence_on_a: packet.seq_on_a,
                        port_id_on_a: packet.port_id_on_a,
                        channel_id_on_a: packet.chan_id_on_a,
                        port_id_on_b: packet.port_id_on_b,
                        channel_id_on_b: packet.chan_id_on_b,
                        timeout_height_on_b: packet.timeout_height_on_b,
                        timeout_timestamp_on_b: packet.timeout_timestamp_on_b,
                        channel_ordering: ordering,
                        packet_data: packet.data,
                    }
                );
        }

        fn emit_write_ack_event(
            ref self: ComponentState<TContractState>,
            packet: Packet,
            acknowledgement: Acknowledgement,
        ) {
            self
                .emit(
                    WriteAcknowledgementEvent {
                        sequence_on_a: packet.seq_on_a,
                        port_id_on_a: packet.port_id_on_a,
                        channel_id_on_a: packet.chan_id_on_a,
                        port_id_on_b: packet.port_id_on_b,
                        channel_id_on_b: packet.chan_id_on_b,
                        packet_data: packet.data,
                        acknowledgement: acknowledgement,
                    }
                );
        }

        fn emit_ack_packet_event(
            ref self: ComponentState<TContractState>, packet: Packet, ordering: ChannelOrdering,
        ) {
            self
                .emit(
                    AcknowledgePacketEvent {
                        sequence_on_a: packet.seq_on_a,
                        port_id_on_a: packet.port_id_on_a,
                        channel_id_on_a: packet.chan_id_on_a,
                        port_id_on_b: packet.port_id_on_b,
                        channel_id_on_b: packet.chan_id_on_b,
                        timeout_height_on_b: packet.timeout_height_on_b,
                        timeout_timestamp_on_b: packet.timeout_timestamp_on_b,
                        channel_ordering: ordering,
                    }
                );
        }

        fn emit_timeout_packet_event(
            ref self: ComponentState<TContractState>, packet: Packet, ordering: ChannelOrdering,
        ) {
            self
                .emit(
                    TimeoutPacketEvent {
                        sequence_on_a: packet.seq_on_a,
                        port_id_on_a: packet.port_id_on_a,
                        channel_id_on_a: packet.chan_id_on_a,
                        port_id_on_b: packet.port_id_on_b,
                        channel_id_on_b: packet.chan_id_on_b,
                        timeout_height_on_b: packet.timeout_height_on_b,
                        timeout_timestamp_on_b: packet.timeout_timestamp_on_b,
                        channel_ordering: ordering,
                    }
                );
        }
    }
}

