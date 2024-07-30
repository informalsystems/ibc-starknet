#[starknet::contract]
pub(crate) mod Transfer {
    use starknet::ClassHash;
    use starknet_ibc::apps::governance::component::IBCGovernanceComponent::GovernanceInternalTrait;
    use starknet_ibc::apps::governance::component::IBCGovernanceComponent;
    use starknet_ibc::apps::transfer::component::ICS20TransferComponent;
    use starknet_ibc::apps::transferrable::component::TransferrableComponent::TransferrableInternalTrait;
    use starknet_ibc::apps::transferrable::component::TransferrableComponent;

    component!(path: IBCGovernanceComponent, storage: governance, event: IBCGovernanceEvent);
    component!(path: TransferrableComponent, storage: transferrable, event: TransferrableEvent);
    component!(path: ICS20TransferComponent, storage: transfer, event: ICS20TransferEvent);

    #[abi(embed_v0)]
    impl IBCGovernanceImpl = IBCGovernanceComponent::Governance<ContractState>;
    #[abi(embed_v0)]
    impl ICS20TransferreableImpl =
        TransferrableComponent::Transferrable<ContractState>;
    #[abi(embed_v0)]
    impl ICS20SendTransferImpl =
        ICS20TransferComponent::SendTransfer<ContractState>;
    #[abi(embed_v0)]
    impl ICS20RecvPacketImpl = ICS20TransferComponent::RecvPacket<ContractState>;
    #[abi(embed_v0)]
    impl ICS20TokenAddressImpl =
        ICS20TransferComponent::IBCTokenAddress<ContractState>;
    impl TransferValidationImpl = ICS20TransferComponent::TransferValidationImpl<ContractState>;
    impl TransferExecutionImpl = ICS20TransferComponent::TransferExecutionImpl<ContractState>;
    impl TransferInternalImpl = ICS20TransferComponent::TransferInternalImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        governance: IBCGovernanceComponent::Storage,
        #[substorage(v0)]
        transferrable: TransferrableComponent::Storage,
        #[substorage(v0)]
        transfer: ICS20TransferComponent::Storage,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        IBCGovernanceEvent: IBCGovernanceComponent::Event,
        #[flat]
        TransferrableEvent: TransferrableComponent::Event,
        #[flat]
        ICS20TransferEvent: ICS20TransferComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState, erc20_class_hash: ClassHash) {
        self.governance.initializer();
        self.transferrable.initializer();
        self.transfer.initializer(erc20_class_hash);
    }
}
