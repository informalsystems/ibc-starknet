use cgp::prelude::CanRaiseError;
use hermes_core::test_components::setup::traits::CanBuildTestDriver;
use hermes_error::Error;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_starknet_relayer::contexts::builder::StarknetBuilder;

use crate::contexts::setup::StarknetTestSetup;
use crate::contexts::test_driver::StarknetTestDriver;
use crate::utils::{init_osmosis_bootstrap, init_starknet_bootstrap, load_wasm_client};

pub async fn init_starknet_setup(runtime: &HermesRuntime) -> Result<StarknetTestSetup, Error> {
    let wasm_client_code_path = std::env::var("STARKNET_WASM_CLIENT_PATH").map_err(|_| {
        StarknetTestSetup::raise_error("Wasm blob for Starknet light client is required")
    })?;

    let (wasm_code_hash, wasm_client_byte_code) = load_wasm_client(&wasm_client_code_path).await?;

    let starknet_bootstrap = init_starknet_bootstrap(runtime).await?;

    let cosmos_bootstrap = init_osmosis_bootstrap(runtime, wasm_client_byte_code).await?;

    let starknet_builder = StarknetBuilder::new(
        runtime.clone(),
        cosmos_bootstrap.cosmos_builder.clone(),
        None,
    );

    let setup = StarknetTestSetup::new_with_defaults(
        starknet_bootstrap,
        cosmos_bootstrap,
        starknet_builder,
        wasm_code_hash,
    );

    Ok(setup)
}

pub async fn init_starknet_test_driver(
    runtime: &HermesRuntime,
) -> Result<StarknetTestDriver, Error> {
    let setup = init_starknet_setup(runtime).await?;

    let test_driver: StarknetTestDriver = setup.build_driver().await?;

    Ok(test_driver)
}
