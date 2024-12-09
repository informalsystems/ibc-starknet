use hermes_cosmos_chain_components::impls::types::config::CosmosChainConfig;
use hermes_starknet_test_components::types::wallet::StarknetWallet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StarknetRelayerConfig {
    pub cosmos_chain_config: Option<CosmosChainConfig>,
    pub starknet_chain_config: Option<StarknetChainConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StarknetChainConfig {
    pub json_rpc_url: String,
    pub relayer_wallet: StarknetWallet,
}
