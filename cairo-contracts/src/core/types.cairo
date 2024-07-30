use starknet::ContractAddress;
use starknet::Store;

#[derive(Drop, Serde, Store)]
pub struct ChannelId {
    channel_id: felt252,
}

#[derive(Drop, Serde, Store)]
pub struct PortId {
    port_id: felt252,
}
