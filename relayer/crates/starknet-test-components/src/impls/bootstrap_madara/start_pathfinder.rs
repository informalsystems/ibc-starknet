use cgp::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::{
    ChainFullNodeStarter, ChainFullNodeStarterComponent, HasChainGenesisConfigType,
    HasChainNodeConfigType,
};
use hermes_runtime_components::traits::{CanCreateDir, CanStartChildProcess, HasRuntime};

use crate::types::node_config::StarknetNodeConfig;

#[cgp_new_provider(ChainFullNodeStarterComponent)]
impl<Bootstrap, Runtime> ChainFullNodeStarter<Bootstrap> for StartPathfinder
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainNodeConfigType<ChainNodeConfig = StarknetNodeConfig>
        + HasChainGenesisConfigType
        + CanRaiseAsyncError<Runtime::Error>,
    Runtime: CanStartChildProcess + CanCreateDir,
{
    async fn start_chain_full_nodes(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_node_config: &StarknetNodeConfig,
        chain_genesis_config: &Bootstrap::ChainGenesisConfig,
    ) -> Result<Vec<Runtime::ChildProcess>, Bootstrap::Error> {
        let chain_command = Runtime::file_path_from_string("pathfinder");

        let pathfinder_home = Runtime::join_file_path(
            chain_home_dir,
            &Runtime::file_path_from_string("pathfinder"),
        );

        bootstrap
            .runtime()
            .create_dir(&pathfinder_home)
            .await
            .map_err(Bootstrap::raise_error)?;

        let gateway_port = chain_node_config.rpc_port + 1;

        // Use RPC Port + 2 for Anvil port for now
        let anvil_port = chain_node_config.rpc_port + 2;

        // Use RPC Port + 3 for Pathfinder port for now
        let pathfinder_port = chain_node_config.rpc_port + 3;

        let args = [
            "--data-directory",
            &Runtime::file_path_to_string(&pathfinder_home),
            "--network",
            "custom",
            "--storage.state-tries",
            "archive",
            "--ethereum.url",
            &format!("http://localhost:{anvil_port}"),
            "--gateway-url",
            &format!("http://localhost:{gateway_port}/gateway"),
            "--feeder-gateway-url",
            &format!("http://localhost:{gateway_port}/feeder_gateway"),
            "--chain-id",
            "starknet-devnet",
            "--http-rpc",
            &format!("127.0.0.1:{pathfinder_port}"),
        ];

        let stdout_path = Runtime::join_file_path(
            &pathfinder_home,
            &Runtime::file_path_from_string("stdout.log"),
        );

        let stderr_path = Runtime::join_file_path(
            &pathfinder_home,
            &Runtime::file_path_from_string("stderr.log"),
        );

        let child_process = bootstrap
            .runtime()
            .start_child_process(
                &chain_command,
                &args,
                &[],
                Some(&stdout_path),
                Some(&stderr_path),
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(vec![child_process])
    }
}
