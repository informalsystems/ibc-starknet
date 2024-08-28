#[starknet::contract]
pub(crate) mod TendermintClient {
    use starknet_ibc_client_tendermint::TendermintClientComponent;
    use starknet_ibc_utils::governance::IBCGovernanceComponent;

    component!(path: IBCGovernanceComponent, storage: governance, event: IBCGovernanceEvent);
    component!(path: TendermintClientComponent, storage: client, event: TendermintClientEvent);

    #[abi(embed_v0)]
    impl IBCGovernanceImpl = IBCGovernanceComponent::Governance<ContractState>;
    impl IBCGovernanceInternalImpl = IBCGovernanceComponent::GovernanceInternalImpl<ContractState>;

    #[abi(embed_v0)]
    impl TendermintClientHandlerImpl =
        TendermintClientComponent::TendermintClientHandler<ContractState>;
    #[abi(embed_v0)]
    impl TendermintCommonClientStateImpl =
        TendermintClientComponent::TendermintCommonClientState<ContractState>;
    #[abi(embed_v0)]
    impl TendermintClientValidationImpl =
        TendermintClientComponent::TendermintClientValidation<ContractState>;
    impl TendermintClientExecutionImpl =
        TendermintClientComponent::TendermintClientExecution<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        governance: IBCGovernanceComponent::Storage,
        #[substorage(v0)]
        client: TendermintClientComponent::Storage,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        IBCGovernanceEvent: IBCGovernanceComponent::Event,
        #[flat]
        TendermintClientEvent: TendermintClientComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {
        self.governance.initializer();
    }
}
