#[starknet::contract]
pub mod IBC {
    use starknet_ibc_core::client::ClientEventEmitterComponent;
    use starknet_ibc_core::client::ClientHandlerComponent;

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

    #[storage]
    struct Storage {
        #[substorage(v0)]
        client_emitter: ClientEventEmitterComponent::Storage,
        #[substorage(v0)]
        client_handler: ClientHandlerComponent::Storage,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        ClientEventEmitterEvent: ClientEventEmitterComponent::Event,
        #[flat]
        ClientHandlerEvent: ClientHandlerComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {
        self.client_handler.initializer();
    }
}
