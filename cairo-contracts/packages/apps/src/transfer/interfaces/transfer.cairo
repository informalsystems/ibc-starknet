use starknet::ContractAddress;
use starknet_ibc_apps::transfer::types::{MsgTransfer, PrefixedDenom};

#[starknet::interface]
pub trait ISendTransfer<TContractState> {
    fn send_transfer(ref self: TContractState, msg: MsgTransfer);
}

#[starknet::interface]
pub trait ICreateIbcToken<TContractState> {
    /// Allows the pre-creation of an IBC token using the expected `PrefixedDenom` on Starknet.
    fn create_ibc_token(ref self: TContractState, denom: PrefixedDenom) -> ContractAddress;
}

#[starknet::interface]
pub trait ITransferQuery<TContractState> {
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
    fn ibc_token_address(self: @TContractState, token_key: felt252) -> ContractAddress;
}

