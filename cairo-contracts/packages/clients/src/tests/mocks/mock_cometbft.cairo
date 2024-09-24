#[starknet::contract]
pub(crate) mod MockCometClient {
    use starknet_ibc_clients::cometbft::CometClientComponent;

    component!(path: CometClientComponent, storage: client, event: CometClientEvent);

    #[abi(embed_v0)]
    impl CometClientHandlerImpl =
        CometClientComponent::CometClientHandler<ContractState>;
    #[abi(embed_v0)]
    impl CometClientQueryImpl =
        CometClientComponent::CometClientQuery<ContractState>;

    #[abi(embed_v0)]
    impl CometClientValidationImpl =
        CometClientComponent::CometClientValidation<ContractState>;

    #[abi(embed_v0)]
    impl CometClientExecutionImpl =
        CometClientComponent::CometClientExecution<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        client: CometClientComponent::Storage,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        CometClientEvent: CometClientComponent::Event,
    }
}
