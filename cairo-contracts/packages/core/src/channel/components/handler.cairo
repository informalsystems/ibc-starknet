use starknet_ibc_core::channel::{ChannelEnd, ChannelState, ChannelOrdering, Counterparty};
use starknet_ibc_core::host::{ClientId, PortId, ChannelId, SequencePartialOrd, SequenceZero};

#[starknet::component]
pub mod ChannelHandlerComponent {
    use ChannelEventEmitterComponent::ChannelEventEmitterTrait;
    use ClientHandlerComponent::ClientInternalTrait;
    use RouterHandlerComponent::RouterInternalTrait;
    use core::num::traits::Zero;
    use starknet::storage::Map;
    use starknet::storage::{StorageMapReadAccess, StorageMapWriteAccess};
    use starknet::{get_block_timestamp, get_block_number};
    use starknet_ibc_core::channel::{
        ChannelEventEmitterComponent, IChannelHandler, IChannelQuery, MsgRecvPacket, MsgAckPacket,
        ChannelEnd, ChannelEndTrait, ChannelErrors, PacketTrait, ChannelOrdering, Receipt,
        ReceiptTrait, Packet, Acknowledgement
    };
    use starknet_ibc_core::client::{
        ClientHandlerComponent, ClientContract, ClientContractTrait, HeightImpl
    };
    use starknet_ibc_core::commitment::{
        CommitmentValue, CommitmentValueZero, compute_packet_commtiment, compute_ack_commitment
    };
    use starknet_ibc_core::host::{
        PortId, ChannelId, Sequence, SequenceImpl, SequencePartialOrd, SequenceZero,
        channel_end_key, commitment_key, receipt_key, ack_key, commitment_path, ack_path,
        next_sequence_recv_key, next_sequence_send_key, next_sequence_ack_key
    };
    use starknet_ibc_core::router::{RouterHandlerComponent, AppContractTrait, AppContract};
    use starknet_ibc_utils::ValidateBasic;
    use super::{PORT_ID, CHANNEL_ID, CHANNEL_END};

    #[storage]
    pub struct Storage {
        pub channel_ends: Map<felt252, ChannelEnd>,
        pub packet_commitments: Map<felt252, CommitmentValue>,
        pub packet_receipts: Map<felt252, Receipt>,
        pub packet_acks: Map<felt252, CommitmentValue>,
        pub send_sequences: Map<felt252, Sequence>,
        pub recv_sequences: Map<felt252, Sequence>,
        pub ack_sequences: Map<felt252, Sequence>,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {}

    // -----------------------------------------------------------
    // Channel Initializer
    // -----------------------------------------------------------

    #[generate_trait]
    pub impl ChannelInitializerImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ChannelInitializerTrait<TContractState> {
        fn initializer(ref self: ComponentState<TContractState>) {
            // TODO: Initialize a temporary dummy `ChannelEnd`s for testing the
            // handlers. This should be removed once the channel handshake is
            // implemented.
            self.write_channel_end(@PORT_ID(), @CHANNEL_ID(0), CHANNEL_END(1));
            self.write_next_sequence_recv(@PORT_ID(), @CHANNEL_ID(0), SequenceZero::zero());

            self.write_channel_end(@PORT_ID(), @CHANNEL_ID(1), CHANNEL_END(0));
            self.write_next_sequence_send(@PORT_ID(), @CHANNEL_ID(1), SequenceZero::zero());
        }
    }

    // -----------------------------------------------------------
    // IChannelHandler
    // -----------------------------------------------------------

