use hermes_cli::commands::{CreateChannelArgs, CreateCosmosClientArgs};
use hermes_cli_components::impls::CreateConnectionArgs;
use hermes_cli_components::traits::{CanRunCommand, CommandRunner, CommandRunnerComponent};
use hermes_prelude::*;

use crate::impls::CreateStarknetClientArgs;

#[derive(Debug, clap::Subcommand)]
pub enum CreateSubCommand {
    CosmosClient(CreateCosmosClientArgs),
    StarknetClient(CreateStarknetClientArgs),
    Connection(CreateConnectionArgs),
    Channel(CreateChannelArgs),
}

#[cgp_new_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, CreateSubCommand> for RunCreateSubCommand
where
    App: CanRunCommand<CreateCosmosClientArgs>
        + CanRunCommand<CreateStarknetClientArgs>
        + CanRunCommand<CreateConnectionArgs>
        + CanRunCommand<CreateChannelArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &CreateSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            CreateSubCommand::CosmosClient(args) => app.run_command(args).await,
            CreateSubCommand::StarknetClient(args) => app.run_command(args).await,
            CreateSubCommand::Connection(args) => app.run_command(args).await,
            CreateSubCommand::Channel(args) => app.run_command(args).await,
        }
    }
}
