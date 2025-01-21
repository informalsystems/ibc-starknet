use hermes_cli_components::impls::commands::connection::create::CreateConnectionArgs;
use hermes_cli_components::traits::command::{CanRunCommand, CommandRunner};

#[derive(Debug, clap::Subcommand)]
pub enum ConnectionSubCommand {
    Create(CreateConnectionArgs),
}

pub struct RunConnectionSubCommand;

impl<App> CommandRunner<App, ConnectionSubCommand> for RunConnectionSubCommand
where
    App: CanRunCommand<CreateConnectionArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &ConnectionSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            ConnectionSubCommand::Create(args) => app.run_command(args).await,
        }
    }
}