    #[embeddable_as(CoreChannelHandler)]
    impl CoreChannelHandlerImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>
    > of IChannelHandler<ComponentState<TContractState>> {
        fn send_packet(ref self: ComponentState<TContractState>, packet: Packet) {
            let chan_end_on_a = self.read_channel_end(@packet.port_id_on_a, @packet.chan_id_on_a);

            self.send_packet_validate(packet.clone(), chan_end_on_a.clone());

            self.send_packet_execute(packet, chan_end_on_a);
        }

        fn recv_packet(ref self: ComponentState<TContractState>, msg: MsgRecvPacket) {
            let chan_end_on_b = self
                .read_channel_end(@msg.packet.port_id_on_b, @msg.packet.chan_id_on_b);

            self.recv_packet_validate(msg.clone(), chan_end_on_b.clone());

            self.recv_packet_execute(msg, chan_end_on_b);
        }

        fn ack_packet(ref self: ComponentState<TContractState>, msg: MsgAckPacket) {
            let chan_end_on_a = self
                .read_channel_end(@msg.packet.port_id_on_a, @msg.packet.chan_id_on_b);

            self.ack_packet_validate(msg.clone(), chan_end_on_a.clone());

            self.ack_packet_execute(msg, chan_end_on_a);
        }
    }

    // -----------------------------------------------------------
    // IChannelQuery
    // -----------------------------------------------------------

    #[embeddable_as(CoreChannelQuery)]
    impl CoreChannelQueryImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of IChannelQuery<ComponentState<TContractState>> {
        fn channel_end(
            self: @ComponentState<TContractState>, port_id: PortId, channel_id: ChannelId
        ) -> ChannelEnd {
            self.read_channel_end(@port_id, @channel_id)
        }

        fn packet_commitment(
            self: @ComponentState<TContractState>,
            port_id: PortId,
            channel_id: ChannelId,
            sequence: Sequence
        ) -> CommitmentValue {
            self.read_packet_commitment(@port_id, @channel_id, @sequence)
        }

        fn packet_receipt(
            self: @ComponentState<TContractState>,
            port_id: PortId,
            channel_id: ChannelId,
            sequence: Sequence
        ) -> bool {
            self.read_packet_receipt(@port_id, @channel_id, @sequence).is_ok()
        }

        fn packet_acknowledgement(
            self: @ComponentState<TContractState>,
            port_id: PortId,
            channel_id: ChannelId,
            sequence: Sequence
        ) -> CommitmentValue {
            self.read_packet_ack(@port_id, @channel_id, @sequence)
        }

        fn next_sequence_send(
            self: @ComponentState<TContractState>, port_id: PortId, channel_id: ChannelId
        ) -> Sequence {
            self.read_next_sequence_send(@port_id, @channel_id)
        }

        fn next_sequence_recv(
            self: @ComponentState<TContractState>, port_id: PortId, channel_id: ChannelId
        ) -> Sequence {
            self.read_next_sequence_recv(@port_id, @channel_id)
        }
    }

