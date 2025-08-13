#[starknet::component]
pub mod ChannelHandlerComponent {
    use ChannelEventEmitterComponent::ChannelEventEmitterTrait;
    use ClientHandlerComponent::ClientInternalTrait;
    use ConnectionHandlerComponent::CoreConnectionQuery;
    use RouterHandlerComponent::RouterInternalTrait;
    use core::num::traits::Zero;
    use starknet::storage::{
        Map, StorageMapReadAccess, StorageMapWriteAccess, StoragePointerReadAccess,
        StoragePointerWriteAccess,
    };
    use starknet::{get_block_number, get_block_timestamp};
    use starknet_ibc_core::channel::{
        Acknowledgement, AppVersion, ChannelEnd, ChannelEndTrait, ChannelErrors,
        ChannelEventEmitterComponent, ChannelOrdering, IChannelHandler, IChannelQuery, MsgAckPacket,
        MsgChanOpenAck, MsgChanOpenConfirm, MsgChanOpenInit, MsgChanOpenTry, MsgRecvPacket,
        MsgTimeoutPacket, Packet, PacketTrait, Receipt, ReceiptTrait,
    };
    use starknet_ibc_core::client::{
        ClientContract, ClientContractTrait, ClientHandlerComponent, Height, HeightImpl,
    };
    use starknet_ibc_core::commitment::{
        Commitment, CommitmentZero, StateProof, compute_ack_commitment, compute_packet_commitment,
    };
    use starknet_ibc_core::connection::{
        ConnectionEnd, ConnectionEndTrait, ConnectionErrors, ConnectionHandlerComponent,
        VersionTrait,
    };
    use starknet_ibc_core::host::{
        ChannelId, ChannelIdImpl, ChannelIdZero, ClientIdImpl, ConnectionId, ConnectionIdZero,
        PortId, Sequence, SequenceImpl, SequencePartialOrd, SequenceTrait, SequenceZero, ack_key,
        ack_path, channel_end_key, channel_end_path, commitment_key, commitment_path,
        next_sequence_ack_key, next_sequence_recv_key, next_sequence_recv_path,
        next_sequence_send_key, receipt_key, receipt_path,
    };
    use starknet_ibc_core::router::{AppContract, AppContractTrait, RouterHandlerComponent};
    use starknet_ibc_utils::{ValidateBasic, poseidon_hash};

    #[storage]
    pub struct Storage {
        pub next_channel_sequence: u64,
        pub channel_ends: Map<felt252, ChannelEnd>,
        pub packet_commitments: Map<felt252, Commitment>,
        pub packet_receipts: Map<felt252, Receipt>,
        pub packet_acks: Map<felt252, Commitment>,
        pub send_sequences: Map<felt252, Sequence>,
        pub recv_sequences: Map<felt252, Sequence>,
        pub ack_sequences: Map<felt252, Sequence>,
        //
        // The following fields are used to generate storage proofs for IBC.
        // Since Starknet storage stores data per field,
        // we (poseidon) hash the intended data and store
        // the hash as storage commitment.
        pub channel_ends_commitments: Map<felt252, felt252>,
        pub packet_commitments_commitments: Map<felt252, felt252>,
        pub packet_receipts_commitments: Map<felt252, felt252>,
        pub packet_acks_commitments: Map<felt252, felt252>,
        pub send_sequences_commitments: Map<felt252, felt252>,
        pub recv_sequences_commitments: Map<felt252, felt252>,
        pub ack_sequences_commitments: Map<felt252, felt252>,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {}

    // -----------------------------------------------------------
    // IChannelHandler
    // -----------------------------------------------------------

    // NOTE: Authorized relayer check is temporary and will be removed once
    // commitment proof verification is implemented.

    #[embeddable_as(CoreChannelHandler)]
    impl CoreChannelHandlerImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl ConnectionHandler: ConnectionHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>,
    > of IChannelHandler<ComponentState<TContractState>> {
        fn chan_open_init(
            ref self: ComponentState<TContractState>, msg: MsgChanOpenInit,
        ) -> ChannelId {
            let channel_sequence = self.read_next_channel_sequence();
            self.chan_open_init_validate(@msg);
            self.chan_open_init_execute(channel_sequence, msg)
        }

        fn chan_open_try(
            ref self: ComponentState<TContractState>, msg: MsgChanOpenTry,
        ) -> ChannelId {
            let channel_sequence = self.read_next_channel_sequence();
            self.chan_open_try_validate(@msg);
            self.chan_open_try_execute(channel_sequence, msg)
        }

        fn chan_open_ack(ref self: ComponentState<TContractState>, msg: MsgChanOpenAck) {
            let chan_end_on_a = self.read_channel_end(@msg.port_id_on_a, @msg.chan_id_on_a);
            self.chan_open_ack_validate(@chan_end_on_a, @msg);
            self.chan_open_ack_execute(chan_end_on_a, msg);
        }

        fn chan_open_confirm(ref self: ComponentState<TContractState>, msg: MsgChanOpenConfirm) {
            let chan_end_on_b = self.read_channel_end(@msg.port_id_on_b, @msg.chan_id_on_b);
            self.chan_open_confirm_validate(@chan_end_on_b, @msg);
            self.chan_open_confirm_execute(chan_end_on_b, msg);
        }

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
                .read_channel_end(@msg.packet.port_id_on_a, @msg.packet.chan_id_on_a);
            self.ack_packet_validate(msg.clone(), chan_end_on_a.clone());
            self.ack_packet_execute(msg, chan_end_on_a);
        }

