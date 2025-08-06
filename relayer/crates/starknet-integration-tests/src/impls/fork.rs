use alloc::sync::Arc;
use core::str::FromStr;
use std::path::PathBuf;

use cgp::extra::runtime::HasRuntime;
use hermes_core::runtime_components::traits::{CanCreateDir, CanExecCommand, CanSleep};
use hermes_core::test_components::setup::traits::{FullNodeForker, FullNodeForkerComponent};
use hermes_cosmos::error::HermesError;
use hermes_cosmos::integration_tests::contexts::{
    CosmosBootstrap, CosmosBootstrapFields, CosmosChainDriver,
};
use hermes_cosmos::integration_tests::impls::copy_dir_recursive;
use hermes_cosmos::relayer::contexts::{CosmosBuilder, CosmosChain};
use hermes_cosmos::test_components::bootstrap::traits::CanStartChainFullNodes;
use hermes_prelude::*;
use tendermint_rpc::{HttpClient, Url};

use crate::contexts::{StarknetChainDriver, StarknetTestDriver};

#[cgp_new_provider(FullNodeForkerComponent)]
impl FullNodeForker<StarknetTestDriver> for ForkSecondFullNode {
    async fn fork_full_node(
        driver: &StarknetTestDriver,
    ) -> Result<StarknetTestDriver, HermesError> {
        // TODO
        // Retrieve necessary Cosmos full node data
        let genesis_config = driver.cosmos_chain_driver.genesis_config.clone();
        let chain_node_config = driver.cosmos_chain_driver.chain_node_config.clone();
        let chain_home_dir = driver
            .cosmos_chain_driver
            .chain_node_config
            .chain_home_dir
            .clone();

        let runtime = driver.cosmos_chain_driver.chain.runtime.clone();
        let builder = CosmosBuilder::new_with_default(runtime.clone());

        let node_bootstrap = CosmosBootstrap {
            fields: Arc::new(CosmosBootstrapFields {
                runtime: runtime.clone(),
                cosmos_builder: builder.clone(),
                should_randomize_identifiers: true,
                chain_store_dir: chain_home_dir.clone(),
                chain_command_path: driver.cosmos_chain_driver.chain_command_path.clone(),
                account_prefix: driver
                    .cosmos_chain_driver
                    .chain
                    .chain_config
                    .account_prefix
                    .clone(),
                staking_denom_prefix: driver
                    .cosmos_chain_driver
                    .genesis_config
                    .staking_denom
                    .to_string(),
                transfer_denom_prefix: driver
                    .cosmos_chain_driver
                    .genesis_config
                    .transfer_denom
                    .to_string(),
                genesis_config_modifier: Box::new(|_| Ok(())),
                comet_config_modifier: Box::new(|_| Ok(())),
                dynamic_gas: driver
                    .cosmos_chain_driver
                    .chain
                    .chain_config
                    .gas_config
                    .dynamic_gas_config
                    .clone(),
            }),
        };

        // Stop full node
        // `pkill` is used here instead of `Child::kill()` as the `kill()` method requires
        // the child process to be mutable.
        runtime
            .exec_command(
                &PathBuf::from("pkill".to_string()),
                &["-f", &driver.cosmos_chain_driver.chain.chain_id.to_string()],
            )
            .await?;

        driver
            .relay_driver_a_b
            .birelay
            .runtime()
            .sleep(core::time::Duration::from_secs(5))
            .await;

        // Build forked full node data
        let fork_chain_home_dir = chain_home_dir
            .as_path()
            .parent()
            .expect("failed to retrieve parent path of the chain home directory")
            .join(format!(
                "fork-{}",
                driver.cosmos_chain_driver.chain.chain_id
            ));
        let mut fork_chain_node_config = chain_node_config.clone();
        fork_chain_node_config.chain_home_dir = fork_chain_home_dir.clone();
        fork_chain_node_config.rpc_port += 1;
        fork_chain_node_config.p2p_port += 1;
        fork_chain_node_config.grpc_port += 1;
        let fork_rpc_port = fork_chain_node_config.rpc_port;
        let fork_p2p_port = fork_chain_node_config.p2p_port;

        let fork_bootstrap = CosmosBootstrap {
            fields: Arc::new(CosmosBootstrapFields {
                runtime: runtime.clone(),
                cosmos_builder: builder.clone(),
                should_randomize_identifiers: true,
                chain_store_dir: fork_chain_home_dir.clone(),
                chain_command_path: driver.cosmos_chain_driver.chain_command_path.clone(),
                account_prefix: driver
                    .cosmos_chain_driver
                    .chain
                    .chain_config
                    .account_prefix
                    .clone(),
                staking_denom_prefix: driver
                    .cosmos_chain_driver
                    .genesis_config
                    .staking_denom
                    .to_string(),
                transfer_denom_prefix: driver
                    .cosmos_chain_driver
                    .genesis_config
                    .transfer_denom
                    .to_string(),
                genesis_config_modifier: Box::new(|_| Ok(())),
                comet_config_modifier: Box::new(|_| Ok(())),
                dynamic_gas: driver
                    .cosmos_chain_driver
                    .chain
                    .chain_config
                    .gas_config
                    .dynamic_gas_config
                    .clone(),
            }),
        };

        // Create forked full node directory and copy full node data inside
        runtime.create_dir(&fork_chain_home_dir).await?;

        // Copy data to fork
        copy_dir_recursive(&chain_home_dir, &fork_chain_home_dir)?;

        let fork_chain_config_path = fork_chain_home_dir.join("config").join("config.toml");

        let fork_chain_config = std::fs::read_to_string(fork_chain_config_path.clone())
            .expect("failed to read fork config.toml");

        let mut toml_value: toml::Table = fork_chain_config.parse()?;

        // Update RPC and P2P addresses to avoid collision
        toml_value
            .get_mut("rpc")
            .and_then(|rpc| rpc.as_table_mut())
            .expect("Failed to retrieve `rpc` in node configuration")
            .insert(
                "laddr".to_string(),
                toml::Value::String(format!("tcp://0.0.0.0:{fork_rpc_port}")),
            );
        toml_value
            .get_mut("p2p")
            .and_then(|p2p| p2p.as_table_mut())
            .expect("Failed to retrieve `p2p` in node configuration")
            .insert(
                "laddr".to_string(),
                toml::Value::String(format!("tcp://0.0.0.0:{fork_p2p_port}")),
            );

        std::fs::write(fork_chain_config_path, toml::to_string(&toml_value)?)?;

        // Start the forked chain full node in the background, and return the child process handle
        let mut chain_processes = fork_bootstrap
            .start_chain_full_nodes(
                &fork_chain_home_dir,
                &fork_chain_node_config,
                &genesis_config,
            )
            .await?;

        driver
            .relay_driver_a_b
            .birelay
            .runtime()
            .sleep(core::time::Duration::from_secs(1))
            .await;

        let mut node_chain_processes = node_bootstrap
            .start_chain_full_nodes(&chain_home_dir, &chain_node_config, &genesis_config)
            .await?;

        chain_processes.append(&mut node_chain_processes);

        let mut fork_b_chain_config = driver.cosmos_chain_driver.chain.chain_config.clone();

        let fork_b_grpc_url_str = format!(
            "{}://{}:{}",
            driver
                .cosmos_chain_driver
                .chain
                .chain_config
                .grpc_addr
                .scheme(),
            driver
                .cosmos_chain_driver
                .chain
                .chain_config
                .grpc_addr
                .host(),
            fork_chain_node_config.grpc_port
        );
        let fork_b_rpc_url_str = format!(
            "{}://{}:{}",
            driver
                .cosmos_chain_driver
                .chain
                .chain_config
                .rpc_addr
                .scheme(),
            driver
                .cosmos_chain_driver
                .chain
                .chain_config
                .rpc_addr
                .host(),
            fork_chain_node_config.rpc_port
        );

        fork_b_chain_config.grpc_addr = Url::from_str(&fork_b_grpc_url_str)?;
        fork_b_chain_config.rpc_addr = Url::from_str(&fork_b_rpc_url_str)?;

        let mut fork_b_rpc_client = HttpClient::new(fork_b_chain_config.rpc_addr.clone())?;
        fork_b_rpc_client.set_compat_mode(driver.cosmos_chain_driver.chain.compat_mode);

        let forked_chain_b = CosmosChain::new(
            fork_b_chain_config,
            fork_b_rpc_client,
            driver.cosmos_chain_driver.chain.compat_mode,
            driver.cosmos_chain_driver.chain.key_entries.clone(),
            driver.cosmos_chain_driver.chain.runtime.clone(),
            driver.cosmos_chain_driver.chain.telemetry.clone(),
            driver.cosmos_chain_driver.chain.packet_filter.clone(),
        );

        let forked_starknet_chain_driver = StarknetChainDriver {
            runtime: driver.starknet_chain_driver.runtime.clone(),
            chain: driver.starknet_chain_driver.chain.clone(),
            chain_store_dir: driver.starknet_chain_driver.chain_store_dir.clone(),
            genesis_config: driver.starknet_chain_driver.genesis_config.clone(),
            node_config: driver.starknet_chain_driver.node_config.clone(),
            wallets: driver.starknet_chain_driver.wallets.clone(),
            chain_processes: vec![],
            relayer_wallet_1: driver.starknet_chain_driver.relayer_wallet_1.clone(),
            relayer_wallet_2: driver.starknet_chain_driver.relayer_wallet_2.clone(),
            user_wallet_a: driver.starknet_chain_driver.user_wallet_a.clone(),
            user_wallet_b: driver.starknet_chain_driver.user_wallet_b.clone(),
        };

        let forked_cosmos_chain_driver = CosmosChainDriver {
            chain: forked_chain_b,
            chain_command_path: driver.cosmos_chain_driver.chain_command_path.clone(),
            chain_node_config: fork_chain_node_config,
            genesis_config,
            chain_processes,
            validator_wallet: driver.cosmos_chain_driver.validator_wallet.clone(),
            relayer_wallet: driver.cosmos_chain_driver.relayer_wallet.clone(),
            user_wallet_a: driver.cosmos_chain_driver.user_wallet_a.clone(),
            user_wallet_b: driver.cosmos_chain_driver.user_wallet_b.clone(),
            wallets: driver.cosmos_chain_driver.wallets.clone(),
        };

        Ok(StarknetTestDriver {
            relay_driver_a_b: driver.relay_driver_a_b.clone(),
            relay_driver_b_a: driver.relay_driver_b_a.clone(),
            starknet_chain_driver: forked_starknet_chain_driver,
            cosmos_chain_driver: forked_cosmos_chain_driver,
            client_id_a: driver.client_id_a.clone(),
            client_id_b: driver.client_id_b.clone(),
            connection_id_a: driver.connection_id_a.clone(),
            connection_id_b: driver.connection_id_b.clone(),
            channel_id_a: driver.channel_id_a.clone(),
            channel_id_b: driver.channel_id_b.clone(),
            port_id_a: driver.port_id_a.clone(),
            port_id_b: driver.port_id_b.clone(),
            create_client_payload_options_a: driver.create_client_payload_options_a.clone(),
            create_client_payload_options_b: driver.create_client_payload_options_b.clone(),
            create_client_message_options_a: (),
            create_client_message_options_b: (),
            recover_client_payload_options_a:
                hermes_starknet_chain_components::impls::StarknetRecoverClientPayload,
            recover_client_payload_options_b: driver.recover_client_payload_options_b.clone(),
        })
    }
}
