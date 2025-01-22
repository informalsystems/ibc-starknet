use hermes_cli_components::traits::command::{CanRunCommand, CommandRunner};

use crate::commands::connection::subcommand::ConnectionSubCommand;
use crate::commands::query::subcommand::QuerySubCommand;
use crate::impls::bootstrap::subcommand::BootstrapSubCommand;

#[derive(Debug, clap::Subcommand)]
pub enum AllSubCommands {
    #[clap(subcommand)]
    Bootstrap(BootstrapSubCommand),
    #[clap(subcommand)]
    Query(QuerySubCommand),
    #[clap(subcommand)]
    Connection(ConnectionSubCommand),
}

pub struct RunAllSubCommand;

impl<App> CommandRunner<App, AllSubCommands> for RunAllSubCommand
where
    App: CanRunCommand<BootstrapSubCommand> + CanRunCommand<QuerySubCommand>,
{
    async fn run_command(
        app: &App,
        subcommand: &AllSubCommands,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            AllSubCommands::Bootstrap(args) => app.run_command(args).await,
            AllSubCommands::Query(args) => app.run_command(args).await,
            AllSubCommands::Connection(args) => app.run_command(args).await,
        }
    }
}
