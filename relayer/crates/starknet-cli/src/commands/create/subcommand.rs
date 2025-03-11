use cgp::prelude::*;
use hermes_cli::commands::client::create::CreateCosmosClientArgs;
use hermes_cli_components::impls::commands::connection::create::CreateConnectionArgs;
use hermes_cli_components::traits::command::{
    CanRunCommand, CommandRunner, CommandRunnerComponent,
};

use crate::impls::create_client::CreateStarknetClientArgs;

#[derive(Debug, clap::Subcommand)]
pub enum CreateSubCommand {
    CosmosClient(CreateCosmosClientArgs),
    StarknetClient(CreateStarknetClientArgs),
    Connection(CreateConnectionArgs),
}

#[cgp_new_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, CreateSubCommand> for RunCreateSubCommand
where
    App: CanRunCommand<CreateCosmosClientArgs>
        + CanRunCommand<CreateStarknetClientArgs>
        + CanRunCommand<CreateConnectionArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &CreateSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            CreateSubCommand::CosmosClient(args) => app.run_command(args).await,
            CreateSubCommand::StarknetClient(args) => app.run_command(args).await,
            CreateSubCommand::Connection(args) => app.run_command(args).await,
        }
    }
}
