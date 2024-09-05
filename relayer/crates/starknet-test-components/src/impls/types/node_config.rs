use cgp::core::Async;
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk::ProvideChainNodeConfigType;

use crate::types::node_config::StarknetNodeConfig;

pub struct ProvideStarknetNodeConfigType;

impl<Bootstrap: Async> ProvideChainNodeConfigType<Bootstrap> for ProvideStarknetNodeConfigType {
    type ChainNodeConfig = StarknetNodeConfig;
}
