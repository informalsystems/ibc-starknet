use hermes_core::runtime_components::traits::{HasChildProcessType, HasFilePathType, HasRuntime};
use hermes_cosmos_core::test_components::bootstrap::traits::{
    ChainFullNodeStarter, ChainFullNodeStarterComponent, HasChainGenesisConfigType,
    HasChainNodeConfigType,
};
use hermes_prelude::*;

use crate::impls::{StartAnvil, StartMadaraSequencer, StartPathfinder};

#[cgp_new_provider(ChainFullNodeStarterComponent)]
impl<Bootstrap, Runtime> ChainFullNodeStarter<Bootstrap> for StartMadaraStack
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainNodeConfigType
        + HasChainGenesisConfigType
        + HasAsyncErrorType,
    Runtime: HasChildProcessType + HasFilePathType,
    StartMadaraSequencer: ChainFullNodeStarter<Bootstrap>,
    StartAnvil: ChainFullNodeStarter<Bootstrap>,
    StartPathfinder: ChainFullNodeStarter<Bootstrap>,
{
    async fn start_chain_full_nodes(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_node_config: &Bootstrap::ChainNodeConfig,
        chain_genesis_config: &Bootstrap::ChainGenesisConfig,
    ) -> Result<Vec<Runtime::ChildProcess>, Bootstrap::Error> {
        let mut madara_processes = StartMadaraSequencer::start_chain_full_nodes(
            bootstrap,
            chain_home_dir,
            chain_node_config,
            chain_genesis_config,
        )
        .await?;

        let anvil_processes = StartAnvil::start_chain_full_nodes(
            bootstrap,
            chain_home_dir,
            chain_node_config,
            chain_genesis_config,
        )
        .await?;
        madara_processes.extend(anvil_processes);

        let pathfinder_processes = StartPathfinder::start_chain_full_nodes(
            bootstrap,
            chain_home_dir,
            chain_node_config,
            chain_genesis_config,
        )
        .await?;
        madara_processes.extend(pathfinder_processes);

        Ok(madara_processes)
    }
}
