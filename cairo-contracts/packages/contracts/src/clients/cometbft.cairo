#[starknet::contract]
pub mod CometClient {
    use core::num::traits::Zero;
    use ibc_utils::storage::write_raw_key;
    use openzeppelin_access::ownable::OwnableComponent;
    use openzeppelin_upgrades::UpgradeableComponent;
    use openzeppelin_upgrades::interface::IUpgradeable;
    use starknet::{ClassHash, ContractAddress};
    use starknet_ibc_clients::cometbft::{CometClientComponent, CometErrors};

    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    component!(path: UpgradeableComponent, storage: upgradeable, event: UpgradeableEvent);
    component!(path: CometClientComponent, storage: client, event: CometClientEvent);

    #[abi(embed_v0)]
    impl OwnableMixinImpl = OwnableComponent::OwnableMixinImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;

    impl UpgradeableInternalImpl = UpgradeableComponent::InternalImpl<ContractState>;

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
        upgradeable: UpgradeableComponent::Storage,
        #[substorage(v0)]
        client: CometClientComponent::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        OwnableEvent: OwnableComponent::Event,
        #[flat]
        UpgradeableEvent: UpgradeableComponent::Event,
        #[flat]
        CometClientEvent: CometClientComponent::Event,
    }

    #[constructor]
    fn constructor(
        ref self: ContractState,
        owner: ContractAddress,
        comet_lib: ClassHash,
        ics23_lib: ClassHash,
        protobuf_lib: ClassHash,
    ) {
        self.ownable.initializer(owner);

        // store the library classes
        // not using storage keys, as these keys are read without contract context.
        write_raw_key::<'comet-library'>(comet_lib);
        write_raw_key::<'ics23-library'>(ics23_lib);
        write_raw_key::<'protobuf-library'>(protobuf_lib);
    }

    #[abi(embed_v0)]
    impl UpgradeableImpl of IUpgradeable<ContractState> {
        fn upgrade(ref self: ContractState, new_class_hash: ClassHash) {
            self.ownable.assert_only_owner();
            self.upgradeable.upgrade(new_class_hash);
        }
    }
}
