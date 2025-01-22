use hermes_cli_components::impls::commands::client::update::UpdateClientArgs;
use hermes_cli_components::traits::command::{CanRunCommand, CommandRunner};

#[derive(Debug, clap::Subcommand)]
pub enum ClientSubCommand {
    CreateClient(CreateClientArgs),
    UpdateClient(UpdateClientArgs),
}

pub struct RunClientSubCommand;

impl<App> CommandRunner<App, ClientSubCommand> for RunClientSubCommand
where
    App: CanRunCommand<UpdateClientArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &ClientSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            ClientSubCommand::UpdateClient(args) => app.run_command(args).await,
            ClientSubCommand::CreateClient(args) => app.run_command(args).await,
        }
    }
}
