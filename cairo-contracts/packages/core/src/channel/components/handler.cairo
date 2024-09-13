#[starknet::component]
pub mod ChannelHandlerComponent {
    use ClientHandlerComponent::ClientInternalTrait;
    use starknet::ContractAddress;
    use starknet::storage::Map;
    use starknet::storage::StoragePathEntry;
    use starknet::{get_block_timestamp, get_block_number};
    use starknet_ibc_core::channel::{
        ChannelEventEmitterComponent, IChannelHandler, MsgRecvPacket, MsgRecvPacketTrait,
        ChannelEnd, ChannelEndTrait, ChannelErrors, PacketTrait, ApplicationContract,
        ApplicationContractTrait, ChannelOrdering, Receipt
    };
    use starknet_ibc_core::client::{ClientHandlerComponent, ClientContractTrait, StatusTrait};
    use starknet_ibc_core::host::{
        PortId, PortIdTrait, ChannelId, ChannelIdTrait, Sequence, channel_end_key, receipt_key,
        commitment_path
    };
    use starknet_ibc_core::router::{RouterHandlerComponent, IRouter};
    use starknet_ibc_utils::{ValidateBasicTrait, ComputeKeyTrait};

    #[storage]
    struct Storage {
        pub channel_ends: Map<felt252, ChannelEnd>,
        pub packet_receipts: Map<felt252, Receipt>,
        pub packet_acks: Map<felt252, ByteArray>,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {}

    #[generate_trait]
    pub impl ChannelInitializerImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ChannelInitializerTrait<TContractState> {
        fn initializer(ref self: ComponentState<TContractState>) {}
    }

    #[embeddable_as(CoreChannelHandler)]
    pub impl CoreChannelHandlerImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
        impl RouterHandler: RouterHandlerComponent::HasComponent<TContractState>
    > of IChannelHandler<ComponentState<TContractState>> {
        fn recv_packet(ref self: ComponentState<TContractState>, msg: MsgRecvPacket) {
            let chan_end_on_b = self
                .read_channel_end(@msg.packet.port_id_on_b, @msg.packet.chan_id_on_b);

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

            assert(chan_end_on_b.is_open(), ChannelErrors::INVALID_CHANNEL_STATE);

            assert(
                chan_end_on_b
                    .counterparty_matches(@msg.packet.port_id_on_a, @msg.packet.chan_id_on_a),
                ChannelErrors::INVALID_COUNTERPARTY
            );

            // TODO: verify connection end if we ever decide to implement ICS-03

            let host_height = get_block_number();

            let host_timestamp = get_block_timestamp();

            msg.packet.check_timed_out(@host_height, @host_timestamp);

            let client_comp = get_dep_component!(self, ClientHandler);

            let client = client_comp.get_client(chan_end_on_b.client_id.client_type);

            let client_status = client.status(chan_end_on_b.client_id.sequence);

            assert(client_status.is_active(), ChannelErrors::INACTIVE_CLIENT);

            let client_latest_height = client.latest_height(chan_end_on_b.client_id.sequence);

            msg.verify_proof_height(@client_latest_height);

            let packet_commitment_on_a = msg.packet.compute_packet_commitment();

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
                    msg.proof_commitment_on_a
                );

            match chan_end_on_b.ordering {
                ChannelOrdering::Unordered => {
                    let _receipt = self
                        .read_packet_receipt(
                            @msg.packet.port_id_on_a, @msg.packet.chan_id_on_a, @msg.packet.seq_on_a
                        );
                },
                ChannelOrdering::Ordered => {}
            };
        }

        fn recv_packet_execute(
            ref self: ComponentState<TContractState>, msg: MsgRecvPacket, chan_end_on_b: ChannelEnd
        ) {
            // Check if another relayer already relayed the packet.
            match chan_end_on_b.ordering {
                ChannelOrdering::Unordered => {},
                ChannelOrdering::Ordered => { return; }
            };

            let router_comp = get_dep_component!(@self, RouterHandler);

            let maybe_app_address = router_comp
                .get_app_address(msg.packet.port_id_on_b.compute_key());

            assert(maybe_app_address.is_some(), ChannelErrors::UNSUPPORTED_PORT_ID);

            let app: ApplicationContract = maybe_app_address.unwrap().into();

            let _ack = app.on_recv_packet(msg.packet);
        }
    }

    #[generate_trait]
    pub(crate) impl ChannelReaderImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ChannelReaderTrait<TContractState> {
        fn read_channel_end(
            self: @ComponentState<TContractState>, port_id: @PortId, channel_id: @ChannelId
        ) -> ChannelEnd {
            self.channel_ends.read(channel_end_key(port_id, channel_id))
        }

        fn read_packet_receipt(
            self: @ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence
        ) -> Receipt {
            self.packet_receipts.read(receipt_key(port_id, channel_id, sequence))
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

        fn write_packet_receipt(
            ref self: ComponentState<TContractState>,
            port_id: @PortId,
            channel_id: @ChannelId,
            sequence: @Sequence,
            receipt: Receipt
        ) {
            self.packet_receipts.write(receipt_key(port_id, channel_id, sequence), receipt);
        }
    }

    #[generate_trait]
    pub(crate) impl EventEmitterImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ChannelEventEmitterComponent::HasComponent<TContractState>
    > of EventEmitterTrait<TContractState> {}
}
