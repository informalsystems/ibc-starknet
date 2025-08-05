#[starknet::contract]
pub mod ProtobufLib {
    use starknet_ibc_libs::protobuf::ProtobufLibComponent;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        comet_lib: ProtobufLibComponent::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        ProtobufLibEvent: ProtobufLibComponent::Event,
    }

    component!(path: ProtobufLibComponent, storage: comet_lib, event: ProtobufLibEvent);

    #[abi(embed_v0)]
    impl ProtobufLibImpl = ProtobufLibComponent::ProtobufLib<ContractState>;
    // note: there is no initializer because it is supposed to be used via library calls.
}
