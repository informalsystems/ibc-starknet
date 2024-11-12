#[starknet::component]
pub mod ConnectionEventEmitterComponent {
    #[storage]
    pub struct Storage {}

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        ConnOpenInitEvent: ConnOpenInitEvent,
        ConnOpenTryEvent: ConnOpenTryEvent,
        ConnOpenAckEvent: ConnOpenAckEvent,
        ConnOpenConfirmEvent: ConnOpenConfirmEvent,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct ConnOpenInitEvent {}

    #[derive(Debug, Drop, starknet::Event)]
    pub struct ConnOpenTryEvent {}

    #[derive(Debug, Drop, starknet::Event)]
    pub struct ConnOpenAckEvent {}

    #[derive(Debug, Drop, starknet::Event)]
    pub struct ConnOpenConfirmEvent {}

    #[generate_trait]
    pub impl ConnectionEventEmitterImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ConnectinoEventEmitterTrait<TContractState> {}
}

