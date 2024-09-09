use starknet::core::types::Felt;

pub struct StarknetChainStatus {
    pub block_number: u64,
    pub block_hash: Felt,
}
