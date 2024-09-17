#[starknet::contract]
pub mod IBCCore {
    use starknet_ibc_core::channel::ChannelEventEmitterComponent;
    use starknet_ibc_core::channel::ChannelHandlerComponent;
    use starknet_ibc_core::client::ClientEventEmitterComponent;
    use starknet_ibc_core::client::ClientHandlerComponent;
    use starknet_ibc_core::router::RouterHandlerComponent;

    // -----------------------------------------------------------
    // Setup Client Components
    // -----------------------------------------------------------
    component!(
        path: ClientEventEmitterComponent, storage: client_emitter, event: ClientEventEmitterEvent
    );
    component!(path: ClientHandlerComponent, storage: client_handler, event: ClientHandlerEvent);

    #[abi(embed_v0)]
    impl CoreClientHandlerImpl =
        ClientHandlerComponent::CoreClientHandler<ContractState>;
    #[abi(embed_v0)]
    impl CoreRegisterClientImpl =
        ClientHandlerComponent::CoreRegisterClient<ContractState>;
    impl ClientInitializerImpl = ClientHandlerComponent::ClientInitializerImpl<ContractState>;

    // -----------------------------------------------------------
    // Setup Channel Components
    // -----------------------------------------------------------

    component!(
        path: ChannelEventEmitterComponent,
        storage: channel_emitter,
        event: ChannelEventEmitterEvent
    );
    component!(path: ChannelHandlerComponent, storage: channel_handler, event: ChannelHandlerEvent);

    #[abi(embed_v0)]
    impl CoreChannelHanderImpl =
        ChannelHandlerComponent::CoreChannelHandler<ContractState>;
    impl ChannelInitializerImpl = ChannelHandlerComponent::ChannelInitializerImpl<ContractState>;

    // -----------------------------------------------------------
    // Setup Router Components
    // -----------------------------------------------------------

    component!(path: RouterHandlerComponent, storage: router_handler, event: RouterHandlerEvent);

    #[abi(embed_v0)]
    impl CoreRouterHanderImpl =
        RouterHandlerComponent::CoreRouterHandler<ContractState>;
    impl RouterInitializerImpl = RouterHandlerComponent::RouterInitializerImpl<ContractState>;


    #[storage]
    struct Storage {
        #[substorage(v0)]
        client_emitter: ClientEventEmitterComponent::Storage,
        #[substorage(v0)]
        client_handler: ClientHandlerComponent::Storage,
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
        ClientEventEmitterEvent: ClientEventEmitterComponent::Event,
        #[flat]
        ClientHandlerEvent: ClientHandlerComponent::Event,
        #[flat]
        ChannelEventEmitterEvent: ChannelEventEmitterComponent::Event,
        #[flat]
        ChannelHandlerEvent: ChannelHandlerComponent::Event,
        #[flat]
        RouterHandlerEvent: RouterHandlerComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {
        self.client_handler.initializer();
        self.channel_handler.initializer();
        self.router_handler.initializer();
    }
}
