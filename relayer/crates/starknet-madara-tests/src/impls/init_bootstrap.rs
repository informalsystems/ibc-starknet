use std::sync::Arc;
use std::time::SystemTime;

use hermes_core::runtime_components::traits::CanReadFileAsString;
use hermes_error::Error;
use hermes_runtime::types::runtime::HermesRuntime;
use starknet::core::types::contract::SierraClass;

use crate::contexts::{MadaraBootstrap, MadaraBootstrapFields};

pub async fn init_madara_bootstrap(runtime: &HermesRuntime) -> Result<MadaraBootstrap, Error> {
    let chain_command_path = std::env::var("STARKNET_BIN")
        .unwrap_or("madara".into())
        .into();

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let erc20_contract = load_contract_from_env(runtime, "ERC20_CONTRACT").await?;

    let ics20_contract = load_contract_from_env(runtime, "ICS20_CONTRACT").await?;

    let ibc_core_contract = load_contract_from_env(runtime, "IBC_CORE_CONTRACT").await?;

    let comet_client_contract = load_contract_from_env(runtime, "COMET_CLIENT_CONTRACT").await?;

    let comet_lib_contract = load_contract_from_env(runtime, "COMET_LIB_CONTRACT").await?;

    let ics23_lib_contract = load_contract_from_env(runtime, "ICS23_LIB_CONTRACT").await?;

    let protobuf_lib_contract = load_contract_from_env(runtime, "PROTOBUF_LIB_CONTRACT").await?;

    let starknet_bootstrap = MadaraBootstrap {
        fields: Arc::new(MadaraBootstrapFields {
            runtime: runtime.clone(),
            chain_command_path,
            chain_store_dir: format!("./test-data/{timestamp}").into(),
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

pub async fn load_contract_from_env(
    runtime: &HermesRuntime,
    var: &str,
) -> Result<SierraClass, Error> {
    let contract_path = std::env::var(var)?;

    let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

    let contract = serde_json::from_str(&contract_str)?;

    Ok(contract)
}
