#![recursion_limit = "256"]

use hermes_core::runtime_components::traits::CanSleep;
use hermes_cosmos::error::types::Error;
use hermes_cosmos::integration_tests::init::init_test_runtime;
use hermes_starknet_integration_tests::utils::init_starknet_test_driver;
use tracing::info;

fn main() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let test_driver = init_starknet_test_driver(&runtime).await?;

        info!("Starknet test driver initialized");

        info!("Starknet IBC info:");
        info!("  Client ID: {}", test_driver.client_id_a);
        info!("  Connection ID: {}", test_driver.connection_id_a);
        info!("  Channel ID: {}", test_driver.channel_id_a);
        info!("  Port ID: {}", test_driver.port_id_a);
        info!(
            "  Wallet A: {:?}",
            test_driver.starknet_chain_driver.user_wallet_a
        );
        info!(
            "  Wallet B: {:?}",
            test_driver.starknet_chain_driver.user_wallet_b
        );
        info!("  Node config:");
        info!(
            "    RPC: {}",
            test_driver.starknet_chain_driver.node_config.rpc_port,
        );

        info!("Osmosis IBC info:");
        info!("  Client ID: {}", test_driver.client_id_b);
        info!("  Connection ID: {}", test_driver.connection_id_b);
        info!("  Channel ID: {}", test_driver.channel_id_b);
        info!("  Port ID: {}", test_driver.port_id_b);
        info!(
            "  Wallet A: {:?}",
            test_driver.cosmos_chain_driver.user_wallet_a.keypair
        );
        info!(
            "  Wallet B: {:?}",
            test_driver.cosmos_chain_driver.user_wallet_b.keypair
        );
        info!("  Node config:");
        info!(
            "    RPC: {}",
            test_driver.cosmos_chain_driver.chain_node_config.rpc_port,
        );
        info!(
            "    gRPC: {}",
            test_driver.cosmos_chain_driver.chain_node_config.grpc_port,
        );

        runtime.sleep(core::time::Duration::from_secs(3600)).await;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
