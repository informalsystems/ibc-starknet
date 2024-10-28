#[starknet::contract]
pub mod MockRouterHandler {
    use starknet_ibc_core::router::{RouterHandlerComponent};

    component!(path: RouterHandlerComponent, storage: router_handler, event: RouterHandlerEvent);

    impl RouterInitializerImpl = RouterHandlerComponent::RouterInitializerImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        router_handler: RouterHandlerComponent::Storage
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        RouterHandlerEvent: RouterHandlerComponent::Event
    }

    #[constructor]
    fn constructor(ref self: ContractState) {
        self.router_handler.initializer();
    }
}
