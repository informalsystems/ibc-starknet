#[starknet::contract]
pub mod CometClient {
    use core::num::traits::Zero;
    use openzeppelin_access::ownable::OwnableComponent;
    use starknet::ContractAddress;
    use starknet_ibc_clients::cometbft::{CometClientComponent, CometErrors};
    use starknet_ibc_utils::governance::IBCGovernanceComponent;

    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    component!(path: IBCGovernanceComponent, storage: governance, event: IBCGovernanceEvent);
    component!(path: CometClientComponent, storage: client, event: CometClientEvent);

    #[abi(embed_v0)]
    impl OwnableMixinImpl = OwnableComponent::OwnableMixinImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;

    #[abi(embed_v0)]
    impl IBCGovernanceImpl = IBCGovernanceComponent::Governance<ContractState>;
    impl IBCGovernanceInternalImpl = IBCGovernanceComponent::GovernanceInternalImpl<ContractState>;

    #[abi(embed_v0)]
    impl CometClientHandlerImpl =
        CometClientComponent::CometClientHandler<ContractState>;
    #[abi(embed_v0)]
    impl CometClientQueryImpl =
        CometClientComponent::CometClientQuery<ContractState>;

    // NOTE: The client state validation interface is exposed for public use.
    // However, only the IBC core contract (owner) can invoke the execution methods.

    #[abi(embed_v0)]
    impl CometClientValidationImpl =
        CometClientComponent::CometClientValidation<ContractState>;
    impl CometClientExecutionImpl = CometClientComponent::CometClientExecution<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
        #[substorage(v0)]
        governance: IBCGovernanceComponent::Storage,
        #[substorage(v0)]
        client: CometClientComponent::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        OwnableEvent: OwnableComponent::Event,
        #[flat]
        IBCGovernanceEvent: IBCGovernanceComponent::Event,
        #[flat]
        CometClientEvent: CometClientComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress) {
        assert(owner.is_non_zero(), CometErrors::ZERO_OWNER);
        self.ownable.initializer(owner);
        self.governance.initializer();
    }
}
