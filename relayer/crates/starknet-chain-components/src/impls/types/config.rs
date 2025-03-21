use core::time::Duration;

use hermes_cosmos_chain_components::impls::types::config::CosmosChainConfig;
use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;

use crate::impls::types::address::StarknetAddress;

#[derive(Debug, Serialize, Deserialize)]
pub struct StarknetRelayerConfig {
    pub cosmos_chain_config: Option<CosmosChainConfig>,
    pub starknet_chain_config: Option<StarknetChainConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StarknetChainConfig {
    pub json_rpc_url: String,
    pub relayer_wallet: String,
    #[serde(with = "humantime_serde")]
    pub poll_interval: Duration,
    pub block_time: Duration,
    pub contract_addresses: StarknetContractAddresses,
    pub contract_classes: StarknetContractClasses,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct StarknetContractAddresses {
    pub ibc_core: Option<StarknetAddress>,
    pub ibc_client: Option<StarknetAddress>,
    pub ibc_ics20: Option<StarknetAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StarknetContractClasses {
    pub erc20: Option<Felt>,
    pub ics20: Option<Felt>,
    pub ibc_client: Option<Felt>,
}
