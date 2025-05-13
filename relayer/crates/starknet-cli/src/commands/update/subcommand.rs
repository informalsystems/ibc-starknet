use hermes_cli_components::impls::UpdateClientArgs;
use hermes_cli_components::traits::{CanRunCommand, CommandRunner, CommandRunnerComponent};
use hermes_prelude::*;

#[derive(Debug, clap::Subcommand)]
pub enum UpdateSubCommand {
    Client(UpdateClientArgs),
}

pub struct RunUpdateSubCommand;

#[cgp_provider(CommandRunnerComponent)]
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
