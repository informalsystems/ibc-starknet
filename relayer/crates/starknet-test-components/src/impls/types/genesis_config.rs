use hermes_cosmos_core::test_components::bootstrap::traits::{
    ChainGenesisConfigTypeComponent, ProvideChainGenesisConfigType,
};
use hermes_prelude::*;

use crate::types::StarknetGenesisConfig;

pub struct ProvideStarknetGenesisConfigType;

#[cgp_provider(ChainGenesisConfigTypeComponent)]
impl<Bootstrap: Async> ProvideChainGenesisConfigType<Bootstrap>
    for ProvideStarknetGenesisConfigType
{
    type ChainGenesisConfig = StarknetGenesisConfig;
}
