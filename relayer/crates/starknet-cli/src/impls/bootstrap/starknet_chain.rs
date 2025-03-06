use cgp::prelude::*;
use hermes_cli_components::traits::bootstrap::{
    BootstrapLoader, BootstrapLoaderComponent, HasBootstrapType,
};
use hermes_runtime::types::error::TokioRuntimeError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_starknet_integration_tests::contexts::bootstrap::StarknetBootstrap;

#[derive(Debug, clap::Parser, HasField)]
pub struct BootstrapStarknetChainArgs {
    #[clap(long = "chain-id", required = true)]
    pub chain_id: String,

    #[clap(long = "chain-store-dir", required = true)]
    pub chain_store_dir: String,

    #[clap(long = "chain-command-path", default_value = "starknet-devnet")]
    pub chain_command_path: String,

    pub erc20_contract_path: String,
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

        let bootstrap = StarknetBootstrap {
            runtime: runtime.clone(),
            chain_command_path: args.chain_command_path.clone().into(),
            chain_store_dir: args.chain_store_dir.clone().into(),
            erc20_contract,
        };

        Ok(bootstrap)
    }
}
