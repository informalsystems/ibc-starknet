use cgp::prelude::*;
use hermes_cli_components::traits::bootstrap::{BootstrapLoader, HasBootstrapType};
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_starknet_integration_tests::contexts::bootstrap::StarknetBootstrap;

#[derive(Debug, clap::Parser, HasField)]
pub struct BootstrapStarknetChainArgs {
    #[clap(long = "chain-store-dir", required = true)]
    pub chain_store_dir: String,

    #[clap(long = "chain-command-path", default_value = "starknet-devnet")]
    pub chain_command_path: String,
}

pub struct LoadStarknetBootstrap;

impl<App> BootstrapLoader<App, BootstrapStarknetChainArgs> for LoadStarknetBootstrap
where
    App: HasBootstrapType<Bootstrap = StarknetBootstrap>
        + HasRuntime<Runtime = HermesRuntime>
        + HasErrorType,
{
    async fn load_bootstrap(
        app: &App,
        args: &BootstrapStarknetChainArgs,
    ) -> Result<App::Bootstrap, App::Error> {
        let runtime = app.runtime();

        let bootstrap = StarknetBootstrap {
            runtime: runtime.clone(),
            chain_command_path: args.chain_command_path.clone().into(),
            chain_store_dir: args.chain_store_dir.clone().into(),
        };

        Ok(bootstrap)
    }
}
