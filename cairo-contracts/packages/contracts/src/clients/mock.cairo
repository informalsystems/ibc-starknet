#[starknet::contract]
pub mod MockClient {
    use core::num::traits::Zero;
    use openzeppelin_access::ownable::OwnableComponent;
    use openzeppelin_upgrades::UpgradeableComponent;
    use openzeppelin_upgrades::interface::IUpgradeable;
    use starknet::{ClassHash, ContractAddress};
    use starknet_ibc_clients::mock::{MockClientComponent, MockErrors};

    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    component!(path: UpgradeableComponent, storage: upgradeable, event: UpgradeableEvent);
    component!(path: MockClientComponent, storage: client, event: MockClientEvent);

    #[abi(embed_v0)]
    impl OwnableMixinImpl = OwnableComponent::OwnableMixinImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;

    impl UpgradeableInternalImpl = UpgradeableComponent::InternalImpl<ContractState>;

    #[abi(embed_v0)]
    impl MockClientHandlerImpl =
        MockClientComponent::MockClientHandler<ContractState>;
    #[abi(embed_v0)]
    impl MockClientQueryImpl = MockClientComponent::MockClientQuery<ContractState>;

    // NOTE: The client state validation interface is exposed for public use.
    // However, only the IBC core contract (owner) can invoke the execution methods.

    #[abi(embed_v0)]
    impl MockClientValidationImpl =
        MockClientComponent::MockClientValidation<ContractState>;
    impl MockClientExecutionImpl = MockClientComponent::MockClientExecution<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
        #[substorage(v0)]
        upgradeable: UpgradeableComponent::Storage,
        #[substorage(v0)]
        client: MockClientComponent::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        OwnableEvent: OwnableComponent::Event,
        #[flat]
        UpgradeableEvent: UpgradeableComponent::Event,
        #[flat]
        MockClientEvent: MockClientComponent::Event,
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
    }

    #[abi(embed_v0)]
    impl UpgradeableImpl of IUpgradeable<ContractState> {
        fn upgrade(ref self: ContractState, new_class_hash: ClassHash) {
            self.ownable.assert_only_owner();
            self.upgradeable.upgrade(new_class_hash);
        }
    }
}
