use starknet::ContractAddress;
use starknet_ibc_apps::transfer::types::{MsgTransfer, PrefixedDenom};
use starknet_ibc_core_channel::Packet;

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
    /// NOTE: The token key is the Poseidon hash of the token's name, with the
    /// name  prefixed by the destination trace path. For example, if the base
    /// denomination is `uatom` and it is transferred on `transfer/channel-0`,
    /// the token key is the Poseidon hash of a `PrefixedDenom` as follows:
    /// ```cairo
    /// PrefixedDenom {
    ///     trace_path: [TracePrefix {
    ///                     port_id: PortId { port_id: "transfer"},
    ///                     channel_id: ChannelId { channel_id: "channel-0" }
    ///                 }],
    ///     base: Denom::Hosted("uatom")
    /// }
    /// ```
    /// Hashing the denom is delegated to the client as it is more cost-efficient.
    fn ibc_token_address(self: @TContractState, token_key: felt252) -> Option<ContractAddress>;
}

