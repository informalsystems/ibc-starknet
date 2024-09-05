use cgp::core::Async;
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk::ProvideChainGenesisConfigType;

use crate::types::genesis_config::StarknetGenesisConfig;

pub struct ProvideStarknetGenesisConfigType;

impl<Bootstrap: Async> ProvideChainGenesisConfigType<Bootstrap>
    for ProvideStarknetGenesisConfigType
{
    type ChainGenesisConfig = StarknetGenesisConfig;
}
