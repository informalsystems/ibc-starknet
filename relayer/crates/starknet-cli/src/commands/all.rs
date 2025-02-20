use cgp::prelude::*;
use hermes_cli_components::traits::command::{
    CanRunCommand, CommandRunner, CommandRunnerComponent,
};

use crate::commands::create::subcommand::CreateSubCommand;
use crate::commands::query::subcommand::QuerySubCommand;
use crate::commands::start::StartSubCommand;
use crate::commands::update::subcommand::UpdateSubCommand;
use crate::impls::bootstrap::subcommand::BootstrapSubCommand;

#[derive(Debug, clap::Subcommand)]
pub enum AllSubCommands {
    #[clap(subcommand)]
    Start(StartSubCommand),

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
        + CanRunCommand<StartSubCommand>,
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
