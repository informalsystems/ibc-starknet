use alloc::sync::Arc;
use std::path::PathBuf;

use futures::lock::Mutex;
use hermes_core::runtime_components::traits::{CanCreateDir, CanExecCommand, CanSleep};
use hermes_core::test_components::test_case::traits::node::{
    FullNodeHalter, FullNodeHalterComponent, FullNodeResumer, FullNodeResumerComponent,
};
use hermes_cosmos::error::HermesError;
use hermes_cosmos::integration_tests::impls::copy_dir_recursive;
use hermes_cosmos::test_components::bootstrap::traits::CanStartChainFullNodes;
use hermes_prelude::*;
use hermes_starknet_chain_context::contexts::{StarknetChain, StarknetChainFields};
use hermes_starknet_test_components::traits::CanStartChainForkedFullNodes;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;
use starknet_crypto::Felt;

use crate::contexts::{StarknetBootstrap, StarknetBootstrapFields, StarknetChainDriver};
use crate::utils::load_contract_from_env;

pub struct StarknetFullNodeResumeOptions {
    pub sequencer_private_key: Felt,
}

pub struct StarknetFullNodeHandler;

#[cgp_provider(FullNodeHalterComponent)]
impl FullNodeHalter<StarknetChainDriver> for StarknetFullNodeHandler {
    async fn halt_full_node(chain_driver: &StarknetChainDriver) -> Result<(), HermesError> {
        let runtime = chain_driver.chain.runtime.clone();
        let node_pid = chain_driver
            .chain_processes
            .first()
            .expect("Failed to retrieve Starknet chain process")
            .id()
            .expect("failed to retrieve Starknet chain process ID");

        // Stop full node
        // `kill` is used here instead of `Child::kill()` as the `kill()` method requires
        // the child process to be mutable.
        runtime
            .exec_command(
                &PathBuf::from("kill".to_string()),
                &["-s", "KILL", &node_pid.to_string()],
            )
            .await?;

        runtime.sleep(core::time::Duration::from_secs(3)).await;

        Ok(())
    }
}

