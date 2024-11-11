use hermes_cli_components::traits::command::{CanRunCommand, CommandRunner};

use crate::impls::bootstrap::subcommand::BootstrapSubCommand;

#[derive(Debug, clap::Subcommand)]
pub enum AllSubCommands {
    #[clap(subcommand)]
    Bootstrap(BootstrapSubCommand),
}

pub struct RunAllSubCommand;

impl<App> CommandRunner<App, AllSubCommands> for RunAllSubCommand
where
    App: CanRunCommand<BootstrapSubCommand>,
{
    async fn run_command(
        app: &App,
        subcommand: &AllSubCommands,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            AllSubCommands::Bootstrap(args) => app.run_command(args).await,
        }
    }
}
