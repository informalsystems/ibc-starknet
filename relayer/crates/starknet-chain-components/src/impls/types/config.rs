use core::time::Duration;

use hermes_cosmos_core::chain_components::impls::CosmosChainConfig;
use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;

use crate::impls::StarknetAddress;

#[derive(Debug, Serialize, Deserialize)]
pub struct StarknetRelayerConfig {
    pub cosmos_chain_config: Option<CosmosChainConfig>,
    pub starknet_chain_config: Option<StarknetChainConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StarknetChainConfig {
    pub json_rpc_url: String,
    pub feeder_gateway_url: String,
    pub ed25519_attestator_addresses: Option<Vec<String>>,
    pub relayer_wallet_1: String,
    pub relayer_wallet_2: String,
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