        fn timeout_packet(ref self: ComponentState<TContractState>, msg: MsgTimeoutPacket) {
            let chan_end_on_a = self
                .read_channel_end(@msg.packet.port_id_on_a, @msg.packet.chan_id_on_a);
            self.timeout_packet_validate(msg.clone(), chan_end_on_a.clone());
            self.timeout_packet_execute(msg, chan_end_on_a);
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
            self: @ComponentState<TContractState>, port_id: PortId, channel_id: ChannelId,
        ) -> ChannelEnd {
            self.read_channel_end(@port_id, @channel_id)
        }

        fn packet_commitment(
            self: @ComponentState<TContractState>,
            port_id: PortId,
            channel_id: ChannelId,
            sequence: Sequence,
        ) -> Option<Commitment> {
            let commitment = self.read_packet_commitment(@port_id, @channel_id, @sequence);

            if commitment.is_non_zero() {
                Option::Some(commitment)
            } else {
                Option::None
            }
        }

        fn packet_receipt(
            self: @ComponentState<TContractState>,
            port_id: PortId,
            channel_id: ChannelId,
            sequence: Sequence,
        ) -> bool {
            self.read_packet_receipt(@port_id, @channel_id, @sequence).is_ok()
        }

        fn packet_acknowledgement(
            self: @ComponentState<TContractState>,
            port_id: PortId,
            channel_id: ChannelId,
            sequence: Sequence,
        ) -> Commitment {
            self.read_packet_ack(@port_id, @channel_id, @sequence)
        }

        fn packet_commitment_sequences(
            self: @ComponentState<TContractState>, port_id: PortId, channel_id: ChannelId,
        ) -> Array<Sequence> {
            let mut send_sequences = ArrayTrait::new();
            let mut send_seq = self.read_next_sequence_send(@port_id, @channel_id).decrement();
            while let Option::Some(sequence) = @send_seq {
                if self
                    .packet_commitments
                    .read(commitment_key(@port_id, @channel_id, sequence))
                    .is_non_zero() {
                    send_sequences.append(sequence.clone());
                }
                send_seq = send_seq.unwrap().decrement();
            }
            send_sequences
        }

        fn packet_ack_sequences(
            self: @ComponentState<TContractState>,
            port_id: PortId,
            channel_id: ChannelId,
            sequences: Array<Sequence>,
        ) -> Array<Sequence> {
            assert(sequences.len() > 0, ChannelErrors::EMPTY_SEQUENCE_LIST);
            let mut ack_sequences = ArrayTrait::new();
            for seq in sequences {
                if self.packet_acks.read(ack_key(@port_id, @channel_id, @seq)).is_non_zero() {
                    ack_sequences.append(seq);
                }
            }
            ack_sequences
        }

        fn unreceived_packet_sequences(
            self: @ComponentState<TContractState>,
            port_id: PortId,
            channel_id: ChannelId,
            sequences: Array<Sequence>,
        ) -> Array<Sequence> {
            assert(sequences.len() > 0, ChannelErrors::EMPTY_SEQUENCE_LIST);
            let mut unreceived_sequences = ArrayTrait::new();
            for seq in sequences {
                if self.read_packet_receipt(@port_id, @channel_id, @seq).is_none() {
                    unreceived_sequences.append(seq);
                }
            }
            unreceived_sequences
        }

        fn unreceived_ack_sequences(
            self: @ComponentState<TContractState>,
            port_id: PortId,
            channel_id: ChannelId,
            sequences: Array<Sequence>,
        ) -> Array<Sequence> {
            assert(sequences.len() > 0, ChannelErrors::EMPTY_SEQUENCE_LIST);
            let mut unreceived_sequences = ArrayTrait::new();
            for seq in sequences {
                if self
                    .packet_commitments
                    .read(commitment_key(@port_id, @channel_id, @seq))
                    .is_non_zero() {
                    unreceived_sequences.append(seq)
                };
            }
            unreceived_sequences
        }

        fn next_sequence_send(
            self: @ComponentState<TContractState>, port_id: PortId, channel_id: ChannelId,
        ) -> Sequence {
            self.read_next_sequence_send(@port_id, @channel_id)
        }

        fn next_sequence_recv(
            self: @ComponentState<TContractState>, port_id: PortId, channel_id: ChannelId,
        ) -> Sequence {
            self.read_next_sequence_recv(@port_id, @channel_id)
        }
    }

    // -----------------------------------------------------------
    // Channel handler implementations
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl ChanOpenInitImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl ConnectionHandler: ConnectionHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>,
    > of ChanOpenInitTrait<TContractState> {
        fn chan_open_init_validate(self: @ComponentState<TContractState>, msg: @MsgChanOpenInit) {
            msg.validate_basic();

            let conn_end_on_a = self.get_connection(msg.conn_id_on_a.clone());

            // NOTE: Not needed check if the connection end is OPEN. Optimistic
            // channel handshake is allowed.

            assert(
                conn_end_on_a.version.is_feature_supported(@(*msg.ordering).into()),
                ChannelErrors::UNSUPPORTED_ORDERING,
            );

            let client = self.get_client(conn_end_on_a.client_id.client_type);

            client.verify_is_active(conn_end_on_a.client_id.sequence);
        }

        fn chan_open_init_execute(
            ref self: ComponentState<TContractState>, channel_sequence: u64, msg: MsgChanOpenInit,
        ) -> ChannelId {
            let chan_id_on_a = ChannelIdImpl::new(channel_sequence);

            let app = self.get_app(@msg.port_id_on_a);

            let version_on_a = app
                .on_chan_open_init(
                    msg.port_id_on_a.clone(),
                    chan_id_on_a.clone(),
                    msg.conn_id_on_a.clone(),
                    msg.port_id_on_b.clone(),
                    msg.version_proposal.clone(),
                    msg.ordering,
                );

            let chan_end_on_a = ChannelEndTrait::init(
                msg.ordering,
                msg.port_id_on_b.clone(),
                msg.conn_id_on_a.clone(),
                msg.version_proposal.clone(),
            );

            self.write_channel_end(@msg.port_id_on_a, @chan_id_on_a, chan_end_on_a);

            self.write_next_channel_sequence(channel_sequence + 1);

            self.write_next_sequence_send(@msg.port_id_on_a, @chan_id_on_a, SequenceImpl::one());

            self.write_next_sequence_recv(@msg.port_id_on_a, @chan_id_on_a, SequenceImpl::one());

            self.write_next_sequence_ack(@msg.port_id_on_a, @chan_id_on_a, SequenceImpl::one());

            self
                .emit_chan_open_init_event(
                    msg.port_id_on_a,
                    chan_id_on_a.clone(),
                    msg.port_id_on_b,
                    msg.conn_id_on_a,
                    version_on_a,
                );

            chan_id_on_a
        }
    }

