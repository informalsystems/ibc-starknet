use std::sync::Arc;
use std::time::SystemTime;

use hermes_core::runtime_components::traits::CanReadFileAsString;
use hermes_error::Error;
use hermes_runtime::types::runtime::HermesRuntime;
use starknet_v13::core::types::contract::SierraClass;

use crate::contexts::{MadaraBootstrap, MadaraBootstrapFields};

pub async fn init_madara_bootstrap(runtime: &HermesRuntime) -> Result<MadaraBootstrap, Error> {
    let chain_command_path = std::env::var("STARKNET_BIN")
        .unwrap_or("madara".into())
        .into();

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let starknet_bootstrap = MadaraBootstrap {
        fields: Arc::new(MadaraBootstrapFields {
            runtime: runtime.clone(),
            chain_command_path,
            chain_store_dir: format!("./test-data/{timestamp}").into(),
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
