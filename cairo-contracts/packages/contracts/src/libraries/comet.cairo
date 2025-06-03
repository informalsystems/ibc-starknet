#[starknet::contract]
pub mod CometLib {
    use starknet_ibc_libs::comet::CometLibComponent;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        comet_lib: CometLibComponent::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        CometLibEvent: CometLibComponent::Event,
    }

    component!(path: CometLibComponent, storage: comet_lib, event: CometLibEvent);

    #[abi(embed_v0)]
    impl CometLibImpl = CometLibComponent::CometLib<ContractState>;
}