    #[generate_trait]
    pub(crate) impl ChanOpenTryImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl ConnectionHandler: ConnectionHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>,
    > of ChanOpenTryTrait<TContractState> {
        fn chan_open_try_validate(self: @ComponentState<TContractState>, msg: @MsgChanOpenTry) {
            msg.validate_basic();

            let conn_end_on_b = self.get_connection(msg.conn_id_on_b.clone());

            assert(conn_end_on_b.is_open(), ConnectionErrors::INVALID_CONNECTION_STATE);

            assert(
                conn_end_on_b.version.is_feature_supported(@(*msg.ordering).into()),
                ChannelErrors::UNSUPPORTED_ORDERING,
            );

            let client = self.get_client(conn_end_on_b.client_id.client_type);

            client.verify_is_active(conn_end_on_b.client_id.sequence);

            let expected_chan_end_on_a = ChannelEndTrait::init(
                *msg.ordering,
                msg.port_id_on_b.clone(),
                conn_end_on_b.counterparty.connection_id.clone(),
                msg.version_on_a.clone(),
            );

            self
                .verify_channel_end(
                    @client,
                    conn_end_on_b,
                    msg.chan_id_on_a.clone(),
                    expected_chan_end_on_a,
                    msg.proof_chan_end_on_a.clone(),
                    *msg.proof_height_on_a,
                );
        }

        fn chan_open_try_execute(
            ref self: ComponentState<TContractState>, channel_sequence: u64, msg: MsgChanOpenTry,
        ) -> ChannelId {
            let chan_id_on_b = ChannelIdImpl::new(channel_sequence);

            let app = self.get_app(@msg.port_id_on_b);

            let version_on_b = app
                .on_chan_open_try(
                    msg.port_id_on_b.clone(),
                    chan_id_on_b.clone(),
                    msg.conn_id_on_b.clone(),
                    msg.port_id_on_a.clone(),
                    msg.version_on_a.clone(),
                    msg.ordering,
                );

            let chan_end_on_b = ChannelEndTrait::try_open(
                msg.ordering,
                msg.port_id_on_a.clone(),
                msg.chan_id_on_a.clone(),
                msg.conn_id_on_b.clone(),
                msg.version_on_a.clone(),
            );

            self.write_channel_end(@msg.port_id_on_b, @chan_id_on_b, chan_end_on_b);

            self.write_next_channel_sequence(channel_sequence + 1);

            self.write_next_sequence_send(@msg.port_id_on_b, @chan_id_on_b, SequenceImpl::one());

            self.write_next_sequence_recv(@msg.port_id_on_b, @chan_id_on_b, SequenceImpl::one());

            self.write_next_sequence_ack(@msg.port_id_on_b, @chan_id_on_b, SequenceImpl::one());

            self
                .emit_chan_open_try_event(
                    msg.port_id_on_b,
                    chan_id_on_b.clone(),
                    msg.port_id_on_a,
                    msg.chan_id_on_a,
                    msg.conn_id_on_b,
                    version_on_b,
                );

            chan_id_on_b
        }
    }

    #[generate_trait]
    pub(crate) impl ChanOpenAckImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl ConnectionHandler: ConnectionHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>,
    > of ChanOpenAckTrait<TContractState> {
        fn chan_open_ack_validate(
            self: @ComponentState<TContractState>, chan_en_on_a: @ChannelEnd, msg: @MsgChanOpenAck,
        ) {
            msg.validate_basic();

            assert(chan_en_on_a.is_init(), ChannelErrors::INVALID_CHANNEL_STATE);

            let conn_end_on_a = self.get_connection(chan_en_on_a.connection_id.clone());

            assert(conn_end_on_a.is_open(), ConnectionErrors::INVALID_CONNECTION_STATE);

            let client = self.get_client(conn_end_on_a.client_id.client_type);

            client.verify_is_active(conn_end_on_a.client_id.sequence);

            let expected_chan_end_on_b = ChannelEndTrait::try_open(
                *chan_en_on_a.ordering,
                msg.port_id_on_a.clone(),
                msg.chan_id_on_a.clone(),
                conn_end_on_a.counterparty.connection_id.clone(),
                msg.version_on_b.clone(),
            );

            self
                .verify_channel_end(
                    @client,
                    conn_end_on_a,
                    msg.chan_id_on_b.clone(),
                    expected_chan_end_on_b,
                    msg.proof_chan_end_on_b.clone(),
                    *msg.proof_height_on_b,
                );
        }

        fn chan_open_ack_execute(
            ref self: ComponentState<TContractState>,
            chan_end_on_a: ChannelEnd,
            msg: MsgChanOpenAck,
        ) {
            let app = self.get_app(@msg.port_id_on_a);

            app
                .on_chan_open_ack(
                    msg.port_id_on_a.clone(), msg.chan_id_on_a.clone(), msg.version_on_b.clone(),
                );

            self
                .write_channel_end(
                    @msg.port_id_on_a,
                    @msg.chan_id_on_a,
                    chan_end_on_a
                        .clone()
                        .to_open_with_params(msg.chan_id_on_b.clone(), msg.version_on_b),
                );

            self
                .emit_chan_open_ack_event(
                    msg.port_id_on_a,
                    msg.chan_id_on_a,
                    chan_end_on_a.counterparty_port_id().clone(),
                    msg.chan_id_on_b,
                    chan_end_on_a.connection_id,
                );
        }
    }

