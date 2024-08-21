use std::env::var;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_wasm_relayer::context::cosmos_bootstrap::CosmosWithWasmClientBootstrap;
use hermes_error::types::Error;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use sha2::{Digest, Sha256};

#[test]
fn test_starknet_light_client() -> Result<(), Error> {
    let runtime = init_test_runtime();

    let store_postfix = format!(
        "{}-{}",
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),
        rand::random::<u64>()
    );

    let store_dir = std::env::current_dir()?.join(format!("test-data/{store_postfix}"));

    let wasm_client_code_path = PathBuf::from(
        var("WASM_STARKNET_CLIENT_PATH").expect("Wasm blob for Starknet light client is required"),
    );

    let cosmos_builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    runtime.runtime.clone().block_on(async move {
        let wasm_client_byte_code = tokio::fs::read(&wasm_client_code_path).await?;

        let _wasm_code_hash: [u8; 32] = {
            let mut hasher = Sha256::new();
            hasher.update(&wasm_client_byte_code);
            hasher.finalize().into()
        };

        let cosmos_bootstrap = Arc::new(CosmosWithWasmClientBootstrap {
            runtime: runtime.clone(),
            builder: cosmos_builder,
            should_randomize_identifiers: true,
            chain_store_dir: store_dir.join("chains"),
            chain_command_path: "simd".into(),
            account_prefix: "cosmos".into(),
            staking_denom: "stake".into(),
            transfer_denom: "coin".into(),
            wasm_client_byte_code,
            governance_proposal_authority: "cosmos10d07y265gmmuvt4z0w9aw880jnsr700j6zn9kn".into(), // TODO: don't hard code this
        });

        let _cosmos_chain_driver = cosmos_bootstrap.bootstrap_chain("cosmos").await?;

        Ok(())
    })
}
