use cgp::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::chain::start_chain::{
    ChainFullNodeStarter, ChainFullNodeStarterComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::child_process::CanStartChildProcess;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::types::node_config::StarknetNodeConfig;

#[cgp_new_provider(ChainFullNodeStarterComponent)]
impl<Bootstrap, Runtime> ChainFullNodeStarter<Bootstrap> for StartAnvil
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainNodeConfigType<ChainNodeConfig = StarknetNodeConfig>
        + HasChainGenesisConfigType
        + CanRaiseAsyncError<Runtime::Error>,
    Runtime: CanStartChildProcess + HasFilePathType,
{
    async fn start_chain_full_nodes(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_node_config: &StarknetNodeConfig,
        chain_genesis_config: &Bootstrap::ChainGenesisConfig,
    ) -> Result<Vec<Runtime::ChildProcess>, Bootstrap::Error> {
        let chain_command = Runtime::file_path_from_string("anvil");

        // Use RPC Port + 1 for Anvil port for now
        let rpc_port = chain_node_config.rpc_port + 1;

        let args = [
            "--block-time",
            "1",
            "--chain-id",
            "11155111",
            "--port",
            &rpc_port.to_string(),
        ];

        let stdout_path = Runtime::join_file_path(
            chain_home_dir,
            &Runtime::file_path_from_string("stdout.log"),
        );

        let stderr_path = Runtime::join_file_path(
            chain_home_dir,
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