    #[generate_trait]
    pub(crate) impl ChanOpenConfirmImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl ConnectionHandler: ConnectionHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>,
    > of ChanOpenConfirmTrait<TContractState> {
        fn chan_open_confirm_validate(
            self: @ComponentState<TContractState>,
            chan_end_on_b: @ChannelEnd,
            msg: @MsgChanOpenConfirm,
        ) {
            msg.validate_basic();

            assert(chan_end_on_b.is_try_open(), ChannelErrors::INVALID_CHANNEL_STATE);

            let conn_end_on_b = self.get_connection(chan_end_on_b.connection_id.clone());

            assert(conn_end_on_b.is_open(), ConnectionErrors::INVALID_CONNECTION_STATE);

            let client = self.get_client(conn_end_on_b.client_id.client_type);

            client.verify_is_active(conn_end_on_b.client_id.sequence);

            let expected_chan_end_on_a = ChannelEndTrait::open(
                *chan_end_on_b.ordering,
                msg.port_id_on_b.clone(),
                msg.chan_id_on_b.clone(),
                conn_end_on_b.counterparty.connection_id.clone(),
                chan_end_on_b.version.clone(),
            );

            self
                .verify_channel_end(
                    @client,
                    conn_end_on_b.clone(),
                    chan_end_on_b.counterparty_channel_id().clone(),
                    expected_chan_end_on_a,
                    msg.proof_chan_end_on_a.clone(),
                    *msg.proof_height_on_a,
                );
        }

        fn chan_open_confirm_execute(
            ref self: ComponentState<TContractState>,
            chan_end_on_b: ChannelEnd,
            msg: MsgChanOpenConfirm,
        ) {
            let app = self.get_app(@msg.port_id_on_b);

            app.on_chan_open_confirm(msg.port_id_on_b.clone(), msg.chan_id_on_b.clone());

            self
                .write_channel_end(
                    @msg.port_id_on_b, @msg.chan_id_on_b, chan_end_on_b.clone().to_open(),
                );

            self
                .emit_chan_open_confirm_event(
                    chan_end_on_b.counterparty_port_id().clone(),
                    chan_end_on_b.counterparty_channel_id().clone(),
                    chan_end_on_b.connection_id,
                    msg,
                );
        }
    }

    #[generate_trait]
    pub(crate) impl SendPacketImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl ConnectionHandler: ConnectionHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>,
    > of SendPacketTrait<TContractState> {
        fn send_packet_validate(
            self: @ComponentState<TContractState>, packet: Packet, chan_end_on_a: ChannelEnd,
        ) {
            packet.validate_basic();

            self.verify_send_sequence_matches(@packet);

            chan_end_on_a.validate(@packet.port_id_on_b, @packet.chan_id_on_b);

            let conn_end_on_a = self.get_connection(chan_end_on_a.connection_id);

            let client = self.get_client(conn_end_on_a.client_id.client_type);

            let client_sequence = conn_end_on_a.client_id.sequence;

            client.verify_is_active(client_sequence);

            assert(packet.is_timeout_set(), ChannelErrors::MISSING_PACKET_TIMEOUT);

            assert(
                !packet
                    .is_timed_out(
                        @client.latest_height(client_sequence),
                        @client.latest_timestamp(client_sequence),
                    ),
                ChannelErrors::TIMED_OUT_PACKET,
            );
        }

        fn send_packet_execute(
            ref self: ComponentState<TContractState>, packet: Packet, chan_end_on_a: ChannelEnd,
        ) {
            let mut seq_on_a = self
                .read_next_sequence_send(@packet.port_id_on_a, @packet.chan_id_on_a);

            self
                .write_next_sequence_send(
                    @packet.port_id_on_a, @packet.chan_id_on_a, seq_on_a.increment(),
                );

            let app = self.get_app(@packet.port_id_on_a);

            let json_packet_data = app.json_packet_data(packet.data.clone());

            let packet_commitment_on_a = compute_packet_commitment(
                json_packet_data.span(), packet.timeout_height_on_b, packet.timeout_timestamp_on_b,
            );

            self
                .write_packet_commitment(
                    @packet.port_id_on_a,
                    @packet.chan_id_on_a,
                    @packet.seq_on_a,
                    packet_commitment_on_a,
                );

            self.emit_send_packet_event(packet, chan_end_on_a.ordering);
        }
    }

