use std::net::IpAddr;

use starknet::core::types::Felt;

#[derive(Clone)]
pub struct StarknetNodeConfig {
    pub rpc_addr: IpAddr,
    pub rpc_port: u16,
    pub sequencer_private_key: Felt,
}
