use hermes_cli_components::traits::command::{CanRunCommand, CommandRunner};

use crate::impls::bootstrap::starknet_chain::BootstrapStarknetChainArgs;

#[derive(Debug, clap::Subcommand)]
pub enum BootstrapSubCommand {
    StarknetChain(BootstrapStarknetChainArgs),
}

pub struct RunBootstrapSubCommand;

impl<App> CommandRunner<App, BootstrapSubCommand> for RunBootstrapSubCommand
where
    App: CanRunCommand<BootstrapStarknetChainArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &BootstrapSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            BootstrapSubCommand::StarknetChain(args) => app.run_command(args).await,
        }
    }
}