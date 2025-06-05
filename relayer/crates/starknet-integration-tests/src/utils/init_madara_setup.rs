use hermes_core::test_components::setup::traits::CanBuildTestDriver;
use hermes_cosmos::error::Error;
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use hermes_prelude::CanRaiseError;
use hermes_starknet_madara_tests::impls::init_madara_bootstrap;
use hermes_starknet_relayer::contexts::MadaraBuilder;

use crate::contexts::{MadaraTestDriver, MadaraTestSetup};
use crate::utils::{init_osmosis_bootstrap, load_wasm_client};

pub async fn init_madara_setup(runtime: &HermesRuntime) -> Result<MadaraTestSetup, Error> {
    let wasm_client_code_path = std::env::var("STARKNET_WASM_CLIENT_PATH").map_err(|_| {
        MadaraTestSetup::raise_error("Wasm blob for Starknet light client is required")
    })?;

    let (wasm_code_hash, wasm_client_byte_code) = load_wasm_client(&wasm_client_code_path).await?;

    let starknet_bootstrap = init_madara_bootstrap(runtime).await?;

    let cosmos_bootstrap = init_osmosis_bootstrap(runtime, wasm_client_byte_code).await?;

    let starknet_builder = MadaraBuilder::new(
        runtime.clone(),
        cosmos_bootstrap.cosmos_builder.clone(),
        None,
    );

    let setup = MadaraTestSetup::new_with_defaults(
        starknet_bootstrap,
        cosmos_bootstrap,
        starknet_builder,
        wasm_code_hash,
    );

    Ok(setup)
}

pub async fn init_madara_test_driver(runtime: &HermesRuntime) -> Result<MadaraTestDriver, Error> {
    let setup = init_madara_setup(runtime).await?;

    let test_driver: MadaraTestDriver = setup.build_driver().await?;

    Ok(test_driver)
}
