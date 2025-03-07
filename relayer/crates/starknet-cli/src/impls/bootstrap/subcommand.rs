use cgp::prelude::*;
use hermes_cli::commands::bootstrap::chain::BootstrapCosmosChainArgs;
use hermes_cli_components::traits::command::{
    CanRunCommand, CommandRunner, CommandRunnerComponent,
};

use crate::impls::bootstrap::starknet_chain::BootstrapStarknetChainArgs;

#[derive(Debug, clap::Subcommand)]
pub enum BootstrapSubCommand {
    StarknetChain(BootstrapStarknetChainArgs),
    CosmosChain(BootstrapCosmosChainArgs),
}

#[cgp_new_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, BootstrapSubCommand> for RunBootstrapSubCommand
where
    App: CanRunCommand<BootstrapStarknetChainArgs> + CanRunCommand<BootstrapCosmosChainArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &BootstrapSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            BootstrapSubCommand::StarknetChain(args) => app.run_command(args).await,
            BootstrapSubCommand::CosmosChain(args) => app.run_command(args).await,
        }
    }
}
