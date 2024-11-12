#[starknet::component]
pub mod ConnectionHandlerComponent {
    use ClientHandlerComponent::ClientInternalTrait;
    use ConnectionEventEmitterComponent::ConnectionEventEmitterTrait;
    use starknet::storage::StoragePathEntry;
    use starknet::storage::{
        StorageMapReadAccess, StorageMapWriteAccess, StoragePointerReadAccess,
        StoragePointerWriteAccess,
    };
    use starknet::storage::{Map, Vec, VecTrait, MutableVecTrait};
    use starknet_ibc_core::client::{ClientHandlerComponent, ClientContract, ClientContractTrait,};
    use starknet_ibc_core::connection::{
        ConnectionEventEmitterComponent, IConnectionHandler, IConnectionQuery, MsgConnOpenAck,
        MsgConnOpenConfirm, MsgConnOpenInit, MsgConnOpenInitTrait, MsgConnOpenTry, ConnectionEnd,
        ConnectionEndTrait, ConnectionErrors
    };
    use starknet_ibc_core::host::{
        ClientId, ConnectionId, ConnectionIdImpl, client_connection_key, connection_end_key
    };
    use starknet_ibc_utils::ValidateBasic;

    #[storage]
    pub struct Storage {
        pub next_connection_sequence: u64,
        pub client_to_connections: Map<felt252, Vec<ConnectionId>>,
        pub connection_ends: Map<felt252, ConnectionEnd>
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {}

    // -----------------------------------------------------------
    // IConnectionHandler
    // -----------------------------------------------------------

    #[embeddable_as(CoreConnectionHandler)]
    pub impl CoreConnectionHandlerImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ConnectionEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>
    > of IConnectionHandler<ComponentState<TContractState>> {
        fn conn_open_init(ref self: ComponentState<TContractState>, msg: MsgConnOpenInit) {
            let connection_sequence = self.read_next_connection_sequence();
            self.conn_open_init_validate(connection_sequence, msg.clone());
            self.conn_open_init_execute(connection_sequence, msg);
        }

        fn conn_open_try(ref self: ComponentState<TContractState>, msg: MsgConnOpenTry) {}

        fn conn_open_ack(ref self: ComponentState<TContractState>, msg: MsgConnOpenAck) {}

        fn conn_open_confirm(ref self: ComponentState<TContractState>, msg: MsgConnOpenConfirm) {}
    }

    // -----------------------------------------------------------
    // IConnectionQuery
    // -----------------------------------------------------------

    #[embeddable_as(CoreConnectionQuery)]
    impl CoreConnectionQueryImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of IConnectionQuery<ComponentState<TContractState>> {
        fn connection_end(
            ref self: ComponentState<TContractState>, connection_id: ConnectionId
        ) -> ConnectionEnd {
            self.read_connection_end(connection_id)
        }
    }

    // -----------------------------------------------------------
    // Connection handler implementations
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl ConnOpenInitImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ConnectionEventEmitterComponent::HasComponent<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
    > of ChanOpenInitTrait<TContractState> {
        fn conn_open_init_validate(
            self: @ComponentState<TContractState>, connection_sequence: u64, msg: MsgConnOpenInit
        ) {
            msg.validate_basic();

            let client = self.get_client(*msg.client_type_on_a());

            let client_sequence = msg.client_id_on_a.sequence;

            client.verify_is_active(client_sequence);
        }

        fn conn_open_init_execute(
            ref self: ComponentState<TContractState>, connection_sequence: u64, msg: MsgConnOpenInit
        ) {
            let conn_end_on_a = ConnectionEndTrait::init(
                msg.client_id_on_a.clone(),
                msg.counterparty_client_id().clone(),
                msg.counterparty_prefix().clone(),
                msg.delay_period
            );

            let conn_id_on_a = ConnectionIdImpl::new(connection_sequence);

            self.write_connection_end(conn_id_on_a.clone(), conn_end_on_a);

            self.write_client_to_connections(msg.client_id_on_a.clone(), conn_id_on_a.clone());

            self.write_next_connection_sequence(connection_sequence + 1);

            self
                .emit_conn_open_init_event(
                    msg.client_id_on_a.clone(),
                    conn_id_on_a.clone(),
                    msg.counterparty_client_id().clone(),
                    msg.counterparty_connection_id().clone()
                );
        }
    }

    // -----------------------------------------------------------
    // Connection Internal
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl ConnectionInternalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ConnectionInternalTrait<TContractState> {}

    // -----------------------------------------------------------
    // Connection access
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl ConnectionAccessImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl ClientHandler: ClientHandlerComponent::HasComponent<TContractState>,
    > of ConnectionAccessTrait<TContractState> {
        fn get_client(
            self: @ComponentState<TContractState>, client_type: felt252
        ) -> ClientContract {
            let client_comp = get_dep_component!(self, ClientHandler);

            client_comp.get_client(client_type)
        }
    }

    // -----------------------------------------------------------
    // Connection Reader/Writer
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl ConnectionReaderImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ConnectionReaderTrait<TContractState> {
        fn read_next_connection_sequence(self: @ComponentState<TContractState>) -> u64 {
            self.next_connection_sequence.read()
        }

        fn read_client_to_connections(
            self: @ComponentState<TContractState>, client_id: ClientId
        ) -> Array<ConnectionId> {
            let mut conn_ids: Array<ConnectionId> = ArrayTrait::new();

            let entry = self.client_to_connections.entry(client_connection_key(@client_id));

            let mut i = 0;

            while i < entry.len() {
                conn_ids.append(entry.at(i).read());
                i += 1;
            };

            conn_ids
        }

        fn read_connection_end(
            self: @ComponentState<TContractState>, connection_id: ConnectionId
        ) -> ConnectionEnd {
            let connection_end = self.connection_ends.read(connection_end_key(@connection_id));

            assert(!connection_end.is_zero(), ConnectionErrors::MISSING_CONNECTION_END);

            connection_end
        }
    }

    #[generate_trait]
    pub(crate) impl ConnectionWriterImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ConnectionWriterTrait<TContractState> {
        fn write_next_connection_sequence(
            ref self: ComponentState<TContractState>, connection_sequence: u64
        ) {
            self.next_connection_sequence.write(connection_sequence)
        }

        fn write_client_to_connections(
            ref self: ComponentState<TContractState>,
            client_id: ClientId,
            connection_id: ConnectionId
        ) {
            self
                .client_to_connections
                .entry(client_connection_key(@client_id))
                .append()
                .write(connection_id);
        }

        fn write_connection_end(
            ref self: ComponentState<TContractState>,
            connection_id: ConnectionId,
            connection_end: ConnectionEnd
        ) {
            self.connection_ends.write(connection_end_key(@connection_id), connection_end)
        }
    }

    // -----------------------------------------------------------
    // Connection Event Emitter
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl EventEmitterImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ConnectionEventEmitterComponent::HasComponent<TContractState>
    > of EventEmitterTrait<TContractState> {
        fn emit_conn_open_init_event(
            ref self: ComponentState<TContractState>,
            client_id_on_a: ClientId,
            connection_id_on_a: ConnectionId,
            client_id_on_b: ClientId,
            connection_id_on_b: ConnectionId,
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter
                .emit_conn_open_init_event(
                    client_id_on_a, connection_id_on_a, client_id_on_b, connection_id_on_b
                );
        }
    }
}

