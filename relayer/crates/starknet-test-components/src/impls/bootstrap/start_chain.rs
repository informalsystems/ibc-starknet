use cgp::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::chain::start_chain::{
    ChainFullNodeStarter, ChainFullNodeStarterComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::child_process::CanStartChildProcess;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::types::genesis_config::StarknetGenesisConfig;
use crate::types::node_config::StarknetNodeConfig;

pub struct StartStarknetDevnet;

#[cgp_provider(ChainFullNodeStarterComponent)]
impl<Bootstrap, Runtime> ChainFullNodeStarter<Bootstrap> for StartStarknetDevnet
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainNodeConfigType<ChainNodeConfig = StarknetNodeConfig>
        + HasChainGenesisConfigType<ChainGenesisConfig = StarknetGenesisConfig>
        + HasChainCommandPath
        + CanRaiseAsyncError<Runtime::Error>,
    Runtime: CanStartChildProcess + HasFilePathType,
{
    async fn start_chain_full_node(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_node_config: &StarknetNodeConfig,
        chain_genesis_config: &StarknetGenesisConfig,
    ) -> Result<Runtime::ChildProcess, Bootstrap::Error> {
        let chain_command = bootstrap.chain_command_path();

        let chain_state_path = Runtime::join_file_path(
            chain_home_dir,
            &Runtime::file_path_from_string("chain-state.json"),
        );

        let args = [
            "--seed",
            &chain_genesis_config.seed.to_string(),
            "--port",
            &chain_node_config.rpc_port.to_string(),
            "--block-generation-on",
            "1",
            "--state-archive-capacity",
            "full",
            "--dump-on",
            "block",
            "--request-body-size-limit=3000000", // 3M; ibc-core contract class is too large
            "--dump-path",
            &Runtime::file_path_to_string(&chain_state_path),
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
                chain_command,
                &args,
                &[],
                Some(&stdout_path),
                Some(&stderr_path),
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(child_process)
    }
}
