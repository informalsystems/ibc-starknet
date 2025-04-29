use hermes_cosmos_core::test_components::bootstrap::traits::{
    ChainNodeConfigTypeComponent, ProvideChainNodeConfigType,
};
use hermes_prelude::*;

use crate::types::node_config::StarknetNodeConfig;

pub struct ProvideStarknetNodeConfigType;

#[cgp_provider(ChainNodeConfigTypeComponent)]
impl<Bootstrap: Async> ProvideChainNodeConfigType<Bootstrap> for ProvideStarknetNodeConfigType {
    type ChainNodeConfig = StarknetNodeConfig;
}
