#[starknet::component]
pub mod ConnectionEventEmitterComponent {
    use starknet_ibc_core::host::{ClientId, ConnectionId};

    #[storage]
    pub struct Storage {}

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        ConnOpenInitEvent: ConnOpenInitEvent,
        ConnOpenTryEvent: ConnOpenTryEvent,
        ConnOpenAckEvent: ConnOpenAckEvent,
        ConnOpenConfirmEvent: ConnOpenConfirmEvent,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct ConnOpenInitEvent {
        #[key]
        pub client_id_on_a: ClientId,
        #[key]
        pub connection_id_on_a: ConnectionId,
        #[key]
        pub client_id_on_b: ClientId,
        #[key]
        pub connection_id_on_b: ConnectionId,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct ConnOpenTryEvent {}

    #[derive(Debug, Drop, starknet::Event)]
    pub struct ConnOpenAckEvent {}

    #[derive(Debug, Drop, starknet::Event)]
    pub struct ConnOpenConfirmEvent {}

    #[generate_trait]
    pub impl ConnectionEventEmitterImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ConnectionEventEmitterTrait<TContractState> {
        fn emit_conn_open_init_event(
            ref self: ComponentState<TContractState>,
            client_id_on_a: ClientId,
            connection_id_on_a: ConnectionId,
            client_id_on_b: ClientId,
            connection_id_on_b: ConnectionId,
        ) {
            self
                .emit(
                    ConnOpenInitEvent {
                        client_id_on_a, connection_id_on_a, client_id_on_b, connection_id_on_b,
                    }
                );
        }
    }
}

