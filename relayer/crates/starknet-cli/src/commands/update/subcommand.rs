use hermes_cli::commands::client::update::UpdateClientArgs;
use hermes_cli_components::impls::commands::client::update::UpdateClientArgs;
use hermes_cli_components::traits::command::{CanRunCommand, CommandRunner};

#[derive(Debug, clap::Subcommand)]
pub enum UpdateSubCommand {
    Client(UpdateClientArgs),
}

pub struct RunUpdateSubCommand;

impl<App> CommandRunner<App, UpdateSubCommand> for RunUpdateSubCommand
where
    App: CanRunCommand<UpdateClientArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &UpdateSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            UpdateSubCommand::Client(args) => app.run_command(args).await,
        }
    }
}
