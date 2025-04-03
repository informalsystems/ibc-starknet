#[starknet::contract]
pub mod MockClient {
    use core::num::traits::Zero;
    use openzeppelin_access::ownable::OwnableComponent;
    use starknet::ContractAddress;
    use starknet_ibc_clients::mock::{MockClientComponent, MockErrors};
    use starknet_ibc_utils::governance::IBCGovernanceComponent;

    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    component!(path: IBCGovernanceComponent, storage: governance, event: IBCGovernanceEvent);
    component!(path: MockClientComponent, storage: client, event: MockClientEvent);

    #[abi(embed_v0)]
    impl OwnableMixinImpl = OwnableComponent::OwnableMixinImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;

    #[abi(embed_v0)]
    impl IBCGovernanceImpl = IBCGovernanceComponent::Governance<ContractState>;
    impl IBCGovernanceInternalImpl = IBCGovernanceComponent::GovernanceInternalImpl<ContractState>;

    #[abi(embed_v0)]
    impl MockClientHandlerImpl =
        MockClientComponent::MockClientHandler<ContractState>;
    #[abi(embed_v0)]
    impl MockClientQueryImpl = MockClientComponent::MockClientQuery<ContractState>;

    // NOTE: The client state validation interface is exposed for public use.
    // However, only the IBC core contract (owner) can invoke the execution methods.

    #[abi(embed_v0)]
    impl MockClientValidationImpl =
        MockClientComponent::MockClientValidation<ContractState>;
    impl MockClientExecutionImpl = MockClientComponent::MockClientExecution<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
        #[substorage(v0)]
        governance: IBCGovernanceComponent::Storage,
        #[substorage(v0)]
        client: MockClientComponent::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        OwnableEvent: OwnableComponent::Event,
        #[flat]
        IBCGovernanceEvent: IBCGovernanceComponent::Event,
        #[flat]
        MockClientEvent: MockClientComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress) {
        assert(owner.is_non_zero(), MockErrors::ZERO_OWNER);
        self.ownable.initializer(owner);
        self.governance.initializer();
    }
}
