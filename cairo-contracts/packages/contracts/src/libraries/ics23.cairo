#[starknet::contract]
pub mod Ics23Lib {
    use starknet_ibc_libs::ics23::Ics23LibComponent;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        comet_lib: Ics23LibComponent::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        Ics23LibEvent: Ics23LibComponent::Event,
    }

    component!(path: Ics23LibComponent, storage: comet_lib, event: Ics23LibEvent);

    #[abi(embed_v0)]
    impl Ics23LibImpl = Ics23LibComponent::Ics23Lib<ContractState>;
}
