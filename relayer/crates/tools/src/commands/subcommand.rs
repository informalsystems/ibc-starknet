use hermes_cli_components::traits::{CanRunCommand, CommandRunner, CommandRunnerComponent};
use hermes_prelude::*;

use crate::commands::starknet::subcommand::StarknetSubCommand;

#[derive(Debug, clap::Subcommand)]
pub enum AllSubCommands {
    #[clap(subcommand)]
    Starknet(StarknetSubCommand),
}

pub struct RunAllSubCommand;

#[cgp_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, AllSubCommands> for RunAllSubCommand
where
    App: CanRunCommand<StarknetSubCommand>,
{
    async fn run_command(
        app: &App,
        subcommand: &AllSubCommands,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            AllSubCommands::Starknet(args) => app.run_command(args).await,
        }
    }
}
