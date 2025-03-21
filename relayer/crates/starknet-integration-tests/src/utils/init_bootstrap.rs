use std::time::SystemTime;

use hermes_error::Error;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use starknet::core::types::contract::SierraClass;

use crate::contexts::starknet_bootstrap::StarknetBootstrap;

pub async fn init_starknet_bootstrap(runtime: &HermesRuntime) -> Result<StarknetBootstrap, Error> {
    let chain_command_path = std::env::var("STARKNET_BIN")
        .unwrap_or("starknet-devnet".into())
        .into();

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let erc20_contract = load_contract_from_env(runtime, "ERC20_CONTRACT").await?;

    let ics20_contract = load_contract_from_env(runtime, "ICS20_CONTRACT").await?;

    let ibc_core_contract = load_contract_from_env(runtime, "IBC_CORE_CONTRACT").await?;

    let comet_client_contract = load_contract_from_env(runtime, "COMET_CLIENT_CONTRACT").await?;

    let starknet_bootstrap = StarknetBootstrap {
        runtime: runtime.clone(),
        chain_command_path,
        chain_store_dir: format!("./test-data/{timestamp}").into(),
        erc20_contract,
        ics20_contract,
        ibc_core_contract,
        comet_client_contract,
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
