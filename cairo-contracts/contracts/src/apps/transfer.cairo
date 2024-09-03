#[starknet::contract]
pub mod TransferApp {
    use starknet::ClassHash;
    use starknet_ibc_apps::transfer::components::{TokenTransferComponent, TransferrableComponent};
    use starknet_ibc_utils::governance::IBCGovernanceComponent;

    component!(path: IBCGovernanceComponent, storage: governance, event: IBCGovernanceEvent);
    component!(path: TransferrableComponent, storage: transferrable, event: TransferrableEvent);
    component!(path: TokenTransferComponent, storage: transfer, event: TokenTransferEvent);

    #[abi(embed_v0)]
    impl IBCGovernanceImpl = IBCGovernanceComponent::Governance<ContractState>;
    impl IBCGovernanceInternalImpl = IBCGovernanceComponent::GovernanceInternalImpl<ContractState>;

    #[abi(embed_v0)]
    impl TokenTransferreableImpl =
        TransferrableComponent::Transferrable<ContractState>;
    impl TokenTransferreableInternal =
        TransferrableComponent::TransferrableInternalImpl<ContractState>;

    #[abi(embed_v0)]
    impl TokenSendTransferImpl =
        TokenTransferComponent::SendTransfer<ContractState>;
    #[abi(embed_v0)]
    impl TokenRecvPacketImpl = TokenTransferComponent::RecvPacket<ContractState>;
    #[abi(embed_v0)]
    impl TokenTokenAddressImpl =
        TokenTransferComponent::IBCTokenAddress<ContractState>;
    impl TransferValidationImpl = TokenTransferComponent::TransferValidationImpl<ContractState>;
    impl TransferExecutionImpl = TokenTransferComponent::TransferExecutionImpl<ContractState>;
    impl TransferInternalImpl = TokenTransferComponent::TransferInternalImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        governance: IBCGovernanceComponent::Storage,
        #[substorage(v0)]
        transferrable: TransferrableComponent::Storage,
        #[substorage(v0)]
        transfer: TokenTransferComponent::Storage,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        IBCGovernanceEvent: IBCGovernanceComponent::Event,
        #[flat]
        TransferrableEvent: TransferrableComponent::Event,
        #[flat]
        TokenTransferEvent: TokenTransferComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState, erc20_class_hash: ClassHash) {
        self.governance.initializer();
        self.transferrable.initializer();
        self.transfer.initializer(erc20_class_hash);
    }
}
