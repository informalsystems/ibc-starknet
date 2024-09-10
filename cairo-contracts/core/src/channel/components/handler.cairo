#[starknet::component]
pub mod ChannelHandlerComponent {
    use ClientHandlerComponent::ClientInternalTrait;
    use starknet::ContractAddress;
    use starknet::storage::Map;
    use starknet::storage::StoragePathEntry;
    use starknet::{get_block_timestamp, get_block_number};
    use starknet_ibc_core::channel::{
        ChannelEventEmitterComponent, IChannelHandler, MsgRecvPacket, MsgRecvPacketTrait,
        ChannelEnd, ChannelEndTrait, ChannelErrors, PacketTrait
    };
    use starknet_ibc_core::client::{ClientHandlerComponent, ClientContractTrait, StatusTrait};
    use starknet_ibc_core::host::{PortId, ChannelId, ChannelIdTrait};
    use starknet_ibc_core::router::{RouterHandlerComponent, IRouter};
    use starknet_ibc_utils::ValidateBasicTrait;

    #[storage]
    struct Storage {
        pub channel_ends: Map<(felt252, u64), ChannelEnd>,
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
            self.recv_packet_validate(msg.clone());
            self.recv_packet_execute(msg);
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
        fn recv_packet_validate(self: @ComponentState<TContractState>, msg: MsgRecvPacket) {
            msg.validate_basic();

            let chan_end_on_b = self
                .read_channel_end(msg.packet.port_id_on_b.clone(), msg.packet.chan_id_on_b.clone());

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

            client
                .verify_membership(
                    chan_end_on_b.client_id.sequence,
                    'commitment path', // TODO: implement commitment path
                    packet_commitment_on_a,
                    msg.proof_commitment_on_a
                );
        }
        fn recv_packet_execute(ref self: ComponentState<TContractState>, msg: MsgRecvPacket) {
            let router_comp = get_dep_component!(@self, RouterHandler);

            let _app_address = router_comp.get_app_address('transfer');
        }
    }

    #[generate_trait]
    pub(crate) impl ChannelReaderImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ChannelReaderTrait<TContractState> {
        fn read_channel_end(
            self: @ComponentState<TContractState>, port_id: PortId, channel_id: ChannelId
        ) -> ChannelEnd {
            self.channel_ends.read(('0', channel_id.sequence()))
        }
    }

    #[generate_trait]
    pub(crate) impl ChannelWriterImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ChannelWriterTrait<TContractState> {
        fn write_channel_end(
            ref self: ComponentState<TContractState>,
            port_id: PortId,
            channel_id: ChannelId,
            channel_end: ChannelEnd
        ) {
            self.channel_ends.write(('0', channel_id.sequence()), channel_end);
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

