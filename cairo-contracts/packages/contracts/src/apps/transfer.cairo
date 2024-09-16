#[starknet::contract]
pub mod TransferApp {
    use core::num::traits::Zero;
    use openzeppelin_access::ownable::OwnableComponent;
    use starknet::ClassHash;
    use starknet::ContractAddress;
    use starknet_ibc_apps::transfer::TransferErrors;
    use starknet_ibc_apps::transfer::components::{TokenTransferComponent, TransferrableComponent};
    use starknet_ibc_utils::governance::IBCGovernanceComponent;

    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    component!(path: IBCGovernanceComponent, storage: governance, event: IBCGovernanceEvent);
    component!(path: TransferrableComponent, storage: transferrable, event: TransferrableEvent);
    component!(path: TokenTransferComponent, storage: transfer, event: TokenTransferEvent);

    // Ownable Mixin
    #[abi(embed_v0)]
    impl OwnableMixinImpl = OwnableComponent::OwnableMixinImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;

    // IBC Governance
    #[abi(embed_v0)]
    impl IBCGovernanceImpl = IBCGovernanceComponent::Governance<ContractState>;
    impl IBCGovernanceInternalImpl = IBCGovernanceComponent::GovernanceInternalImpl<ContractState>;

    // Transferrable
    #[abi(embed_v0)]
    impl TokenTransferreableImpl =
        TransferrableComponent::Transferrable<ContractState>;
    impl TokenTransferreableInternal =
        TransferrableComponent::TransferrableInternalImpl<ContractState>;

    // Token Transfer
    #[abi(embed_v0)]
    impl TokenSendTransferImpl =
        TokenTransferComponent::SendTransfer<ContractState>;
    #[abi(embed_v0)]
    impl TransferAppCallbackImpl =
        TokenTransferComponent::TransferAppCallback<ContractState>;
    #[abi(embed_v0)]
    impl TokenTokenAddressImpl =
        TokenTransferComponent::IBCTokenAddress<ContractState>;
    impl TransferValidationImpl = TokenTransferComponent::TransferValidationImpl<ContractState>;
    impl TransferExecutionImpl = TokenTransferComponent::TransferExecutionImpl<ContractState>;
    impl TransferInitializerImpl = TokenTransferComponent::TransferInitializerImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
        #[substorage(v0)]
        governance: IBCGovernanceComponent::Storage,
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
        IBCGovernanceEvent: IBCGovernanceComponent::Event,
        #[flat]
        TransferrableEvent: TransferrableComponent::Event,
        #[flat]
        TokenTransferEvent: TokenTransferComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress, erc20_class_hash: ClassHash) {
        assert(owner.is_non_zero(), TransferErrors::ZERO_OWNER);
        self.ownable.initializer(owner);
        self.governance.initializer();
        self.transferrable.initializer();
        self.transfer.initializer(erc20_class_hash);
    }
}
