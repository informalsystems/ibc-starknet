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
    /// Returns the contract address of an IBC token given its key.
    ///
    /// NOTE: The token key is the Poseidon hash of the token's name, with the name
    /// prefixed by the source trace path. For example, if the base denomination
    /// is `uatom` and it is transferred on `transfer/channel-0`, the token key is the
    /// Poseidon hash of `transfer/channel-0/uatom`. Hashing the name is delegated
    /// to the client as it is more cost-effective to perform off-chain.
    fn ibc_token_address(self: @TContractState, token_key: felt252) -> Option<ContractAddress>;
}

