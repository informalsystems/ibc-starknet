// This mock contract extends the preset ICS-20 token transfer component,
// granting external access to all internal validation and execution functions
// for testing purposes.
#[starknet::contract]
pub(crate) mod MockTransferApp {
    use openzeppelin_access::ownable::OwnableComponent;
    use starknet::ClassHash;
    use starknet::ContractAddress;
    use starknet_ibc_apps::transfer::components::{TokenTransferComponent, TransferrableComponent};
    use starknet_ibc_apps::transfer::types::{
        PrefixedDenom, Denom, DenomTrait, PacketData, TracePrefix, Memo, TracePrefixTrait,
        PrefixedDenomTrait
    };
    use starknet_ibc_apps::transfer::{ERC20Contract, ERC20ContractTrait};
    use starknet_ibc_core::host::{PortId, ChannelId, ChannelIdTrait};

    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    component!(path: TransferrableComponent, storage: transferrable, event: TransferrableEvent);
    component!(path: TokenTransferComponent, storage: transfer, event: TokenTransferEvent);

    // Ownable Mixin
    #[abi(embed_v0)]
    impl OwnableMixinImpl = OwnableComponent::OwnableMixinImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;

    // Transferrable
    #[abi(embed_v0)]
    impl TokenTransferreableImpl =
        TransferrableComponent::Transferrable<ContractState>;
    impl TokenTransferreableInternalImpl =
        TransferrableComponent::TransferrableInternalImpl<ContractState>;

    // Token Transfer
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
    impl TransferInitializerImpl = TokenTransferComponent::TransferInitializerImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
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
        TransferrableEvent: TransferrableComponent::Event,
        #[flat]
        TokenTransferEvent: TokenTransferComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress, erc20_class_hash: ClassHash) {
        self.ownable.initializer(owner);
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
