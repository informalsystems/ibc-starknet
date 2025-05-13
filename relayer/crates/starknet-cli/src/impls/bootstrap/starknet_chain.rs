use std::sync::Arc;

use hermes_cli_components::traits::{BootstrapLoader, BootstrapLoaderComponent, HasBootstrapType};
use hermes_core::runtime_components::traits::{CanReadFileAsString, HasRuntime};
use hermes_cosmos::runtime::types::error::TokioRuntimeError;
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use hermes_prelude::*;
use hermes_starknet_integration_tests::contexts::{StarknetBootstrap, StarknetBootstrapFields};

#[derive(Debug, clap::Parser, HasField)]
pub struct BootstrapStarknetChainArgs {
    #[clap(long = "chain-id", required = true)]
    pub chain_id: String,

    #[clap(long = "chain-store-dir", required = true)]
    pub chain_store_dir: String,

    #[clap(long = "chain-command-path", default_value = "starknet-devnet")]
    pub chain_command_path: String,

    #[clap(long = "erc20-contract-path")]
    pub erc20_contract_path: String,

    #[clap(long = "ics20-contract-path")]
    pub ics20_contract_path: String,

    #[clap(long = "ibc-core-contract-path")]
    pub ibc_core_contract_path: String,

    #[clap(long = "comet-client-contract-path")]
    pub comet_client_contract_path: String,
}

pub struct LoadStarknetBootstrap;

#[cgp_provider(BootstrapLoaderComponent)]
impl<App, Tag> BootstrapLoader<App, Tag, BootstrapStarknetChainArgs> for LoadStarknetBootstrap
where
    App: HasBootstrapType<Tag, Bootstrap = StarknetBootstrap>
        + HasRuntime<Runtime = HermesRuntime>
        + CanRaiseAsyncError<serde_json::Error>
        + CanRaiseAsyncError<TokioRuntimeError>,
{
    async fn load_bootstrap(
        app: &App,
        args: &BootstrapStarknetChainArgs,
    ) -> Result<App::Bootstrap, App::Error> {
        let runtime = app.runtime();

        let erc20_contract = {
            let contract_str = runtime
                .read_file_as_string(&args.erc20_contract_path.clone().into())
                .await
                .map_err(App::raise_error)?;

            serde_json::from_str(&contract_str).map_err(App::raise_error)?
        };

        let ics20_contract = {
            let contract_str = runtime
                .read_file_as_string(&args.ics20_contract_path.clone().into())
                .await
                .map_err(App::raise_error)?;

            serde_json::from_str(&contract_str).map_err(App::raise_error)?
        };

        let ibc_core_contract = {
            let contract_str = runtime
                .read_file_as_string(&args.ibc_core_contract_path.clone().into())
                .await
                .map_err(App::raise_error)?;

            serde_json::from_str(&contract_str).map_err(App::raise_error)?
        };

        let comet_client_contract = {
            let contract_str = runtime
                .read_file_as_string(&args.comet_client_contract_path.clone().into())
                .await
                .map_err(App::raise_error)?;

            serde_json::from_str(&contract_str).map_err(App::raise_error)?
        };

        let bootstrap = StarknetBootstrap {
            fields: Arc::new(StarknetBootstrapFields {
                runtime: runtime.clone(),
                chain_command_path: args.chain_command_path.clone().into(),
                chain_store_dir: args.chain_store_dir.clone().into(),
                erc20_contract,
                ics20_contract,
                ibc_core_contract,
                comet_client_contract,
            }),
        };

        Ok(bootstrap)
    }
}