    // -----------------------------------------------------------
    // Channel handler implementations
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl SendPacketImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>
    > of SendPacketTrait<TContractState> {
        fn send_packet_validate(
            self: @ComponentState<TContractState>, packet: Packet, chan_end_on_a: ChannelEnd
        ) {
            packet.validate_basic();

            self.verify_send_sequence_matches(@packet);

            chan_end_on_a.validate(@packet.port_id_on_b, @packet.chan_id_on_b);

            // TODO: verify connection end if we ever decide to implement ICS-03

            let client = self.get_client(chan_end_on_a.client_id.client_type);

            let client_sequence = chan_end_on_a.client_id.sequence;

            client.verify_is_active(client_sequence);

            assert(packet.is_timeout_set(), ChannelErrors::MISSING_PACKET_TIMEOUT);

            packet
                .verify_not_timed_out(
                    @client.latest_height(client_sequence),
                    @client.latest_timestamp(client_sequence)
                );
        }

        fn send_packet_execute(
            ref self: ComponentState<TContractState>, packet: Packet, chan_end_on_a: ChannelEnd
        ) {
            let mut seq_on_a = self
                .read_next_sequence_send(@packet.port_id_on_a, @packet.chan_id_on_a);

            self
                .write_next_sequence_send(
                    @packet.port_id_on_a, @packet.chan_id_on_a, seq_on_a.increment()
                );

            let app = self.get_app(@packet.port_id_on_a);

            let json_packet_data = app.json_packet_data(packet.data.clone());

            let packet_commitment_on_a = compute_packet_commtiment(
                @json_packet_data,
                packet.timeout_height_on_b.clone(),
                packet.timeout_timestamp_on_b.clone()
            );

            self
                .write_packet_commitment(
                    @packet.port_id_on_a,
                    @packet.chan_id_on_a,
                    @packet.seq_on_a,
                    packet_commitment_on_a
                );

            self.emit_send_packet_event(packet, chan_end_on_a.ordering);
        }

        fn verify_send_sequence_matches(self: @ComponentState<TContractState>, packet: @Packet) {
            let expected_sequence = self
                .read_next_sequence_send(packet.port_id_on_a, packet.chan_id_on_a);

            assert(@expected_sequence == packet.seq_on_a, ChannelErrors::INVALID_PACKET_SEQUENCE)
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

            msg
                .packet
                .verify_not_timed_out(
                    @HeightImpl::new(0, get_block_number()), @get_block_timestamp().into()
                );

            let client = self.get_client(chan_end_on_b.client_id.client_type);

            client.verify_is_active(chan_end_on_b.client_id.sequence);

            client.verify_proof_height(@msg.proof_height_on_a, chan_end_on_b.client_id.sequence);

            let app = self.get_app(@msg.packet.port_id_on_a);

            let json_packet_data = app.json_packet_data(msg.packet.data.clone());

            self
                .verify_packet_commitment(
                    @client, chan_end_on_b.clone(), msg.clone(), json_packet_data
                );

            match @chan_end_on_b.ordering {
                ChannelOrdering::Unordered => {
                    let receipt = self
                        .read_packet_receipt(
                            @msg.packet.port_id_on_b, @msg.packet.chan_id_on_b, @msg.packet.seq_on_a
                        );

                    assert(receipt.is_none(), ChannelErrors::MISSING_PACKET_RECEIPT);

                    let ack_exists = self
                        .packet_ack_exists(
                            @msg.packet.port_id_on_b, @msg.packet.chan_id_on_b, @msg.packet.seq_on_a
                        );

                    assert(!ack_exists, ChannelErrors::ACK_ALREADY_EXISTS);
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
                        let ack_exists = self
                            .packet_ack_exists(
                                @msg.packet.port_id_on_b,
                                @msg.packet.chan_id_on_b,
                                @msg.packet.seq_on_a
                            );

                        assert(!ack_exists, ChannelErrors::ACK_ALREADY_EXISTS);
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
                    compute_ack_commitment(ack.clone())
                );

            self.emit_recv_packet_event(msg.packet.clone(), chan_end_on_b.ordering);

            self.emit_write_ack_event(msg.packet, ack);
        }

        fn verify_packet_commitment(
            self: @ComponentState<TContractState>,
            client: @ClientContract,
            chan_end_on_b: ChannelEnd,
            msg: MsgRecvPacket,
            json_packet_data: ByteArray
        ) {
            let packet_commitment_on_a = compute_packet_commtiment(
                @json_packet_data,
                msg.packet.timeout_height_on_b.clone(),
                msg.packet.timeout_timestamp_on_b.clone()
            );

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
    }

    #[generate_trait]
    pub(crate) impl AckPacketImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>
    > of AckPacketTrait<TContractState> {
        fn ack_packet_validate(
            self: @ComponentState<TContractState>, msg: MsgAckPacket, chan_end_on_a: ChannelEnd
        ) {
            msg.validate_basic();

            // TODO: verify connection end if we ever decide to implement ICS-03

            let packet = @msg.packet;

            chan_end_on_a.validate(packet.port_id_on_a, packet.chan_id_on_a);

            let app = self.get_app(packet.port_id_on_a);

            let json_packet_data = app.json_packet_data(packet.data.clone());

            self.verify_packet_commitment_matches(packet, @json_packet_data);

            if chan_end_on_a.is_ordered() {
                self.verify_ack_sequence_matches(packet);
            }

            let client = self.get_client(chan_end_on_a.client_id.client_type);

            let client_sequence = chan_end_on_a.client_id.sequence;

            client.verify_is_active(client_sequence);

            client.verify_proof_height(@msg.proof_height_on_a, client_sequence);

            self.verify_packet_acknowledgement(@client, chan_end_on_a, msg);
        }

        fn ack_packet_execute(
            ref self: ComponentState<TContractState>, msg: MsgAckPacket, chan_end_on_a: ChannelEnd
        ) {
            let mut packet = msg.packet;

            let app = self.get_app(@packet.port_id_on_b);

            app.on_ack_packet(packet.clone(), msg.acknowledgement);

            self
                .delete_packet_commitment(
                    @packet.port_id_on_a, @packet.chan_id_on_a, @packet.seq_on_a
                );

            self.emit_ack_packet_event(packet.clone(), chan_end_on_a.ordering.clone());

            if chan_end_on_a.is_ordered() {
                self
                    .write_next_sequence_ack(
                        @packet.port_id_on_a, @packet.chan_id_on_a, packet.seq_on_a.increment()
                    );
            }
        }

        fn verify_packet_acknowledgement(
            self: @ComponentState<TContractState>,
            client: @ClientContract,
            chan_end_on_a: ChannelEnd,
            msg: MsgAckPacket,
        ) {
            let ack_commitment_on_a = compute_ack_commitment(msg.acknowledgement.clone());

            let mut path: ByteArray =
                "Ibc/"; // Setting prefix manually for now. This should come from the connection layer once implemented.

            let ack_path = ack_path(
                msg.packet.port_id_on_a.clone(),
                msg.packet.chan_id_on_a.clone(),
                msg.packet.seq_on_a.clone()
            );

            path.append(@ack_path);

            client
                .verify_membership(
                    chan_end_on_a.client_id.sequence,
                    path,
                    ack_commitment_on_a,
                    msg.proof_ack_on_a.clone()
                );
        }

        /// Verifies if the packet commitment matches the one stored earlier
        /// during the send packet process. If it doesn't exist or doesn't match,
        /// an error is returned. Note that this logic differs from ibc-rs, where
        /// non-existence is a no-op to avoid the relayer paying transaction fees.
        /// Here, in any case, the relayer pays a fee regardless.
        fn verify_packet_commitment_matches(
            self: @ComponentState<TContractState>, packet: @Packet, json_packet_data: @ByteArray
        ) {
            let packet_commitment = self
                .read_packet_commitment(packet.port_id_on_a, packet.chan_id_on_a, packet.seq_on_a);

            let expected_packet_commitment = compute_packet_commtiment(
                json_packet_data,
                packet.timeout_height_on_b.clone(),
                packet.timeout_timestamp_on_b.clone()
            );
            assert(
                packet_commitment == expected_packet_commitment,
                ChannelErrors::MISMATCHED_PACKET_COMMITMENT
            );
        }

        fn verify_ack_sequence_matches(self: @ComponentState<TContractState>, packet: @Packet) {
            let expected_sequence = self
                .read_next_sequence_ack(packet.port_id_on_a, packet.chan_id_on_a);

            assert(@expected_sequence == packet.seq_on_a, ChannelErrors::INVALID_PACKET_SEQUENCE)
        }
    }

    // -----------------------------------------------------------
    // Channel accesses
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl ChannelAccessImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>
    > of ChannelAccessTrait<TContractState> {
        fn get_client(
            self: @ComponentState<TContractState>, client_type: felt252
        ) -> ClientContract {
            let client_comp = get_dep_component!(self, ClientHandler);

            client_comp.get_client(client_type)
        }

        fn get_app(self: @ComponentState<TContractState>, port_id: @PortId) -> AppContract {
            let router_comp = get_dep_component!(self, RouterHandler);

            router_comp.get_app(port_id)
        }
    }

    // -----------------------------------------------------------
    // Channel reader/writer
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl ChannelReaderImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ChannelReaderTrait<TContractState> {
        fn read_channel_end(
            self: @ComponentState<TContractState>, port_id: @PortId, channel_id: @ChannelId
        ) -> ChannelEnd {
            let channel_end = self.channel_ends.read(channel_end_key(port_id, channel_id));

            assert(!channel_end.is_zero(), ChannelErrors::MISSING_CHANNEL_END);

            channel_end
        }

        fn read_packet_commitment(
            self: @ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence
        ) -> CommitmentValue {
            let commitment = self
                .packet_commitments
                .read(commitment_key(port_id, channel_id, sequence));

            assert(commitment.is_non_zero(), ChannelErrors::MISSING_PACKET_COMMITMENT);

            commitment
        }

        fn read_packet_receipt(
            self: @ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence
        ) -> Receipt {
            self.packet_receipts.read(receipt_key(port_id, channel_id, sequence))
        }

        fn read_packet_ack(
            self: @ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence
        ) -> CommitmentValue {
            let ack = self.packet_acks.read(ack_key(port_id, channel_id, sequence));

            assert(ack.is_non_zero(), ChannelErrors::MISSING_PACKET_ACK);

            ack
        }

        fn packet_ack_exists(
            self: @ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence
        ) -> bool {
            self.packet_acks.read(ack_key(port_id, channel_id, sequence)).is_non_zero()
        }

        fn read_next_sequence_send(
            self: @ComponentState<TContractState>, port_id: @PortId, channel_id: @ChannelId
        ) -> Sequence {
            self.send_sequences.read(next_sequence_send_key(port_id, channel_id))
        }

        fn read_next_sequence_recv(
            self: @ComponentState<TContractState>, port_id: @PortId, channel_id: @ChannelId
        ) -> Sequence {
            self.recv_sequences.read(next_sequence_recv_key(port_id, channel_id))
        }

        fn read_next_sequence_ack(
            self: @ComponentState<TContractState>, port_id: @PortId, channel_id: @ChannelId
        ) -> Sequence {
            self.ack_sequences.read(next_sequence_ack_key(port_id, channel_id))
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
            self.channel_ends.write(channel_end_key(port_id, channel_id), channel_end);
        }

        fn write_packet_commitment(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence,
            commitment: CommitmentValue
        ) {
            self
                .packet_commitments
                .write(commitment_key(port_id, channel_id, sequence), commitment);
        }

        fn delete_packet_commitment(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence,
        ) {
            self
                .packet_commitments
                .write(commitment_key(port_id, channel_id, sequence), CommitmentValueZero::zero());
        }

        fn write_packet_receipt(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence,
            receipt: Receipt
        ) {
            self.packet_receipts.write(receipt_key(port_id, channel_id, sequence), receipt);
        }

        fn write_packet_ack(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence,
            ack_commitment: CommitmentValue
        ) {
            self.packet_acks.write(ack_key(port_id, channel_id, sequence), ack_commitment);
        }

        fn write_next_sequence_send(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: Sequence
        ) {
            self.send_sequences.write(next_sequence_send_key(port_id, channel_id), sequence);
        }

        fn write_next_sequence_recv(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: Sequence
        ) {
            self.recv_sequences.write(next_sequence_recv_key(port_id, channel_id), sequence);
        }

        fn write_next_sequence_ack(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: Sequence
        ) {
            self.ack_sequences.write(next_sequence_ack_key(port_id, channel_id), sequence);
        }
    }

    // -----------------------------------------------------------
    // Channel Event Emitter
    // -----------------------------------------------------------

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

// ----------------- Temporary until handshakes are implemented ---------------
pub(crate) fn CLIENT_ID() -> ClientId {
    ClientId { client_type: '07-cometbft', sequence: 0 }
}

pub(crate) fn PORT_ID() -> PortId {
    PortId { port_id: "transfer" }
}

pub(crate) fn CHANNEL_ID(sequence: u64) -> ChannelId {
    ChannelId { channel_id: format!("channel-{sequence}") }
}

pub(crate) fn CHANNEL_END(counterparty_channel_sequence: u64) -> ChannelEnd {
    ChannelEnd {
        state: ChannelState::Open,
        ordering: ChannelOrdering::Unordered,
        remote: Counterparty {
            port_id: PORT_ID(), channel_id: CHANNEL_ID(counterparty_channel_sequence),
        },
        client_id: CLIENT_ID(),
    }
}
