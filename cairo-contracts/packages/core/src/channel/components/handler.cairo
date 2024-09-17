#[starknet::component]
pub mod ChannelHandlerComponent {
    use ChannelEventEmitterComponent::ChannelEventEmitterTrait;
    use ClientHandlerComponent::ClientInternalTrait;
    use RouterHandlerComponent::RouterInternalTrait;
    use core::num::traits::Zero;
    use starknet::ContractAddress;
    use starknet::storage::Map;
    use starknet::{get_block_timestamp, get_block_number};
    use starknet_ibc_core::channel::{
        ChannelEventEmitterComponent, IChannelHandler, MsgRecvPacket, ChannelEnd, ChannelEndTrait,
        ChannelErrors, PacketTrait, ChannelOrdering, Receipt, AcknowledgementTrait, Packet,
        Acknowledgement
    };
    use starknet_ibc_core::client::{
        ClientHandlerComponent, ClientContract, ClientContractTrait, StatusTrait
    };
    use starknet_ibc_core::host::{
        PortId, PortIdTrait, ChannelId, ChannelIdTrait, Sequence, SequenceImpl, SequencePartialOrd,
        channel_end_key, receipt_key, ack_key, commitment_path, next_sequence_recv_key
    };
    use starknet_ibc_core::router::{
        RouterHandlerComponent, IRouter, ApplicationContractTrait, ApplicationContract
    };
    use starknet_ibc_core::tests::{PORT_ID, CHANNEL_ID, CHANNEL_END};
    use starknet_ibc_utils::{ValidateBasicTrait, ComputeKeyTrait};

    #[storage]
    struct Storage {
        pub channel_ends: Map<felt252, Option<ChannelEnd>>,
        pub packet_receipts: Map<felt252, Option<Receipt>>,
        pub packet_acks: Map<felt252, felt252>,
        pub recv_sequences: Map<felt252, Sequence>,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {}

    #[generate_trait]
    pub impl ChannelInitializerImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ChannelInitializerTrait<TContractState> {
        fn initializer(ref self: ComponentState<TContractState>) {
            // TODO: Initialize a temporary dummy `ChannelEnd` for testing the
            // handlers. This should be removed once the channel handshake is
            // implemented.
            self.write_channel_end(@PORT_ID(), @CHANNEL_ID(0), CHANNEL_END());
            self.write_next_sequence_recv(@PORT_ID(), @CHANNEL_ID(0), SequenceImpl::zero());
        }
    }

    #[embeddable_as(CoreChannelHandler)]
    impl CoreChannelHandlerImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>
    > of IChannelHandler<ComponentState<TContractState>> {
        fn recv_packet(ref self: ComponentState<TContractState>, msg: MsgRecvPacket) {
            let chan_end_on_b = self
                .get_channel_end(@msg.packet.port_id_on_b, @msg.packet.chan_id_on_b,);

            self.recv_packet_validate(msg.clone(), chan_end_on_b.clone());

            self.recv_packet_execute(msg, chan_end_on_b);
        }
    }

