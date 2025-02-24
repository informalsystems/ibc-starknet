use cgp::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::{
    ChainGenesisConfigTypeComponent, ProvideChainGenesisConfigType,
};

use crate::types::genesis_config::StarknetGenesisConfig;

pub struct ProvideStarknetGenesisConfigType;

#[cgp_provider(ChainGenesisConfigTypeComponent)]
impl<Bootstrap: Async> ProvideChainGenesisConfigType<Bootstrap>
    for ProvideStarknetGenesisConfigType
{
    type ChainGenesisConfig = StarknetGenesisConfig;
}
