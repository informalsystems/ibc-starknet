#[starknet::contract]
pub mod IBCCore {
    use ibc_utils::storage::write_raw_key;
    use starknet::ClassHash;
    use starknet_ibc_core::channel::{ChannelEventEmitterComponent, ChannelHandlerComponent};
    use starknet_ibc_core::client::{ClientEventEmitterComponent, ClientHandlerComponent};
    use starknet_ibc_core::connection::{
        ConnectionEventEmitterComponent, ConnectionHandlerComponent,
    };
    use starknet_ibc_core::router::RouterHandlerComponent;
    use starknet_ibc_utils::governance::IBCGovernanceComponent;

    // -----------------------------------------------------------
    // Setup Governance Component
    // -----------------------------------------------------------

    component!(path: IBCGovernanceComponent, storage: governance, event: IBCGovernanceEvent);

    #[abi(embed_v0)]
    impl IBCGovernanceImpl = IBCGovernanceComponent::Governance<ContractState>;
    impl IBCGovernanceInternalImpl = IBCGovernanceComponent::GovernanceInternalImpl<ContractState>;

    // -----------------------------------------------------------
    // Setup Client Components
    // -----------------------------------------------------------

    component!(
        path: ClientEventEmitterComponent, storage: client_emitter, event: ClientEventEmitterEvent,
    );
    component!(path: ClientHandlerComponent, storage: client_handler, event: ClientHandlerEvent);

    #[abi(embed_v0)]
    impl CoreClientHandlerImpl =
        ClientHandlerComponent::CoreClientHandler<ContractState>;
    #[abi(embed_v0)]
    impl CoreRegisterClientImpl =
        ClientHandlerComponent::CoreRegisterClient<ContractState>;
    #[abi(embed_v0)]
    impl CoreRegisterRelayerImpl =
        ClientHandlerComponent::CoreRegisterRelayer<ContractState>;
    impl ClientInitializerImpl = ClientHandlerComponent::ClientInitializerImpl<ContractState>;

    // -----------------------------------------------------------
    // Setup Connection Components
    // -----------------------------------------------------------

    component!(
        path: ConnectionEventEmitterComponent,
        storage: connection_emitter,
        event: ConnectionEventEmitterEvent,
    );
    component!(
        path: ConnectionHandlerComponent,
        storage: connection_handler,
        event: ConnectionHandlerEvent,
    );

    #[abi(embed_v0)]
    impl CoreConnectionHandlerImpl =
        ConnectionHandlerComponent::CoreConnectionHandler<ContractState>;
    #[abi(embed_v0)]
    impl CoreConnectionQueryImpl =
        ConnectionHandlerComponent::CoreConnectionQuery<ContractState>;

    // -----------------------------------------------------------
    // Setup Channel Components
    // -----------------------------------------------------------

    component!(
        path: ChannelEventEmitterComponent,
        storage: channel_emitter,
        event: ChannelEventEmitterEvent,
    );
    component!(path: ChannelHandlerComponent, storage: channel_handler, event: ChannelHandlerEvent);

    #[abi(embed_v0)]
    impl CoreChannelHandlerImpl =
        ChannelHandlerComponent::CoreChannelHandler<ContractState>;
    #[abi(embed_v0)]
    impl CoreChannelQueryImpl =
        ChannelHandlerComponent::CoreChannelQuery<ContractState>;

    // -----------------------------------------------------------
    // Setup Router Components
    // -----------------------------------------------------------

    component!(path: RouterHandlerComponent, storage: router_handler, event: RouterHandlerEvent);

    #[abi(embed_v0)]
    impl CoreRouterHandlerImpl =
        RouterHandlerComponent::CoreRouterHandler<ContractState>;
    impl RouterInitializerImpl = RouterHandlerComponent::RouterInitializerImpl<ContractState>;


    #[storage]
    struct Storage {
        #[substorage(v0)]
        governance: IBCGovernanceComponent::Storage,
        #[substorage(v0)]
        client_emitter: ClientEventEmitterComponent::Storage,
        #[substorage(v0)]
        client_handler: ClientHandlerComponent::Storage,
        #[substorage(v0)]
        connection_emitter: ConnectionEventEmitterComponent::Storage,
        #[substorage(v0)]
        connection_handler: ConnectionHandlerComponent::Storage,
        #[substorage(v0)]
        channel_emitter: ChannelEventEmitterComponent::Storage,
        #[substorage(v0)]
        channel_handler: ChannelHandlerComponent::Storage,
        #[substorage(v0)]
        router_handler: RouterHandlerComponent::Storage,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        IBCGovernanceEvent: IBCGovernanceComponent::Event,
        #[flat]
        ClientEventEmitterEvent: ClientEventEmitterComponent::Event,
        #[flat]
        ClientHandlerEvent: ClientHandlerComponent::Event,
        #[flat]
        ConnectionEventEmitterEvent: ConnectionEventEmitterComponent::Event,
        #[flat]
        ConnectionHandlerEvent: ConnectionHandlerComponent::Event,
        #[flat]
        ChannelEventEmitterEvent: ChannelEventEmitterComponent::Event,
        #[flat]
        ChannelHandlerEvent: ChannelHandlerComponent::Event,
        #[flat]
        RouterHandlerEvent: RouterHandlerComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState, protobuf_lib: ClassHash) {
        self.governance.initializer();
        self.client_handler.initializer();
        self.router_handler.initializer();

        // store the library classes
        // not using storage keys, as these keys are read without contract context.
        write_raw_key::<'protobuf-library'>(protobuf_lib);
    }
}
