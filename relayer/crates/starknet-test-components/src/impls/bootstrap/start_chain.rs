use hermes_core::runtime_components::traits::{CanStartChildProcess, HasFilePathType, HasRuntime};
use hermes_cosmos_core::test_components::bootstrap::traits::{
    ChainFullNodeStarter, ChainFullNodeStarterComponent, HasChainCommandPath,
    HasChainGenesisConfigType, HasChainNodeConfigType,
};
use hermes_prelude::*;

use crate::types::{StarknetGenesisConfig, StarknetNodeConfig};

#[cgp_new_provider(ChainFullNodeStarterComponent)]
impl<Bootstrap, Runtime> ChainFullNodeStarter<Bootstrap> for StartStarknetDevnet
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainNodeConfigType<ChainNodeConfig = StarknetNodeConfig>
        + HasChainGenesisConfigType<ChainGenesisConfig = StarknetGenesisConfig>
        + HasChainCommandPath
        + CanRaiseAsyncError<Runtime::Error>,
    Runtime: CanStartChildProcess + HasFilePathType,
{
    async fn start_chain_full_nodes(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_node_config: &StarknetNodeConfig,
        chain_genesis_config: &StarknetGenesisConfig,
    ) -> Result<Vec<Runtime::ChildProcess>, Bootstrap::Error> {
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
            "--initial-balance=1000000000000000000000000000000", // need large balance
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

        Ok(vec![child_process])
    }
}