#[cgp_provider(FullNodeResumerComponent)]
impl FullNodeResumer<StarknetChainDriver> for StarknetFullNodeHandler {
    async fn resume_full_node(
        chain_driver: &StarknetChainDriver,
        options: &StarknetFullNodeResumeOptions,
    ) -> Result<StarknetChainDriver, HermesError> {
        let runtime = chain_driver.chain.runtime.clone();
        let chain_home_dir = chain_driver.chain_store_dir.clone();
        let chain_node_config = chain_driver.node_config.clone();

        // Build forked full node data
        let fork_chain_home_dir = chain_home_dir.as_path().join("fork-madara");
        let mut fork_chain_node_config = chain_node_config.clone();
        fork_chain_node_config.sequencer_private_key = options.sequencer_private_key;
        fork_chain_node_config.rpc_port += 20;
        let fork_rpc_port = fork_chain_node_config.rpc_port;

        // Create forked full node directory and copy full node data inside
        runtime.create_dir(&fork_chain_home_dir).await?;

        let chain_command_path: PathBuf = std::env::var("STARKNET_BIN")
            .unwrap_or("madara".into())
            .into();

        let erc20_contract = load_contract_from_env(&runtime, "ERC20_CONTRACT").await?;

        let ics20_contract = load_contract_from_env(&runtime, "ICS20_CONTRACT").await?;

        let ibc_core_contract = load_contract_from_env(&runtime, "IBC_CORE_CONTRACT").await?;

        let comet_client_contract =
            load_contract_from_env(&runtime, "COMET_CLIENT_CONTRACT").await?;

        let comet_lib_contract = load_contract_from_env(&runtime, "COMET_LIB_CONTRACT").await?;

        let ics23_lib_contract = load_contract_from_env(&runtime, "ICS23_LIB_CONTRACT").await?;

        let protobuf_lib_contract =
            load_contract_from_env(&runtime, "PROTOBUF_LIB_CONTRACT").await?;

        let node_bootstrap = StarknetBootstrap {
            fields: Arc::new(StarknetBootstrapFields {
                runtime: runtime.clone(),
                chain_command_path: chain_command_path.clone(),
                chain_store_dir: chain_home_dir.clone(),
                erc20_contract: erc20_contract.clone(),
                ics20_contract: ics20_contract.clone(),
                ibc_core_contract: ibc_core_contract.clone(),
                comet_client_contract: comet_client_contract.clone(),
                comet_lib_contract: comet_lib_contract.clone(),
                ics23_lib_contract: ics23_lib_contract.clone(),
                protobuf_lib_contract: protobuf_lib_contract.clone(),
            }),
        };

        let fork_bootstrap = StarknetBootstrap {
            fields: Arc::new(StarknetBootstrapFields {
                runtime: runtime.clone(),
                chain_command_path,
                chain_store_dir: fork_chain_home_dir.clone(),
                erc20_contract,
                ics20_contract,
                ibc_core_contract,
                comet_client_contract,
                comet_lib_contract,
                ics23_lib_contract,
                protobuf_lib_contract,
            }),
        };

        let starknet_chain_home_dir = chain_home_dir.as_path().join("starknet");

        // Copy data to fork
        copy_dir_recursive(&starknet_chain_home_dir, &fork_chain_home_dir)?;

        // Start the forked chain full node in the background, and return the child process handle
        let mut forked_chain_processes = fork_bootstrap
            .start_chain_forked_full_nodes(
                &fork_chain_home_dir,
                &fork_chain_node_config,
                &chain_driver.genesis_config,
                &starknet_chain_home_dir,
                "10", // FIXME: Retrieve the correct block number for `--backup-every-n-blocks`
            )
            .await?;

        chain_driver
            .runtime
            .sleep(core::time::Duration::from_secs(1))
            .await;

        let mut node_chain_processes = node_bootstrap
            .start_chain_full_nodes(
                &chain_home_dir,
                &chain_node_config,
                &chain_driver.genesis_config,
            )
            .await?;

        forked_chain_processes.append(&mut node_chain_processes);

        let mut forked_json_rpc_url = chain_driver.chain.json_rpc_url.clone();
        let current_port = forked_json_rpc_url
            .port()
            .expect("Failed to extract port from JSON url");
        forked_json_rpc_url
            .set_port(Some(current_port + 20))
            .expect("Failed to set port");

        let forked_starknet_rpc_client = Arc::new(JsonRpcClient::new(HttpTransport::new(
            forked_json_rpc_url.clone(),
        )));

        let rpc_client = ureq::agent();

        let mut forked_feeder_gateway_url = chain_driver.chain.feeder_gateway_url.clone();
        let current_port = forked_feeder_gateway_url
            .port()
            .expect("Failed to extract port from Feeder gateway url");
        forked_feeder_gateway_url
            .set_port(Some(current_port + 20))
            .expect("Failed to set port");

        let forked_starknet_chain = StarknetChain {
            fields: Arc::new(StarknetChainFields {
                runtime: runtime.clone(),
                chain_id: chain_driver.chain.chain_id.clone(),
                starknet_client: forked_starknet_rpc_client,
                rpc_client,
                json_rpc_url: forked_json_rpc_url,
                feeder_gateway_url: forked_feeder_gateway_url,
                ibc_client_contract_address: chain_driver.chain.ibc_client_contract_address.clone(),
                ibc_core_contract_address: chain_driver.chain.ibc_core_contract_address.clone(),
                ibc_ics20_contract_address: chain_driver.chain.ibc_ics20_contract_address.clone(),
                event_encoding: chain_driver.chain.event_encoding.clone(),
                poll_interval: chain_driver.chain.poll_interval,
                block_time: chain_driver.chain.block_time,
                nonce_mutex: Arc::new(Mutex::new(())),
                signers: chain_driver.chain.signers.clone(),
                client_refresh_rate: chain_driver.chain.client_refresh_rate,
                signer_mutex: Arc::new(Mutex::new(0)),
            }),
        };

        Ok(StarknetChainDriver {
            runtime: chain_driver.runtime.clone(),
            chain: forked_starknet_chain,
            chain_store_dir: fork_chain_home_dir,
            chain_command_path: chain_driver.chain_command_path.clone(),
            genesis_config: chain_driver.genesis_config.clone(),
            node_config: fork_chain_node_config,
            wallets: chain_driver.wallets.clone(),
            chain_processes: forked_chain_processes,
            relayer_wallet_1: chain_driver.relayer_wallet_1.clone(),
            relayer_wallet_2: chain_driver.relayer_wallet_2.clone(),
            user_wallet_a: chain_driver.user_wallet_a.clone(),
            user_wallet_b: chain_driver.user_wallet_b.clone(),
        })
    }
}
