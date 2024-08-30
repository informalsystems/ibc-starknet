// This mock contract extends the preset ICS-20 token transfer component,
// granting external access to all internal validation and execution functions
// for testing purposes.
#[starknet::contract]
pub(crate) mod MockTransferApp {
    use starknet::ClassHash;
    use starknet::ContractAddress;
    use starknet_ibc_app_transfer::ICS20TransferComponent;
    use starknet_ibc_app_transfer::transferrable::TransferrableComponent;
    use starknet_ibc_app_transfer::types::{
        PrefixedDenom, Denom, DenomTrait, PacketData, TracePrefix, Memo, TracePrefixTrait,
        PrefixedDenomTrait
    };
    use starknet_ibc_app_transfer::{ERC20Contract, ERC20ContractTrait};
    use starknet_ibc_core_host::{PortId, ChannelId, ChannelIdTrait};

    component!(path: TransferrableComponent, storage: transferrable, event: TransferrableEvent);
    component!(path: ICS20TransferComponent, storage: transfer, event: ICS20TransferEvent);

    #[abi(embed_v0)]
    impl ICS20TransferreableImpl =
        TransferrableComponent::Transferrable<ContractState>;
    impl ICS20TransferreableInternalImpl =
        TransferrableComponent::TransferrableInternalImpl<ContractState>;
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
        transferrable: TransferrableComponent::Storage,
        #[substorage(v0)]
        transfer: ICS20TransferComponent::Storage,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        TransferrableEvent: TransferrableComponent::Event,
        #[flat]
        ICS20TransferEvent: ICS20TransferComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState, erc20_class_hash: ClassHash) {
        self.transferrable.initializer();
        self.transfer.initializer(erc20_class_hash);
    }

    #[external(v0)]
    fn escrow_validate(
        self: @ContractState,
        from_account: ContractAddress,
        port_id: PortId,
        channel_id: ChannelId,
        denom: ERC20Contract,
        amount: u256,
        memo: Memo,
    ) {
        self.transfer.escrow_validate(from_account, port_id, channel_id, denom, amount, memo,);
    }

    #[external(v0)]
    fn unescrow_validate(
        self: @ContractState,
        to_account: ContractAddress,
        port_id: PortId,
        channel_id: ChannelId,
        denom: ERC20Contract,
        amount: u256,
    ) {
        self.transfer.unescrow_validate(to_account, port_id, channel_id, denom, amount,);
    }

    #[external(v0)]
    fn mint_validate(
        self: @ContractState, account: ContractAddress, denom: PrefixedDenom, amount: u256,
    ) {
        self.transfer.mint_validate(account, denom, amount);
    }

    #[external(v0)]
    fn burn_validate(
        self: @ContractState,
        account: ContractAddress,
        denom: PrefixedDenom,
        amount: u256,
        memo: Memo,
    ) {
        self.transfer.burn_validate(account, denom, amount, memo);
    }


    #[external(v0)]
    fn escrow_execute(
        ref self: ContractState,
        from_account: ContractAddress,
        denom: ERC20Contract,
        amount: u256,
        memo: Memo,
    ) {
        self.transfer.escrow_execute(from_account, denom, amount, memo);
    }

    #[external(v0)]
    fn unescrow_execute(
        ref self: ContractState,
        to_account: ContractAddress,
        port_id: PortId,
        channel_id: ChannelId,
        denom: ERC20Contract,
        amount: u256,
    ) {
        self.transfer.unescrow_execute(to_account, port_id, channel_id, denom, amount);
    }

    #[external(v0)]
    fn mint_execute(
        ref self: ContractState, account: ContractAddress, denom: PrefixedDenom, amount: u256,
    ) {
        self.transfer.mint_execute(account, denom, amount);
    }

    #[external(v0)]
    fn burn_execute(
        ref self: ContractState,
        account: ContractAddress,
        denom: PrefixedDenom,
        amount: u256,
        memo: Memo,
    ) {
        self.transfer.burn_execute(account, denom, amount, memo);
    }
}
