#[starknet::contract]
pub(crate) mod MockClientHandler {
    use starknet_ibc_core::client::{ClientHandlerComponent, ClientEventEmitterComponent};

    component!(
        path: ClientEventEmitterComponent, storage: Client_emitter, event: ClientEventEmitterEvent
    );
    component!(path: ClientHandlerComponent, storage: client_handler, event: ClientHandlerEvent);

    impl ClientInitializerImpl = ClientHandlerComponent::ClientInitializerImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        Client_emitter: ClientEventEmitterComponent::Storage,
        #[substorage(v0)]
        client_handler: ClientHandlerComponent::Storage
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        ClientEventEmitterEvent: ClientEventEmitterComponent::Event,
        #[flat]
        ClientHandlerEvent: ClientHandlerComponent::Event
    }

    #[constructor]
    fn constructor(ref self: ContractState) {
        self.client_handler.initializer();
    }
}
