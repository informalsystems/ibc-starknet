use starknet::core::types::Felt;

#[derive(Debug)]
pub struct StarknetChainStatus {
    pub block_number: u64,
    pub block_hash: Felt,
}
