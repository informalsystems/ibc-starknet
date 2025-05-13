use hermes_cli_components::traits::{CanRunCommand, CommandRunner, CommandRunnerComponent};
use hermes_prelude::*;

use crate::commands::{
    BootstrapSubCommand, CreateSubCommand, QuerySubCommand, StartRelayerArgs, UpdateSubCommand,
};

#[derive(Debug, clap::Subcommand)]
pub enum AllSubCommands {
    Start(StartRelayerArgs),

    #[clap(subcommand)]
    Bootstrap(BootstrapSubCommand),

    #[clap(subcommand)]
    Query(QuerySubCommand),

    #[clap(subcommand)]
    Create(CreateSubCommand),

    #[clap(subcommand)]
    Update(UpdateSubCommand),
}

pub struct RunAllSubCommand;

#[cgp_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, AllSubCommands> for RunAllSubCommand
where
    App: CanRunCommand<BootstrapSubCommand>
        + CanRunCommand<QuerySubCommand>
        + CanRunCommand<CreateSubCommand>
        + CanRunCommand<UpdateSubCommand>
        + CanRunCommand<StartRelayerArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &AllSubCommands,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            AllSubCommands::Start(args) => app.run_command(args).await,
            AllSubCommands::Bootstrap(args) => app.run_command(args).await,
            AllSubCommands::Query(args) => app.run_command(args).await,
            AllSubCommands::Create(args) => app.run_command(args).await,
            AllSubCommands::Update(args) => app.run_command(args).await,
        }
    }
}
