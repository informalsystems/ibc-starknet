use std::time::SystemTime;

use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_chain_components::types::config::gas::eip_type::EipQueryType;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_error::types::Error;
use hermes_ibc_test_suite::tests::transfer::TestIbcTransfer;
use hermes_starknet_relayer::contexts::builder::StarknetBuilder;
use hermes_test_components::setup::traits::driver::CanBuildTestDriver;
use hermes_test_components::test_case::traits::test_case::TestCase;

use crate::contexts::osmosis_bootstrap::OsmosisBootstrap;
use crate::contexts::setup::StarknetTestSetup;
use crate::contexts::test_driver::StarknetTestDriver;
use crate::utils::{init_starknet_bootstrap, load_wasm_client};

#[test]
fn test_starknet_ics20_contract() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let wasm_client_code_path = std::env::var("STARKNET_WASM_CLIENT_PATH")
            .expect("Wasm blob for Starknet light client is required");

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let starknet_bootstrap = init_starknet_bootstrap(&runtime).await?;

        let (wasm_code_hash, wasm_client_byte_code) =
            load_wasm_client(&wasm_client_code_path).await?;

        let cosmos_builder = CosmosBuilder::new_with_default(runtime.clone());

        let starknet_builder = StarknetBuilder::new(runtime.clone(), cosmos_builder.clone(), None);

        let cosmos_bootstrap = OsmosisBootstrap {
            runtime: runtime.clone(),
            cosmos_builder,
            should_randomize_identifiers: true,
            chain_store_dir: format!("./test-data/{timestamp}/osmosis").into(),
            chain_command_path: "osmosisd".into(),
            account_prefix: "osmo".into(),
            staking_denom_prefix: "stake".into(),
            transfer_denom_prefix: "coin".into(),
            wasm_client_byte_code,
            governance_proposal_authority: "osmo10d07y265gmmuvt4z0w9aw880jnsr700jjeq4qp".into(), // TODO: don't hard code this
            dynamic_gas: Some(DynamicGasConfig {
                multiplier: 1.1,
                max: 1.6,
                eip_query_type: EipQueryType::Osmosis,
                denom: "stake".to_owned(),
            }),
        };

        let setup = StarknetTestSetup::new_with_defaults(
            starknet_bootstrap,
            cosmos_bootstrap,
            starknet_builder,
            wasm_code_hash,
        );

        let test_driver: StarknetTestDriver = setup.build_driver().await?;

        TestIbcTransfer.run_test(&test_driver).await?;

        Ok(())
    })
}
