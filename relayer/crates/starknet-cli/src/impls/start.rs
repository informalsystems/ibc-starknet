use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_cli_components::impls::commands::start::{RunStartRelayerCommand, StartRelayerArgs};
use hermes_cli_components::traits::command::{CommandRunner, CommandRunnerComponent};
use hermes_cli_components::traits::output::HasOutputType;

#[derive(Debug, clap::Subcommand)]
pub enum StartSubcommand {
    CosmosWithStarknet(StartRelayerArgs),
    StarknetWithCosmos(StartRelayerArgs),
}

#[new_cgp_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, StartSubcommand> for RunStartSubcommand
where
    App: HasOutputType + HasAsyncErrorType,
    RunStartRelayerCommand<Index<0>, Index<1>>: CommandRunner<App, StartRelayerArgs>,
    RunStartRelayerCommand<Index<1>, Index<0>>: CommandRunner<App, StartRelayerArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &StartSubcommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            StartSubcommand::CosmosWithStarknet(args) => {
                <RunStartRelayerCommand<Index<0>, Index<1>>>::run_command(app, args).await
            }
            StartSubcommand::StarknetWithCosmos(args) => {
                <RunStartRelayerCommand<Index<1>, Index<0>>>::run_command(app, args).await
            }
        }
    }
}
