use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use eyre::eyre;
use hermes_core::runtime_components::traits::CanReadFileAsString;
use hermes_cosmos::chain_components::types::{DynamicGasConfig, EipQueryType};
use hermes_cosmos::error::Error;
use hermes_cosmos::relayer::contexts::CosmosBuilder;
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use starknet::core::types::contract::SierraClass;

use crate::contexts::{OsmosisBootstrap, StarknetBootstrap, StarknetBootstrapFields};

pub async fn init_starknet_bootstrap(
    runtime: &HermesRuntime,
    test_uid: u64,
) -> Result<StarknetBootstrap, Error> {
    let chain_command_path = std::env::var("STARKNET_BIN")
        .unwrap_or("madara".into())
        .into();

    let erc20_contract = load_contract_from_env(runtime, "ERC20_CONTRACT").await?;

    let ics20_contract = load_contract_from_env(runtime, "ICS20_CONTRACT").await?;

    let ibc_core_contract = load_contract_from_env(runtime, "IBC_CORE_CONTRACT").await?;

    let comet_client_contract = load_contract_from_env(runtime, "COMET_CLIENT_CONTRACT").await?;

    let comet_lib_contract = load_contract_from_env(runtime, "COMET_LIB_CONTRACT").await?;

    let ics23_lib_contract = load_contract_from_env(runtime, "ICS23_LIB_CONTRACT").await?;

    let protobuf_lib_contract = load_contract_from_env(runtime, "PROTOBUF_LIB_CONTRACT").await?;

    let starknet_bootstrap = StarknetBootstrap {
        fields: Arc::new(StarknetBootstrapFields {
            runtime: runtime.clone(),
            chain_command_path,
            chain_store_dir: format!("./test-data/{test_uid}/starknet").into(),
            erc20_contract,
            ics20_contract,
            ibc_core_contract,
            comet_client_contract,
            comet_lib_contract,
            ics23_lib_contract,
            protobuf_lib_contract,
        }),
    };

    Ok(starknet_bootstrap)
}

pub async fn init_osmosis_bootstrap(
    runtime: &HermesRuntime,
    wasm_client_byte_code: Vec<u8>,
    wasm_additional_byte_code: Vec<Vec<u8>>,
    test_uid: u64,
) -> Result<OsmosisBootstrap, Error> {
    let cosmos_builder = CosmosBuilder::new_with_default(runtime.clone());

    let osmosis_bootstrap = OsmosisBootstrap {
        runtime: runtime.clone(),
        cosmos_builder,
        should_randomize_identifiers: true,
        chain_store_dir: format!("./test-data/{test_uid}/osmosis").into(),
        chain_command_path: "osmosisd".into(),
        account_prefix: "osmo".into(),
        staking_denom_prefix: "stake".into(),
        transfer_denom_prefix: "coin".into(),
        wasm_client_byte_code,
        wasm_additional_byte_code,
        governance_proposal_authority: "osmo10d07y265gmmuvt4z0w9aw880jnsr700jjeq4qp".into(), // TODO: don't hard code this
        dynamic_gas: Some(DynamicGasConfig {
            multiplier: 1.1,
            max: 1.6,
            eip_query_type: EipQueryType::Osmosis,
            denom: "stake".to_owned(),
        }),
    };

    Ok(osmosis_bootstrap)
}

pub async fn load_contract_from_env(
    runtime: &HermesRuntime,
    var: &str,
) -> Result<SierraClass, Error> {
    let contract_path = std::env::var(var)?;

    let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

    let contract = serde_json::from_str(&contract_str)?;

    Ok(contract)
}

// TODO: Use a lock to preoperly handle concurrent test runs
pub async fn create_test_uid() -> Result<u64, Error> {
    for _ in 0..60 {
        tokio::fs::create_dir_all("./test-data").await?;

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let test_path: PathBuf = format!("./test-data/{timestamp}").into();

        match tokio::fs::create_dir(&test_path).await {
            Ok(_) => {
                return Ok(timestamp);
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to create test data directory: {e}. Retrying after 1 second..."
                );
                tokio::time::sleep(core::time::Duration::from_secs(1)).await;
            }
        }
    }
    Err(eyre!("Failed to create test data directory after 60 attempts").into())
}
