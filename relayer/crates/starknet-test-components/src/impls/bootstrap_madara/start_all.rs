use cgp::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::chain::start_chain::{
    ChainFullNodeStarter, ChainFullNodeStarterComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::child_process::HasChildProcessType;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::impls::bootstrap_madara::{StartAnvil, StartMadaraSequencer, StartPathfinder};

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
