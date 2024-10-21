use hermes_cosmos_chain_components::types::status::Time;
use starknet::core::types::Felt;

#[derive(Debug)]
pub struct StarknetChainStatus {
    pub height: u64,
    pub block_hash: Felt,
    pub time: Time,
}
