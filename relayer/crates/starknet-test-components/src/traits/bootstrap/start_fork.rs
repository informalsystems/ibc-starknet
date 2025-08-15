use hermes_core::runtime_components::traits::{
    ChildProcessOf, FilePathOf, HasChildProcessType, HasFilePathType, HasRuntime,
};
use hermes_cosmos_core::test_components::bootstrap::traits::{
    HasChainGenesisConfigType, HasChainNodeConfigType,
};
use hermes_prelude::*;

#[cgp_component {
  provider: ChainForkedFullNodeStarter,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanStartChainForkedFullNodes:
    HasChainNodeConfigType
    + HasChainGenesisConfigType
    + HasRuntime<Runtime: HasChildProcessType + HasFilePathType>
    + HasAsyncErrorType
{
    async fn start_chain_forked_full_nodes(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
        chain_node_config: &Self::ChainNodeConfig,
        chain_genesis_config: &Self::ChainGenesisConfig,
        backup_dir: &FilePathOf<Self::Runtime>,
        number_of_blocks: &str,
    ) -> Result<Vec<ChildProcessOf<Self::Runtime>>, Self::Error>;
}
