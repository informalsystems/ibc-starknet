#[starknet::contract]
pub mod MockCometClient {
    use core::num::traits::Zero;
    use openzeppelin_access::ownable::OwnableComponent;
    use starknet::ContractAddress;
    use starknet_ibc_clients::mock::{MockClientComponent, MockErrors};

    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    component!(path: MockClientComponent, storage: client, event: MockClientEvent);

    #[abi(embed_v0)]
    impl OwnableMixinImpl = OwnableComponent::OwnableMixinImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;

    #[abi(embed_v0)]
    impl MockClientHandlerImpl =
        MockClientComponent::MockClientHandler<ContractState>;
    #[abi(embed_v0)]
    impl MockClientQueryImpl = MockClientComponent::MockClientQuery<ContractState>;

    #[abi(embed_v0)]
    impl MockClientValidationImpl =
        MockClientComponent::MockClientValidation<ContractState>;

    #[abi(embed_v0)]
    impl MockClientExecutionImpl =
        MockClientComponent::MockClientExecution<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
        #[substorage(v0)]
        client: MockClientComponent::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        OwnableEvent: OwnableComponent::Event,
        #[flat]
        MockClientEvent: MockClientComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress) {
        self.ownable.initializer(owner);
    }
}