    #[generate_trait]
    pub(crate) impl RecvPacketImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl ConnectionHandler: ConnectionHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>,
    > of RecvPacketTrait<TContractState> {
        fn recv_packet_validate(
            self: @ComponentState<TContractState>, msg: MsgRecvPacket, chan_end_on_b: ChannelEnd,
        ) {
            msg.validate_basic();

            chan_end_on_b.validate(@msg.packet.port_id_on_a, @msg.packet.chan_id_on_a);

            assert(
                !msg
                    .packet
                    .is_timed_out(
                        @HeightImpl::new(0, get_block_number()), @get_block_timestamp().into(),
                    ),
                ChannelErrors::TIMED_OUT_PACKET,
            );

            let conn_end_on_a = self.get_connection(chan_end_on_b.connection_id);

            let client = self.get_client(conn_end_on_a.client_id.client_type);

            client.verify_is_active(conn_end_on_a.client_id.sequence);

            let app = self.get_app(@msg.packet.port_id_on_a);

            let json_packet_data = app.json_packet_data(msg.packet.data.clone());

            self
                .verify_packet_commitment(
                    @client, conn_end_on_a, msg.clone(), json_packet_data.span(),
                );

            match @chan_end_on_b.ordering {
                ChannelOrdering::Unordered => {
                    let receipt = self
                        .read_packet_receipt(
                            @msg.packet.port_id_on_b,
                            @msg.packet.chan_id_on_b,
                            @msg.packet.seq_on_a,
                        );

                    assert(receipt.is_none(), ChannelErrors::PACKET_RECEIPT_ALREADY_EXISTS);

                    let ack_exists = self
                        .packet_ack_exists(
                            @msg.packet.port_id_on_b,
                            @msg.packet.chan_id_on_b,
                            @msg.packet.seq_on_a,
                        );

                    assert(!ack_exists, ChannelErrors::PACKET_ACK_ALREADY_EXISTS);
                },
                ChannelOrdering::Ordered => {
                    let next_sequence_recv = self
                        .read_next_sequence_recv(
                            @msg.packet.port_id_on_b, @msg.packet.chan_id_on_b,
                        );

                    assert(
                        @next_sequence_recv >= @msg.packet.seq_on_a,
                        ChannelErrors::INVALID_PACKET_SEQUENCE,
                    );

                    // If the packet sequence matches the expected next
                    // sequence, we check if the ack not exists. As the
                    // existence means the packet was already relayed.
                    if next_sequence_recv == msg.packet.seq_on_a {
                        let ack_exists = self
                            .packet_ack_exists(
                                @msg.packet.port_id_on_b,
                                @msg.packet.chan_id_on_b,
                                @msg.packet.seq_on_a,
                            );

                        assert(!ack_exists, ChannelErrors::PACKET_ACK_ALREADY_EXISTS);
                    }
                },
            };
        }

        fn recv_packet_execute(
            ref self: ComponentState<TContractState>, msg: MsgRecvPacket, chan_end_on_b: ChannelEnd,
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
                            Receipt::Ok,
                        );
                },
                ChannelOrdering::Ordered => {
                    self
                        .write_next_sequence_recv(
                            @msg.packet.port_id_on_b,
                            @msg.packet.chan_id_on_b,
                            msg.packet.seq_on_a.clone(),
                        );
                },
            }

            self
                .write_packet_ack(
                    @msg.packet.port_id_on_b,
                    @msg.packet.chan_id_on_b,
                    @msg.packet.seq_on_a,
                    compute_ack_commitment(ack.clone()),
                );

            self.emit_recv_packet_event(msg.packet.clone(), chan_end_on_b.ordering);

            self.emit_write_ack_event(msg.packet, ack);
        }
    }

    #[generate_trait]
    pub(crate) impl AckPacketImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl ConnectionHandler: ConnectionHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>,
    > of AckPacketTrait<TContractState> {
        fn ack_packet_validate(
            self: @ComponentState<TContractState>, msg: MsgAckPacket, chan_end_on_a: ChannelEnd,
        ) {
            msg.validate_basic();

            let packet = @msg.packet;

            chan_end_on_a.validate(packet.port_id_on_b, packet.chan_id_on_b);

            let conn_end_on_a = self.get_connection(chan_end_on_a.connection_id.clone());

            let client = self.get_client(conn_end_on_a.client_id.client_type);

            client.verify_is_active(conn_end_on_a.client_id.sequence);

            let app = self.get_app(packet.port_id_on_a);

            let json_packet_data = app.json_packet_data(packet.data.clone());

            self.verify_packet_commitment_matches(packet, json_packet_data.span());

            if chan_end_on_a.is_ordered() {
                self.verify_ack_sequence_matches(packet);
            }

            self.verify_packet_acknowledgement(@client, conn_end_on_a, msg);
        }

        fn ack_packet_execute(
            ref self: ComponentState<TContractState>, msg: MsgAckPacket, chan_end_on_a: ChannelEnd,
        ) {
            let mut packet = msg.packet;

            let app = self.get_app(@packet.port_id_on_b);

            app.on_ack_packet(packet.clone(), msg.acknowledgement);

            self
                .delete_packet_commitment(
                    @packet.port_id_on_a, @packet.chan_id_on_a, @packet.seq_on_a,
                );

            self.emit_ack_packet_event(packet.clone(), chan_end_on_a.ordering);

            if chan_end_on_a.is_ordered() {
                self
                    .write_next_sequence_ack(
                        @packet.port_id_on_a, @packet.chan_id_on_a, packet.seq_on_a.increment(),
                    );
            }
        }
    }

    #[generate_trait]
    pub(crate) impl TimeoutPacketImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl ConnectionHandler: ConnectionHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>,
    > of TimeoutPacketTrait<TContractState> {
        fn timeout_packet_validate(
            ref self: ComponentState<TContractState>,
            msg: MsgTimeoutPacket,
            chan_end_on_a: ChannelEnd,
        ) {
            let packet = msg.packet.clone();

            chan_end_on_a.validate(@packet.port_id_on_b, @packet.chan_id_on_b);

            let conn_end_on_a = self.get_connection(chan_end_on_a.connection_id);

            let client = self.get_client(conn_end_on_a.client_id.client_type);

            let client_sequence = conn_end_on_a.client_id.sequence;

            client.verify_is_active(client_sequence);

            let app = self.get_app(@packet.port_id_on_a);

            let json_packet_data = app.json_packet_data(packet.data.clone());

            self.verify_packet_commitment_matches(@packet, json_packet_data.span());

            assert(
                packet
                    .is_timed_out(
                        @client.latest_height(client_sequence),
                        @client.latest_timestamp(client_sequence).into(),
                    ),
                ChannelErrors::PENDING_PACKET,
            );

            match chan_end_on_a.ordering {
                ChannelOrdering::Unordered => {
                    self.verify_receipt_not_exists(@client, conn_end_on_a, msg.clone());
                },
                ChannelOrdering::Ordered => {
                    assert(
                        @packet.seq_on_a >= @msg.next_seq_recv_on_b,
                        ChannelErrors::MISMATCHED_PACKET_SEQUENCE,
                    );

                    self.verify_next_sequence_recv(@client, conn_end_on_a, msg);
                },
            };
        }

        fn timeout_packet_execute(
            ref self: ComponentState<TContractState>,
            msg: MsgTimeoutPacket,
            chan_end_on_a: ChannelEnd,
        ) {
            let mut packet = msg.packet;

            let app = self.get_app(@packet.port_id_on_b);

            app.on_timeout_packet(packet.clone());

            self
                .delete_packet_commitment(
                    @packet.port_id_on_a, @packet.chan_id_on_a, @packet.seq_on_a,
                );

            self.emit_timeout_packet_event(packet.clone(), chan_end_on_a.ordering);

            if chan_end_on_a.is_ordered() {
                self
                    .write_channel_end(
                        @packet.port_id_on_a, @packet.chan_id_on_a, chan_end_on_a.close(),
                    );
                // TODO: emit channel closed event once channel handshake is implemented.
            }
        }
    }

    // -----------------------------------------------------------
    // Channel internals
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl ChannelInternalImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>,
    > of ChannelInternalTrait<TContractState> {
        fn verify_channel_end(
            self: @ComponentState<TContractState>,
            client: @ClientContract,
            connection_end: ConnectionEnd,
            counterparty_channel_id: ChannelId,
            expected_channel_end: ChannelEnd,
            proof: StateProof,
            proof_height: Height,
        ) {
            let client_sequence = connection_end.client_id.sequence;

            client.verify_proof_height(@proof_height, client_sequence);

            let paths = channel_end_path(
                connection_end.counterparty.prefix,
                expected_channel_end.remote.port_id.clone(),
                counterparty_channel_id,
            );

            let root = client.consensus_state_root(client_sequence, proof_height);

            client
                .verify_membership(
                    client_sequence, paths, expected_channel_end.into(), proof, root,
                );
        }

        /// Verifies if the packet commitment matches the one stored earlier
        /// during the send packet process. If it doesn't exist or doesn't match,
        /// an error is returned. Note that this logic differs from ibc-rs, where
        /// non-existence is a no-op to avoid the relayer paying transaction fees.
        /// Here, in any case, the relayer pays a fee regardless.
        fn verify_packet_commitment_matches(
            self: @ComponentState<TContractState>, packet: @Packet, json_packet_data: Span<u8>,
        ) {
            let packet_commitment = self
                .read_packet_commitment(packet.port_id_on_a, packet.chan_id_on_a, packet.seq_on_a);

            assert(packet_commitment.is_non_zero(), ChannelErrors::MISSING_PACKET_COMMITMENT);

            let expected_packet_commitment = compute_packet_commitment(
                json_packet_data, *packet.timeout_height_on_b, *packet.timeout_timestamp_on_b,
            );

            assert(
                packet_commitment == expected_packet_commitment,
                ChannelErrors::MISMATCHED_PACKET_COMMITMENT,
            );
        }

        fn verify_send_sequence_matches(self: @ComponentState<TContractState>, packet: @Packet) {
            let expected_sequence = self
                .read_next_sequence_send(packet.port_id_on_a, packet.chan_id_on_a);

            assert(@expected_sequence == packet.seq_on_a, ChannelErrors::INVALID_PACKET_SEQUENCE)
        }

        fn verify_ack_sequence_matches(self: @ComponentState<TContractState>, packet: @Packet) {
            let expected_sequence = self
                .read_next_sequence_ack(packet.port_id_on_a, packet.chan_id_on_a);

            assert(@expected_sequence == packet.seq_on_a, ChannelErrors::INVALID_PACKET_SEQUENCE)
        }

        fn verify_packet_commitment(
            self: @ComponentState<TContractState>,
            client: @ClientContract,
            connection_end: ConnectionEnd,
            msg: MsgRecvPacket,
            json_packet_data: Span<u8>,
        ) {
            let client_sequence = connection_end.client_id.sequence;

            client.verify_proof_height(@msg.proof_height_on_a, client_sequence);

            let paths = commitment_path(
                connection_end.counterparty.prefix,
                msg.packet.port_id_on_a,
                msg.packet.chan_id_on_a,
                msg.packet.seq_on_a,
            );

            let packet_commitment_on_a = compute_packet_commitment(
                json_packet_data, msg.packet.timeout_height_on_b, msg.packet.timeout_timestamp_on_b,
            );

            let root_on_a = client.consensus_state_root(client_sequence, msg.proof_height_on_a);

            client
                .verify_membership(
                    client_sequence,
                    paths,
                    packet_commitment_on_a.into(),
                    msg.proof_commitment_on_a,
                    root_on_a,
                );
        }

        fn verify_packet_acknowledgement(
            self: @ComponentState<TContractState>,
            client: @ClientContract,
            connection_end: ConnectionEnd,
            msg: MsgAckPacket,
        ) {
            let client_sequence = connection_end.client_id.sequence;

            client.verify_proof_height(@msg.proof_height_on_b, client_sequence);

            let paths = ack_path(
                connection_end.counterparty.prefix,
                msg.packet.port_id_on_a,
                msg.packet.chan_id_on_a,
                msg.packet.seq_on_a,
            );

            let ack_commitment_on_a = compute_ack_commitment(msg.acknowledgement);

            let root_on_b = client.consensus_state_root(client_sequence, msg.proof_height_on_b);

            client
                .verify_membership(
                    client_sequence,
                    paths,
                    ack_commitment_on_a.into(),
                    msg.proof_ack_on_b,
                    root_on_b,
                );
        }

        fn verify_receipt_not_exists(
            self: @ComponentState<TContractState>,
            client: @ClientContract,
            connection_end: ConnectionEnd,
            msg: MsgTimeoutPacket,
        ) {
            let client_sequence = connection_end.client_id.sequence;

            client.verify_proof_height(@msg.proof_height_on_b, client_sequence);

            let paths = receipt_path(
                connection_end.counterparty.prefix,
                msg.packet.port_id_on_b,
                msg.packet.chan_id_on_b,
                msg.packet.seq_on_a,
            );

            let root_on_b = client.consensus_state_root(client_sequence, msg.proof_height_on_b);

            client
                .verify_non_membership(
                    client_sequence, paths, msg.proof_unreceived_on_b, root_on_b,
                );
        }

        fn verify_next_sequence_recv(
            self: @ComponentState<TContractState>,
            client: @ClientContract,
            connection_end: ConnectionEnd,
            msg: MsgTimeoutPacket,
        ) {
            let client_sequence = connection_end.client_id.sequence;

            client.verify_proof_height(@msg.proof_height_on_b, client_sequence);

            let paths = next_sequence_recv_path(
                connection_end.counterparty.prefix,
                msg.packet.port_id_on_b,
                msg.packet.chan_id_on_b,
            );

            let root_on_b = client.consensus_state_root(client_sequence, msg.proof_height_on_b);

            client
                .verify_membership(
                    client_sequence,
                    paths,
                    msg.packet.seq_on_a.into(),
                    msg.proof_unreceived_on_b,
                    root_on_b,
                );
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
        impl ConnectionHandler: ConnectionHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>,
    > of ChannelAccessTrait<TContractState> {
        fn get_client(
            self: @ComponentState<TContractState>, client_type: felt252,
        ) -> ClientContract {
            let client_comp = get_dep_component!(self, ClientHandler);

            client_comp.get_client(client_type)
        }

        fn get_connection(
            self: @ComponentState<TContractState>, connection_id: ConnectionId,
        ) -> ConnectionEnd {
            let connection_comp = get_dep_component!(self, ConnectionHandler);

            connection_comp.connection_end(connection_id)
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
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of ChannelReaderTrait<TContractState> {
        fn read_next_channel_sequence(self: @ComponentState<TContractState>) -> u64 {
            self.next_channel_sequence.read()
        }

        fn read_channel_end(
            self: @ComponentState<TContractState>, port_id: @PortId, channel_id: @ChannelId,
        ) -> ChannelEnd {
            let channel_end = self.channel_ends.read(channel_end_key(port_id, channel_id));

            assert(!channel_end.is_zero(), ChannelErrors::MISSING_CHANNEL_END);

            channel_end
        }

        fn read_packet_commitment(
            self: @ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence,
        ) -> Commitment {
            self.packet_commitments.read(commitment_key(port_id, channel_id, sequence))
        }

        fn read_packet_receipt(
            self: @ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence,
        ) -> Receipt {
            self.packet_receipts.read(receipt_key(port_id, channel_id, sequence))
        }

        fn read_packet_ack(
            self: @ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence,
        ) -> Commitment {
            let ack = self.packet_acks.read(ack_key(port_id, channel_id, sequence));

            assert(ack.is_non_zero(), ChannelErrors::MISSING_PACKET_ACK);

            ack
        }

        fn packet_ack_exists(
            self: @ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence,
        ) -> bool {
            self.packet_acks.read(ack_key(port_id, channel_id, sequence)).is_non_zero()
        }

        fn read_next_sequence_send(
            self: @ComponentState<TContractState>, port_id: @PortId, channel_id: @ChannelId,
        ) -> Sequence {
            self.send_sequences.read(next_sequence_send_key(port_id, channel_id))
        }

        fn read_next_sequence_recv(
            self: @ComponentState<TContractState>, port_id: @PortId, channel_id: @ChannelId,
        ) -> Sequence {
            self.recv_sequences.read(next_sequence_recv_key(port_id, channel_id))
        }

        fn read_next_sequence_ack(
            self: @ComponentState<TContractState>, port_id: @PortId, channel_id: @ChannelId,
        ) -> Sequence {
            self.ack_sequences.read(next_sequence_ack_key(port_id, channel_id))
        }
    }

    #[generate_trait]
    pub(crate) impl ChannelWriterImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of ChannelWriterTrait<TContractState> {
        fn write_next_channel_sequence(
            ref self: ComponentState<TContractState>, channel_sequence: u64,
        ) {
            self.next_channel_sequence.write(channel_sequence);
        }

        fn write_channel_end(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            channel_end: ChannelEnd,
        ) {
            let key = channel_end_key(port_id, channel_id);
            let channel_end_commitment = poseidon_hash(@channel_end);
            self.channel_ends.write(key, channel_end);
            self.channel_ends_commitments.write(key, channel_end_commitment);
        }

        fn write_packet_commitment(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence,
            commitment: Commitment,
        ) {
            let key = commitment_key(port_id, channel_id, sequence);
            let commitment_commitment = poseidon_hash(@commitment);
            self.packet_commitments.write(key, commitment);
            self.packet_commitments_commitments.write(key, commitment_commitment);
        }

        fn delete_packet_commitment(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence,
        ) {
            let key = commitment_key(port_id, channel_id, sequence);
            self.packet_commitments.write(key, CommitmentZero::zero());
            self.packet_commitments_commitments.write(key, 0x0);
        }

        fn write_packet_receipt(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence,
            receipt: Receipt,
        ) {
            let key = receipt_key(port_id, channel_id, sequence);
            let receipt_commitment = poseidon_hash(@receipt);
            self.packet_receipts.write(key, receipt);
            self.packet_receipts_commitments.write(key, receipt_commitment);
        }

        fn write_packet_ack(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence,
            ack_commitment: Commitment,
        ) {
            let key = ack_key(port_id, channel_id, sequence);
            let ack_commitment_commitment = poseidon_hash(@ack_commitment);
            self.packet_acks.write(key, ack_commitment);
            self.packet_acks_commitments.write(key, ack_commitment_commitment);
        }

        fn write_next_sequence_send(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: Sequence,
        ) {
            let key = next_sequence_send_key(port_id, channel_id);
            let sequence_commitment = poseidon_hash(@sequence);
            self.send_sequences.write(key, sequence);
            self.send_sequences_commitments.write(key, sequence_commitment);
        }

        fn write_next_sequence_recv(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: Sequence,
        ) {
            let key = next_sequence_recv_key(port_id, channel_id);
            let sequence_commitment = poseidon_hash(@sequence);
            self.recv_sequences.write(key, sequence);
            self.recv_sequences_commitments.write(key, sequence_commitment);
        }

        fn write_next_sequence_ack(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: Sequence,
        ) {
            let key = next_sequence_ack_key(port_id, channel_id);
            let sequence_commitment = poseidon_hash(@sequence);
            self.ack_sequences.write(key, sequence);
            self.ack_sequences_commitments.write(key, sequence_commitment);
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
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>,
    > of EventEmitterTrait<TContractState> {
        fn emit_chan_open_init_event(
            ref self: ComponentState<TContractState>,
            port_id_on_a: PortId,
            chan_id_on_a: ChannelId,
            port_id_on_b: PortId,
            connection_id_on_a: ConnectionId,
            version_on_a: AppVersion,
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter
                .emit_chan_open_init_event(
                    port_id_on_a, chan_id_on_a, port_id_on_b, connection_id_on_a, version_on_a,
                );
        }

        fn emit_chan_open_try_event(
            ref self: ComponentState<TContractState>,
            port_id_on_b: PortId,
            chan_id_on_b: ChannelId,
            port_id_on_a: PortId,
            chan_id_on_a: ChannelId,
            connection_id_on_b: ConnectionId,
            version_on_b: AppVersion,
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter
                .emit_chan_open_try_event(
                    port_id_on_b,
                    chan_id_on_b,
                    port_id_on_a,
                    chan_id_on_a,
                    connection_id_on_b,
                    version_on_b,
                );
        }

        fn emit_chan_open_ack_event(
            ref self: ComponentState<TContractState>,
            port_id_on_a: PortId,
            chan_id_on_a: ChannelId,
            port_id_on_b: PortId,
            chan_id_on_b: ChannelId,
            conn_id_on_a: ConnectionId,
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter
                .emit_chan_open_ack_event(
                    port_id_on_a, chan_id_on_a, port_id_on_b, chan_id_on_b, conn_id_on_a,
                );
        }

        fn emit_chan_open_confirm_event(
            ref self: ComponentState<TContractState>,
            port_id_on_a: PortId,
            chan_id_on_a: ChannelId,
            conn_id_on_b: ConnectionId,
            msg: MsgChanOpenConfirm,
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter
                .emit_chan_open_confirm_event(
                    msg.port_id_on_b, msg.chan_id_on_b, port_id_on_a, chan_id_on_a, conn_id_on_b,
                );
        }

        fn emit_send_packet_event(
            ref self: ComponentState<TContractState>, packet: Packet, ordering: ChannelOrdering,
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter.emit_send_packet_event(packet, ordering);
        }

        fn emit_recv_packet_event(
            ref self: ComponentState<TContractState>, packet: Packet, ordering: ChannelOrdering,
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter.emit_recv_packet_event(packet, ordering);
        }

        fn emit_write_ack_event(
            ref self: ComponentState<TContractState>, packet: Packet, ack: Acknowledgement,
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter.emit_write_ack_event(packet, ack);
        }

        fn emit_ack_packet_event(
            ref self: ComponentState<TContractState>, packet: Packet, ordering: ChannelOrdering,
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter.emit_ack_packet_event(packet, ordering);
        }

        fn emit_timeout_packet_event(
            ref self: ComponentState<TContractState>, packet: Packet, ordering: ChannelOrdering,
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter.emit_timeout_packet_event(packet, ordering);
        }
    }
}

