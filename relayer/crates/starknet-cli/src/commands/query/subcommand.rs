use hermes_cli_components::impls::{
    QueryBalanceArgs, QueryChainStatusArgs, QueryClientStateArgs, QueryConsensusStateArgs,
};
use hermes_cli_components::traits::{CanRunCommand, CommandRunner, CommandRunnerComponent};
use hermes_prelude::*;

#[derive(Debug, clap::Subcommand)]
pub enum QuerySubCommand {
    ClientState(QueryClientStateArgs),
    ConsensusState(QueryConsensusStateArgs),
    ChainStatus(QueryChainStatusArgs),
    Balance(QueryBalanceArgs),
}

pub struct RunQuerySubCommand;

#[cgp_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, QuerySubCommand> for RunQuerySubCommand
where
    App: CanRunCommand<QueryClientStateArgs>
        + CanRunCommand<QueryConsensusStateArgs>
        + CanRunCommand<QueryChainStatusArgs>
        + CanRunCommand<QueryBalanceArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &QuerySubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            QuerySubCommand::ClientState(args) => app.run_command(args).await,
            QuerySubCommand::ConsensusState(args) => app.run_command(args).await,
            QuerySubCommand::ChainStatus(args) => app.run_command(args).await,
            QuerySubCommand::Balance(args) => app.run_command(args).await,
        }
    }
}
