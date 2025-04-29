use hermes_cli_components::traits::{CanRunCommand, CommandRunner, CommandRunnerComponent};
use hermes_prelude::*;

use crate::impls::{BootstrapOsmosisChainArgs, BootstrapStarknetChainArgs};

#[derive(Debug, clap::Subcommand)]
pub enum BootstrapSubCommand {
    StarknetChain(BootstrapStarknetChainArgs),
    OsmosisChain(BootstrapOsmosisChainArgs),
}

#[cgp_new_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, BootstrapSubCommand> for RunBootstrapSubCommand
where
    App: CanRunCommand<BootstrapStarknetChainArgs> + CanRunCommand<BootstrapOsmosisChainArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &BootstrapSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            BootstrapSubCommand::StarknetChain(args) => app.run_command(args).await,
            BootstrapSubCommand::OsmosisChain(args) => app.run_command(args).await,
        }
    }
}