    #[generate_trait]
    pub(crate) impl RecvPacketImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>
    > of RecvPacketTrait<TContractState> {
        fn recv_packet_validate(
            self: @ComponentState<TContractState>, msg: MsgRecvPacket, chan_end_on_b: ChannelEnd
        ) {
            msg.validate_basic();

            // TODO: verify connection end if we ever decide to implement ICS-03

            chan_end_on_b.validate(@msg.packet.port_id_on_a, @msg.packet.chan_id_on_a);

            self.verify_not_timed_out(@msg.packet);

            let client = self.get_client(chan_end_on_b.client_id.client_type);

            client.verify_is_active(chan_end_on_b.client_id.sequence);

            client.verify_proof_height(@msg.proof_height_on_a, chan_end_on_b.client_id.sequence);

            self.verify_packet_commitment(@client, chan_end_on_b.clone(), msg.clone());

            match @chan_end_on_b.ordering {
                ChannelOrdering::Unordered => {
                    let reciept_resp = self
                        .read_packet_receipt(
                            @msg.packet.port_id_on_b, @msg.packet.chan_id_on_b, @msg.packet.seq_on_a
                        );

                    assert(reciept_resp.is_some(), ChannelErrors::PACKET_ALREADY_RECEIVED);

                    self
                        .verify_ack_not_exists(
                            @msg.packet.port_id_on_b, @msg.packet.chan_id_on_b, @msg.packet.seq_on_a
                        );
                },
                ChannelOrdering::Ordered => {
                    let next_sequence_recv = self
                        .read_next_sequence_recv(
                            @msg.packet.port_id_on_b, @msg.packet.chan_id_on_b
                        );

                    assert(
                        @next_sequence_recv >= @msg.packet.seq_on_a,
                        ChannelErrors::INVALID_PACKET_SEQUENCE
                    );

                    // If the packet sequence matches the expected next
                    // sequence, we check if the ack not exists. As the
                    // existance means the packet was already relayed.
                    if next_sequence_recv == msg.packet.seq_on_a {
                        self
                            .verify_ack_not_exists(
                                @msg.packet.port_id_on_b,
                                @msg.packet.chan_id_on_b,
                                @msg.packet.seq_on_a
                            );
                    }
                }
            };
        }

        fn recv_packet_execute(
            ref self: ComponentState<TContractState>, msg: MsgRecvPacket, chan_end_on_b: ChannelEnd
        ) {
            let app = self.get_app(@msg.packet.port_id_on_b);

            let ack = app.on_recv_packet(msg.packet.clone());

            match @chan_end_on_b.ordering {
                ChannelOrdering::Unordered => {
                    self
                        .write_packet_receipt(
                            @msg.packet.port_id_on_b,
                            @msg.packet.chan_id_on_b,
                            @msg.packet.seq_on_a,
                            Receipt::Ok
                        );
                },
                ChannelOrdering::Ordered => {
                    self
                        .write_next_sequence_recv(
                            @msg.packet.port_id_on_b,
                            @msg.packet.chan_id_on_b,
                            msg.packet.seq_on_a.clone()
                        );
                }
            };

            self
                .write_packet_ack(
                    @msg.packet.port_id_on_b,
                    @msg.packet.chan_id_on_b,
                    @msg.packet.seq_on_a,
                    ack.compute_commitment()
                );

            self.emit_recv_packet_event(msg.packet.clone(), chan_end_on_b.ordering);

            self.emit_write_ack_event(msg.packet, ack);
        }

        fn verify_packet_commitment(
            self: @ComponentState<TContractState>,
            client: @ClientContract,
            chan_end_on_b: ChannelEnd,
            msg: MsgRecvPacket
        ) {
            let packet_commitment_on_a = msg.packet.compute_commitment();

            let mut path: ByteArray =
                "Ibc/"; // Setting prefix manually for now. This should come from the connection layer once implemented.

            let commitment_path = commitment_path(
                msg.packet.port_id_on_a.clone(),
                msg.packet.chan_id_on_a.clone(),
                msg.packet.seq_on_a.clone()
            );

            path.append(@commitment_path);

            client
                .verify_membership(
                    chan_end_on_b.client_id.sequence,
                    path,
                    packet_commitment_on_a,
                    msg.proof_commitment_on_a.clone()
                );
        }

        fn verify_not_timed_out(self: @ComponentState<TContractState>, packet: @Packet) {
            let host_height = get_block_number();

            let host_timestamp = get_block_timestamp();

            packet.check_timed_out(@host_height, @host_timestamp);
        }

        fn verify_ack_not_exists(
            self: @ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence
        ) {
            let ack = self.read_packet_ack(port_id, channel_id, sequence);

            assert(ack.is_zero(), ChannelErrors::ACK_ALREADY_EXISTS);
        }
    }

