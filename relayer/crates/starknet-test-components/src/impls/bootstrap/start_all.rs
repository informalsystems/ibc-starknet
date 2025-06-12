use hermes_core::runtime_components::traits::{HasChildProcessType, HasFilePathType, HasRuntime};
use hermes_cosmos_core::test_components::bootstrap::traits::{
    ChainFullNodeStarter, ChainFullNodeStarterComponent, HasChainGenesisConfigType,
    HasChainNodeConfigType,
};
use hermes_prelude::*;

use crate::impls::{StartAnvil, StartPathfinder, StartStarknetSequencer};

#[cgp_new_provider(ChainFullNodeStarterComponent)]
impl<Bootstrap, Runtime> ChainFullNodeStarter<Bootstrap> for StartStarknetStack
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainNodeConfigType
        + HasChainGenesisConfigType
        + HasAsyncErrorType,
    Runtime: HasChildProcessType + HasFilePathType,
    StartStarknetSequencer: ChainFullNodeStarter<Bootstrap>,
    StartAnvil: ChainFullNodeStarter<Bootstrap>,
    StartPathfinder: ChainFullNodeStarter<Bootstrap>,
{
    async fn start_chain_full_nodes(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_node_config: &Bootstrap::ChainNodeConfig,
        chain_genesis_config: &Bootstrap::ChainGenesisConfig,
    ) -> Result<Vec<Runtime::ChildProcess>, Bootstrap::Error> {
        let mut starknet_processes = StartStarknetSequencer::start_chain_full_nodes(
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
        starknet_processes.extend(anvil_processes);

        let pathfinder_processes = StartPathfinder::start_chain_full_nodes(
            bootstrap,
            chain_home_dir,
            chain_node_config,
            chain_genesis_config,
        )
        .await?;
        starknet_processes.extend(pathfinder_processes);

        Ok(starknet_processes)
    }
}
