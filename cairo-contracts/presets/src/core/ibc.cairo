#[starknet::contract]
pub mod IBC {
    use starknet_ibc_core_client::ClientHandlerComponent;

    component!(path: ClientHandlerComponent, storage: client, event: ClientHandlerEvent);

    impl ClientInitializerImpl = ClientHandlerComponent::ClientInitializerImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        client: ClientHandlerComponent::Storage,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        ClientHandlerEvent: ClientHandlerComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {
        self.client.initializer();
    }
}
