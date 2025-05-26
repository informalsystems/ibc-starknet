#[starknet::contract]
pub mod CometBftFactCheckerContract {
    use starknet_ibc_utils::comet::CometBftFactCheckerComponent;

    component!(
        path: CometBftFactCheckerComponent,
        storage: comet_fact_checker,
        event: CometFactCheckerEvent,
    );

    #[abi(embed_v0)]
    impl CometBftFactCheckerQueryImpl =
        CometBftFactCheckerComponent::CometBftFactCheckerQuery<ContractState>;

    #[abi(embed_v0)]
    impl CometBftFactCheckerStoreImpl =
        CometBftFactCheckerComponent::CometBftFactCheckerStore<ContractState>;


    #[storage]
    struct Storage {
        #[substorage(v0)]
        comet_fact_checker: CometBftFactCheckerComponent::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        CometFactCheckerEvent: CometBftFactCheckerComponent::Event,
    }
    // no constructor
}
