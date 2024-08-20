#[starknet::contract]
pub(crate) mod IBC {
    use starknet_ibc::core::client::ICS02ClientComponent;

    component!(path: ICS02ClientComponent, storage: client, event: ICS02ClientEvent);

    impl ICS02ClientInitializerImpl = ICS02ClientComponent::ClientInitializerImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        client: ICS02ClientComponent::Storage,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        ICS02ClientEvent: ICS02ClientComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {
        self.client.initializer();
    }
}
