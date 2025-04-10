use cgp::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::chain::start_chain::{
    ChainFullNodeStarter, ChainFullNodeStarterComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_runtime_components::traits::fs::create_dir::CanCreateDir;
use hermes_runtime_components::traits::os::child_process::CanStartChildProcess;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::types::node_config::StarknetNodeConfig;

#[cgp_new_provider(ChainFullNodeStarterComponent)]
impl<Bootstrap, Runtime> ChainFullNodeStarter<Bootstrap> for StartMadaraSequencer
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainNodeConfigType<ChainNodeConfig = StarknetNodeConfig>
        + HasChainGenesisConfigType
        + HasChainCommandPath
        + CanRaiseAsyncError<Runtime::Error>,
    Runtime: CanStartChildProcess + CanCreateDir,
{
    async fn start_chain_full_nodes(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_node_config: &StarknetNodeConfig,
        chain_genesis_config: &Bootstrap::ChainGenesisConfig,
    ) -> Result<Vec<Runtime::ChildProcess>, Bootstrap::Error> {
        let chain_command = bootstrap.chain_command_path();

        let madara_home =
            Runtime::join_file_path(chain_home_dir, &Runtime::file_path_from_string("madara"));

        bootstrap
            .runtime()
            .create_dir(&madara_home)
            .await
            .map_err(Bootstrap::raise_error)?;

        let args = [
            "--devnet",
            "--rpc-port",
            &chain_node_config.rpc_port.to_string(),
            "--base-path",
            &Runtime::file_path_to_string(&madara_home),
            "--chain-config-override",
            "block_time=1s,pending_block_update_time=1s",
        ];

        let stdout_path =
            Runtime::join_file_path(&madara_home, &Runtime::file_path_from_string("stdout.log"));

        let stderr_path =
            Runtime::join_file_path(&madara_home, &Runtime::file_path_from_string("stderr.log"));

        let child_process = bootstrap
            .runtime()
            .start_child_process(
                chain_command,
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
