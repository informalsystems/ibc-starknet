#[starknet::component]
pub mod ConnectionHandlerComponent {
    use starknet_ibc_core::connection::{
        ConnectionEventEmitterComponent, IConnectionHandler, IConnectionQuery, MsgConnOpenAck,
        MsgConnOpenConfirm, MsgConnOpenInit, MsgConnOpenTry
    };

    #[storage]
    pub struct Storage {}

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
        impl EventEmitter: ConnectionEventEmitterComponent::HasComponent<TContractState>
    > of IConnectionHandler<ComponentState<TContractState>> {
        fn conn_open_init(ref self: ComponentState<TContractState>, msg: MsgConnOpenInit) {}

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
    > of IConnectionQuery<ComponentState<TContractState>> {}

    // -----------------------------------------------------------
    // Connection Internal
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl ConnectionInternalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ConnectionInternalTrait<TContractState> {}

    // -----------------------------------------------------------
    // Connection Reader/Writer
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl ConnectionReaderImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ConnectionReaderTrait<TContractState> {}

    #[generate_trait]
    pub(crate) impl ConnectionWriterImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ConnectionWriterTrait<TContractState> {}

    // -----------------------------------------------------------
    // Connection Event Emitter
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl EventEmitterImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ConnectionEventEmitterComponent::HasComponent<TContractState>
    > of EventEmitterTrait<TContractState> {}
}

