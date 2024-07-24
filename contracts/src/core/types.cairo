use starknet::ContractAddress;
use starknet::Store;

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct ChannelId {
    channel_id: felt252,
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct PortId {
    port_id: felt252,
}
