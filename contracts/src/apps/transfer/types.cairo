use starknet::ContractAddress;
use starknet_ibc::core::types::{PortId, ChannelId};

/// Maximum memo length allowed for ICS-20 transfers. This bound corresponds to
/// the `MaximumMemoLength` in the `ibc-go`.
pub(crate) const MAXIMUM_MEMO_LENGTH: u32 = 32768;

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct MsgTransfer {
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub packet_data: PacketData,
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct PacketData {
    pub token: PrefixedCoin,
    pub sender: ContractAddress,
    pub receiver: ContractAddress,
    pub memo: Memo,
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct PrefixedCoin {
    pub denom: felt252,
    pub amount: u256,
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct Memo {
    pub memo: ByteArray,
}

