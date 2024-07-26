use starknet::ContractAddress;
use starknet::Store;

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct Height {
    pub revision_number: u64,
    pub revision_height: u64,
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct Timestamp {
    pub timestamp: u64,
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct ChannelId {
    pub channel_id: felt252,
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct PortId {
    pub port_id: felt252,
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct Sequence {
    pub sequence: u64,
}
