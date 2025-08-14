use hermes_starknet_chain_components::impls::StarknetAddress;

#[derive(Clone)]
pub struct StarknetGenesisConfig {
    pub seed: u64,
    pub transfer_denom: StarknetAddress,
    pub staking_denom: StarknetAddress,
}
