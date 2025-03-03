use cgp::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::{
    ChainNodeConfigTypeComponent, ProvideChainNodeConfigType,
};

use crate::types::node_config::StarknetNodeConfig;

pub struct ProvideStarknetNodeConfigType;

#[cgp_provider(ChainNodeConfigTypeComponent)]
impl<Bootstrap: Async> ProvideChainNodeConfigType<Bootstrap> for ProvideStarknetNodeConfigType {
    type ChainNodeConfig = StarknetNodeConfig;
}
