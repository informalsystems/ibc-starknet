use starknet::ContractAddress;
use starknet_ibc::apps::transfer::types::{MsgTransfer, PrefixedDenom};
use starknet_ibc::core::channel::types::Packet;

#[starknet::interface]
pub trait ISendTransfer<TContractState> {
    fn send_validate(self: @TContractState, msg: MsgTransfer);
    fn send_execute(ref self: TContractState, msg: MsgTransfer);
}

#[starknet::interface]
pub trait IRecvPacket<TContractState> {
    fn recv_validate(self: @TContractState, packet: Packet);
    fn recv_execute(ref self: TContractState, packet: Packet);
}

#[starknet::interface]
pub trait ITokenAddress<TContractState> {
    fn ibc_token_address(
        self: @TContractState, prefixed_denom: PrefixedDenom
    ) -> Option<ContractAddress>;
}

