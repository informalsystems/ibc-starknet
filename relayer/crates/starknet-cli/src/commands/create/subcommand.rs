use cgp::prelude::*;
use hermes_cli::commands::client::create::CreateClientArgs;
use hermes_cli_components::traits::command::{
    CanRunCommand, CommandRunner, CommandRunnerComponent,
};

#[derive(Debug, clap::Subcommand)]
pub enum CreateSubCommand {
    Client(CreateClientArgs),
}

#[cgp_new_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, CreateSubCommand> for RunCreateSubCommand
where
    App: CanRunCommand<CreateClientArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &CreateSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            CreateSubCommand::Client(args) => app.run_command(args).await,
        }
    }
}
