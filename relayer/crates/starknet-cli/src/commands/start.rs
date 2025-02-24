use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_cli_components::impls::commands::start::{RunStartRelayerCommand, StartRelayerArgs};
use hermes_cli_components::traits::command::{CommandRunner, CommandRunnerComponent};
use hermes_cli_components::traits::output::HasOutputType;

#[derive(Debug, clap::Subcommand)]
pub enum StartSubCommand {
    StarknetWithCosmos(StartRelayerArgs),
    CosmosWithStarknet(StartRelayerArgs),
}

#[cgp_new_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, StartSubCommand> for RunStartSubCommand
where
    App: HasOutputType + HasAsyncErrorType,
    RunStartRelayerCommand<Index<0>, Index<1>>: CommandRunner<App, StartRelayerArgs>,
    RunStartRelayerCommand<Index<1>, Index<0>>: CommandRunner<App, StartRelayerArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &StartSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            StartSubCommand::StarknetWithCosmos(args) => {
                <RunStartRelayerCommand<Index<0>, Index<1>>>::run_command(app, args).await
            }
            StartSubCommand::CosmosWithStarknet(args) => {
                <RunStartRelayerCommand<Index<1>, Index<0>>>::run_command(app, args).await
            }
        }
    }
}
