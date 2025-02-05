use std::net::SocketAddr;

use hermes_cosmos_chain_components::impls::types::config::CosmosChainConfig;
use serde::{Deserialize, Serialize};

use crate::types::wallet::StarknetWallet;

#[derive(Debug, Serialize, Deserialize)]
pub struct StarknetRelayerConfig {
    pub cosmos_chain_config: Option<CosmosChainConfig>,
    pub starknet_chain_config: Option<StarknetChainConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StarknetChainConfig {
    pub json_rpc_url: SocketAddr,
    pub relayer_wallet: StarknetWallet,
}
