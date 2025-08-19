use hermes_core::runtime_components::traits::{CanCreateDir, CanStartChildProcess, HasRuntime};
use hermes_core::test_components::chain_driver::traits::HasChainCommandPath;
use hermes_cosmos_core::test_components::bootstrap::traits::{
    HasChainGenesisConfigType, HasChainNodeConfigType,
};
use hermes_prelude::*;

use crate::traits::{ChainForkedFullNodeStarter, ChainForkedFullNodeStarterComponent};
use crate::types::StarknetNodeConfig;

#[cgp_new_provider(ChainForkedFullNodeStarterComponent)]
impl<Bootstrap, Runtime> ChainForkedFullNodeStarter<Bootstrap> for StartStarknetForkedSequencer
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainNodeConfigType<ChainNodeConfig = StarknetNodeConfig>
        + HasChainGenesisConfigType
        + HasChainCommandPath
        + CanRaiseAsyncError<Runtime::Error>,
    Runtime: CanStartChildProcess + CanCreateDir,
{
    async fn start_chain_forked_full_nodes(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_node_config: &StarknetNodeConfig,
        chain_genesis_config: &Bootstrap::ChainGenesisConfig,
        backup_dir: &Runtime::FilePath,
        number_of_blocks: &str,
    ) -> Result<Vec<Runtime::ChildProcess>, Bootstrap::Error> {
        let chain_command = bootstrap.chain_command_path();

        let starknet_home =
            Runtime::join_file_path(chain_home_dir, &Runtime::file_path_from_string("starknet"));

        bootstrap
            .runtime()
            .create_dir(&starknet_home)
            .await
            .map_err(Bootstrap::raise_error)?;

        let rpc_port = chain_node_config.rpc_port;
        let sequencer_private_key = chain_node_config.sequencer_private_key;

        let gateway_port = rpc_port + 1;

        let args = [
            "--base-path",
            &Runtime::file_path_to_string(&starknet_home),
            "--rpc-port",
            &rpc_port.to_string(),
            "--gateway-port",
            &gateway_port.to_string(),
            "--chain-config-override",
            "block_time=1s,pending_block_update_time=1s,chain_id=IBC_SN_DEVNET,latest_protocol_version=0.14.0",
            "--devnet",
            "--devnet-unsafe",
            "--gateway-enable",
            "--feeder-gateway-enable",
            "--rpc-storage-proof-max-distance",
            "300", // can generate storage proof for the last 300 blocks
            "--preset",
            "sepolia",
            "--l1-sync-disabled",
            "--l2-sync-disabled",
            "--l1-gas-price",
            "0",
            "--blob-gas-price",
            "0",
            "--private-key",
            &sequencer_private_key.to_hex_string(),
            "--backup-dir",
            &Runtime::file_path_to_string(backup_dir),
            "--restore-from-latest-backup",
            "--backup-every-n-blocks",
            number_of_blocks,
        ];

        let stdout_path = Runtime::join_file_path(
            &starknet_home,
            &Runtime::file_path_from_string("stdout.log"),
        );

        let stderr_path = Runtime::join_file_path(
            &starknet_home,
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
