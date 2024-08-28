#[starknet::contract]
pub(crate) mod CometClient {
    use starknet_ibc_client_cometbft::CometClientComponent;
    use starknet_ibc_utils::governance::IBCGovernanceComponent;

    component!(path: IBCGovernanceComponent, storage: governance, event: IBCGovernanceEvent);
    component!(path: CometClientComponent, storage: client, event: CometClientEvent);

    #[abi(embed_v0)]
    impl IBCGovernanceImpl = IBCGovernanceComponent::Governance<ContractState>;
    impl IBCGovernanceInternalImpl = IBCGovernanceComponent::GovernanceInternalImpl<ContractState>;

    #[abi(embed_v0)]
    impl CometClientHandlerImpl =
        CometClientComponent::CometClientHandler<ContractState>;
    #[abi(embed_v0)]
    impl CometCommonClientStateImpl =
        CometClientComponent::CometCommonClientState<ContractState>;

    // NOTE: The client state validation interface is exposed for public use.
    // However, only the handler contract can invoke the execution methods.

    #[abi(embed_v0)]
    impl CometClientValidationImpl =
        CometClientComponent::CometClientValidation<ContractState>;
    impl CometClientExecutionImpl = CometClientComponent::CometClientExecution<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        governance: IBCGovernanceComponent::Storage,
        #[substorage(v0)]
        client: CometClientComponent::Storage,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        IBCGovernanceEvent: IBCGovernanceComponent::Event,
        #[flat]
        CometClientEvent: CometClientComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {
        self.governance.initializer();
    }
}