    #[generate_trait]
    pub(crate) impl ChannelInternalImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>
    > of ChannelInternalTrait<TContractState> {
        fn get_channel_end(
            self: @ComponentState<TContractState>,
            local_port_id: @PortId,
            local_channel_id: @ChannelId,
        ) -> ChannelEnd {
            let maybe_chan_end = self.read_channel_end(local_port_id, local_channel_id);

            assert(maybe_chan_end.is_some(), ChannelErrors::MISSING_CHANNEL_END);

            maybe_chan_end.unwrap()
        }

        fn get_client(
            self: @ComponentState<TContractState>, client_type: felt252
        ) -> ClientContract {
            let client_comp = get_dep_component!(self, ClientHandler);

            client_comp.get_client(client_type)
        }

        fn get_app(self: @ComponentState<TContractState>, port_id: @PortId) -> ApplicationContract {
            let router_comp = get_dep_component!(self, RouterHandler);

            router_comp.get_app(port_id.key())
        }
    }

    #[generate_trait]
    pub(crate) impl ChannelReaderImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ChannelReaderTrait<TContractState> {
        fn read_channel_end(
            self: @ComponentState<TContractState>, port_id: @PortId, channel_id: @ChannelId
        ) -> Option<ChannelEnd> {
            self.channel_ends.read(channel_end_key(port_id, channel_id))
        }

        fn read_packet_receipt(
            self: @ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence
        ) -> Option<Receipt> {
            self.packet_receipts.read(receipt_key(port_id, channel_id, sequence))
        }

        fn read_packet_ack(
            self: @ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence
        ) -> felt252 {
            self.packet_acks.read(ack_key(port_id, channel_id, sequence))
        }

        fn read_next_sequence_recv(
            self: @ComponentState<TContractState>, port_id: @PortId, channel_id: @ChannelId
        ) -> Sequence {
            self.recv_sequences.read(next_sequence_recv_key(port_id, channel_id))
        }
    }

    #[generate_trait]
    pub(crate) impl ChannelWriterImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ChannelWriterTrait<TContractState> {
        fn write_channel_end(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            channel_end: ChannelEnd
        ) {
            self
                .channel_ends
                .write(channel_end_key(port_id, channel_id), Option::Some(channel_end));
        }

        fn write_packet_receipt(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence,
            receipt: Receipt
        ) {
            self
                .packet_receipts
                .write(receipt_key(port_id, channel_id, sequence), Option::Some(receipt));
        }

        fn write_packet_ack(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence,
            ack: felt252
        ) {
            self.packet_acks.write(ack_key(port_id, channel_id, sequence), ack);
        }

        fn write_next_sequence_recv(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: Sequence
        ) {
            self.recv_sequences.write(next_sequence_recv_key(port_id, channel_id), sequence);
        }
    }

    #[generate_trait]
    pub(crate) impl EventEmitterImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>
    > of EventEmitterTrait<TContractState> {
        fn emit_send_packet_event(
            ref self: ComponentState<TContractState>, packet: Packet, ordering: ChannelOrdering
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter.emit_send_packet_event(packet, ordering);
        }

        fn emit_recv_packet_event(
            ref self: ComponentState<TContractState>, packet: Packet, ordering: ChannelOrdering
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter.emit_recv_packet_event(packet, ordering);
        }

        fn emit_write_ack_event(
            ref self: ComponentState<TContractState>, packet: Packet, ack: Acknowledgement
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter.emit_write_ack_event(packet, ack);
        }

        fn emit_ack_packet_event(
            ref self: ComponentState<TContractState>, packet: Packet, ordering: ChannelOrdering
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter.emit_ack_packet_event(packet, ordering);
        }

        fn emit_timeout_packet_event(
            ref self: ComponentState<TContractState>, packet: Packet, ordering: ChannelOrdering
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter.emit_timeout_packet_event(packet, ordering);
        }
    }
}
