use std::time::SystemTime;

use hermes_error::Error;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;

use crate::contexts::bootstrap::StarknetBootstrap;

pub async fn init_starknet_bootstrap(runtime: &HermesRuntime) -> Result<StarknetBootstrap, Error> {
    let chain_command_path = std::env::var("STARKNET_BIN")
        .unwrap_or("starknet-devnet".into())
        .into();

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let erc20_contract = {
        let contract_path = std::env::var("ERC20_CONTRACT")?;

        let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

        serde_json::from_str(&contract_str)?
    };

    let ics20_contract = {
        let contract_path = std::env::var("ICS20_CONTRACT")?;

        let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

        serde_json::from_str(&contract_str)?
    };

    let starknet_bootstrap = StarknetBootstrap {
        runtime: runtime.clone(),
        chain_command_path,
        chain_store_dir: format!("./test-data/{timestamp}").into(),
        erc20_contract,
        ics20_contract,
    };

    Ok(starknet_bootstrap)
}
