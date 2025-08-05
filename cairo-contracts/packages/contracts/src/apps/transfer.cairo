#[starknet::contract]
pub mod TransferApp {
    use core::num::traits::Zero;
    use openzeppelin_access::ownable::OwnableComponent;
    use openzeppelin_upgrades::UpgradeableComponent;
    use openzeppelin_upgrades::interface::IUpgradeable;
    use starknet::{ClassHash, ContractAddress};
    use starknet_ibc_apps::transfer::{
        TokenTransferComponent, TransferErrors, TransferrableComponent,
    };

    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    component!(path: UpgradeableComponent, storage: upgradeable, event: UpgradeableEvent);
    component!(path: TransferrableComponent, storage: transferrable, event: TransferrableEvent);
    component!(path: TokenTransferComponent, storage: transfer, event: TokenTransferEvent);

    // Ownable Mixin
    #[abi(embed_v0)]
    impl OwnableMixinImpl = OwnableComponent::OwnableMixinImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;

    impl UpgradeableInternalImpl = UpgradeableComponent::InternalImpl<ContractState>;

    // Transferrable
    #[abi(embed_v0)]
    impl TokenTransferrableImpl =
        TransferrableComponent::Transferrable<ContractState>;
    impl TokenTransferrableInternal =
        TransferrableComponent::TransferrableInternalImpl<ContractState>;

    // Token Transfer
    #[abi(embed_v0)]
    impl TokenSendTransferImpl =
        TokenTransferComponent::SendTransfer<ContractState>;
    #[abi(embed_v0)]
    impl TransferAppCallbackImpl =
        TokenTransferComponent::TransferAppCallback<ContractState>;
    #[abi(embed_v0)]
    impl CreateIbcTokenImpl = TokenTransferComponent::CreateIbcToken<ContractState>;
    #[abi(embed_v0)]
    impl TokenTransferQueryImpl =
        TokenTransferComponent::TokenTransferQuery<ContractState>;
    impl TransferValidationImpl = TokenTransferComponent::TransferValidationImpl<ContractState>;
    impl TransferExecutionImpl = TokenTransferComponent::TransferExecutionImpl<ContractState>;
    impl TransferInitializerImpl = TokenTransferComponent::TransferInitializerImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
        #[substorage(v0)]
        upgradeable: UpgradeableComponent::Storage,
        #[substorage(v0)]
        transferrable: TransferrableComponent::Storage,
        #[substorage(v0)]
        transfer: TokenTransferComponent::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        OwnableEvent: OwnableComponent::Event,
        #[flat]
        UpgradeableEvent: UpgradeableComponent::Event,
        #[flat]
        TransferrableEvent: TransferrableComponent::Event,
        #[flat]
        TokenTransferEvent: TokenTransferComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress, erc20_class_hash: ClassHash) {
        assert(owner.is_non_zero(), TransferErrors::ZERO_OWNER);
        self.ownable.initializer(owner);
        self.transferrable.initializer();
        self.transfer.initializer(erc20_class_hash);
    }

    #[abi(embed_v0)]
    impl UpgradeableImpl of IUpgradeable<ContractState> {
        fn upgrade(ref self: ContractState, new_class_hash: ClassHash) {
            self.ownable.assert_only_owner();
            self.upgradeable.upgrade(new_class_hash);
        }
    }
}
