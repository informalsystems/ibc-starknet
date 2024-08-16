// This mock contract extends the preset ICS-20 token transfer component,
// granting external access to all internal validation and execution functions
// for testing purposes.
#[starknet::contract]
pub(crate) mod TransferMock {
    use starknet::ClassHash;
    use starknet::ContractAddress;
    use starknet_ibc::apps::transfer::component::ICS20TransferComponent;
    use starknet_ibc::apps::transfer::types::{
        PrefixedDenom, Denom, DenomTrait, PacketData, TracePrefix, Memo, TracePrefixTrait,
        ERC20TokenTrait, ERC20Token, PrefixedDenomTrait
    };
    use starknet_ibc::apps::transferrable::component::TransferrableComponent::TransferrableInternalTrait;
    use starknet_ibc::apps::transferrable::component::TransferrableComponent;
    use starknet_ibc::core::host::types::{PortId, ChannelId, ChannelIdTrait};
    use starknet_ibc::tests::mocks::interface::ITransferExecute;
    use starknet_ibc::tests::mocks::interface::ITransferValidate;

    component!(path: TransferrableComponent, storage: transferrable, event: TransferrableEvent);
    component!(path: ICS20TransferComponent, storage: transfer, event: ICS20TransferEvent);

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

    #[abi(embed_v0)]
    impl TransferValidateImpl of ITransferValidate<ContractState> {
        fn escrow_validate(
            self: @ContractState,
            from_account: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            denom: ERC20Token,
            amount: u256,
            memo: Memo,
        ) {
            self.transfer.escrow_validate(from_account, port_id, channel_id, denom, amount, memo,);
        }

        fn unescrow_validate(
            self: @ContractState,
            to_account: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            denom: ERC20Token,
            amount: u256,
        ) {
            self.transfer.unescrow_validate(to_account, port_id, channel_id, denom, amount,);
        }

        fn mint_validate(
            self: @ContractState, account: ContractAddress, denom: PrefixedDenom, amount: u256,
        ) {
            self.transfer.mint_validate(account, denom, amount);
        }

        fn burn_validate(
            self: @ContractState,
            account: ContractAddress,
            denom: PrefixedDenom,
            amount: u256,
            memo: Memo,
        ) {
            self.transfer.burn_validate(account, denom, amount, memo);
        }
    }

    #[abi(embed_v0)]
    impl TransferExecuteImpl of ITransferExecute<ContractState> {
        fn escrow_execute(
            ref self: ContractState,
            from_account: ContractAddress,
            denom: ERC20Token,
            amount: u256,
            memo: Memo,
        ) {
            self.transfer.escrow_execute(from_account, denom, amount, memo);
        }

        fn unescrow_execute(
            ref self: ContractState,
            to_account: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            denom: ERC20Token,
            amount: u256,
        ) {
            self.transfer.unescrow_execute(to_account, port_id, channel_id, denom, amount);
        }

        fn mint_execute(
            ref self: ContractState, account: ContractAddress, denom: PrefixedDenom, amount: u256,
        ) {
            self.transfer.mint_execute(account, denom, amount);
        }

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
}
