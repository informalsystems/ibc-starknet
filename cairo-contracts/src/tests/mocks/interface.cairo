use starknet::ContractAddress;
use starknet_ibc::apps::transfer::types::{
    Denom, DenomTrait, PacketData, TracePrefix, Memo, TracePrefixTrait, ERC20TokenTrait, ERC20Token,
    PrefixedDenomTrait
};
use starknet_ibc::apps::transfer::types::{MsgTransfer, PrefixedDenom};
use starknet_ibc::core::channel::types::Packet;
use starknet_ibc::core::host::types::{PortId, ChannelId, ChannelIdTrait};

#[starknet::interface]
pub trait ITransferValidate<TContractState> {
    fn escrow_validate(
        self: @TContractState,
        from_account: ContractAddress,
        port_id: PortId,
        channel_id: ChannelId,
        denom: ERC20Token,
        amount: u256,
        memo: Memo,
    );

    fn unescrow_validate(
        self: @TContractState,
        to_account: ContractAddress,
        port_id: PortId,
        channel_id: ChannelId,
        denom: ERC20Token,
        amount: u256,
    );

    fn mint_validate(
        self: @TContractState, account: ContractAddress, denom: PrefixedDenom, amount: u256,
    );

    fn burn_validate(
        self: @TContractState,
        account: ContractAddress,
        denom: PrefixedDenom,
        amount: u256,
        memo: Memo,
    );
}

#[starknet::interface]
pub trait ITransferExecute<TContractState> {
    fn escrow_execute(
        ref self: TContractState,
        from_account: ContractAddress,
        denom: ERC20Token,
        amount: u256,
        memo: Memo,
    );

    fn unescrow_execute(
        ref self: TContractState,
        to_account: ContractAddress,
        port_id: PortId,
        channel_id: ChannelId,
        denom: ERC20Token,
        amount: u256,
    );

    fn mint_execute(
        ref self: TContractState, account: ContractAddress, denom: PrefixedDenom, amount: u256,
    );

    fn burn_execute(
        ref self: TContractState,
        account: ContractAddress,
        denom: PrefixedDenom,
        amount: u256,
        memo: Memo,
    );
}
