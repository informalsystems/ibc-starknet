use starknet::ContractAddress;
use starknet_ibc::apps::transfer::types::{MsgTransfer, PrefixedCoin, Memo};
use starknet_ibc::core::types::{PortId, ChannelId};

#[starknet::interface]
pub trait ITransfer<TContractState> {
    fn send_transfer(ref self: TContractState, msg: MsgTransfer);
    fn register_token(
        ref self: TContractState, token_name: felt252, token_address: ContractAddress
    );
}

#[starknet::interface]
pub trait ITransferrable<TContractState> {
    fn can_send(self: @TContractState);
    fn can_receive(self: @TContractState);
}

#[starknet::interface]
pub trait ITransferValidationContext<TContractState> {
    fn escrow_validate(
        self: @TContractState,
        from_address: ContractAddress,
        port_id: PortId,
        channel_id: ChannelId,
        coin: PrefixedCoin,
        memo: Memo,
    );
    fn unescrow_validate(
        self: @TContractState,
        to_address: ContractAddress,
        port_id: PortId,
        channel_id: ChannelId,
        coin: PrefixedCoin,
    );
    fn mint_validate(self: @TContractState, address: ContractAddress, coin: PrefixedCoin,);
    fn burn_validate(
        self: @TContractState, address: ContractAddress, coin: PrefixedCoin, memo: Memo,
    );
}

#[starknet::interface]
pub trait ITransferExecutionContext<TContractState> {
    fn escrow_execute(
        ref self: TContractState,
        from_address: ContractAddress,
        port_id: PortId,
        channel_id: ChannelId,
        coin: PrefixedCoin,
        memo: Memo,
    );
    fn unescrow_execute(
        ref self: TContractState,
        to_address: ContractAddress,
        port_id: PortId,
        channel_id: ChannelId,
        coin: PrefixedCoin,
    );
    fn mint_execute(ref self: TContractState, address: ContractAddress, coin: PrefixedCoin,);
    fn burn_execute(
        ref self: TContractState, address: ContractAddress, coin: PrefixedCoin, memo: Memo,
    );
}

