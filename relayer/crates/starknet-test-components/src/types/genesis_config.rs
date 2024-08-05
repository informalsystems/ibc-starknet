use starknet::core::types::Felt;

pub struct StarknetGenesisConfig {
    pub seed: u64,
    pub transfer_denom: Felt,
    pub staking_denom: Felt,
}
