use hermes_cli_components::impls::commands::connection::create::CreateConnectionArgs;
use hermes_cli_components::traits::command::{CanRunCommand, CommandRunner};

#[derive(Debug, clap::Subcommand)]
pub enum CreateSubCommand {
    Connection(CreateConnectionArgs),
}

pub struct RunCreateSubCommand;

impl<App> CommandRunner<App, CreateSubCommand> for RunCreateSubCommand
where
    App: CanRunCommand<CreateConnectionArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &CreateSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            CreateSubCommand::Connection(args) => app.run_command(args).await,
        }
    }
}
