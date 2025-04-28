use cgp::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::{
    ChainFullNodeStarter, ChainFullNodeStarterComponent, HasChainGenesisConfigType,
    HasChainNodeConfigType,
};
use hermes_runtime_components::traits::{CanCreateDir, CanStartChildProcess, HasRuntime};

use crate::types::node_config::StarknetNodeConfig;

#[cgp_new_provider(ChainFullNodeStarterComponent)]
impl<Bootstrap, Runtime> ChainFullNodeStarter<Bootstrap> for StartAnvil
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
        let chain_command = Runtime::file_path_from_string("anvil");

        let anvil_home =
            Runtime::join_file_path(chain_home_dir, &Runtime::file_path_from_string("anvil"));

        bootstrap
            .runtime()
            .create_dir(&anvil_home)
            .await
            .map_err(Bootstrap::raise_error)?;

        // Use RPC Port + 2 for Anvil port for now
        let rpc_port = chain_node_config.rpc_port + 2;

        let args = [
            "--block-time",
            "1",
            "--chain-id",
            "11155111",
            "--port",
            &rpc_port.to_string(),
        ];

        let stdout_path =
            Runtime::join_file_path(&anvil_home, &Runtime::file_path_from_string("stdout.log"));

        let stderr_path =
            Runtime::join_file_path(&anvil_home, &Runtime::file_path_from_string("stderr.log"));

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
