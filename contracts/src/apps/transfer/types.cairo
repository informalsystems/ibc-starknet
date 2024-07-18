use starknet::ContractAddress;
use starknet_ibc::core::types::{PortId, ChannelId};

#[derive(Drop, Serde, Store)]
pub struct MsgTransfer {
    port_id_on_a: PortId,
    chan_id_on_a: ChannelId,
    packet_data: PacketData,
}

#[derive(Drop, Serde, Store)]
pub struct PrefixedCoin {
    denom: ByteArray,
    amount: u64,
}

#[derive(Drop, Serde, Store)]
pub struct Memo {
    memo: ByteArray,
}

#[derive(Drop, Serde, Store)]
pub struct PacketData {
    pub token: PrefixedCoin,
    pub sender: ContractAddress,
    pub receiver: ContractAddress,
    pub memo: Memo,
}
