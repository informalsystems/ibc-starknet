use hermes_cli_components::impls::commands::queries::client_state::QueryClientStateArgs;
use hermes_cli_components::impls::commands::queries::consensus_state::QueryConsensusStateArgs;
use hermes_cli_components::traits::command::{CanRunCommand, CommandRunner};

#[derive(Debug, clap::Subcommand)]
pub enum QuerySubCommand {
    ClientState(QueryClientStateArgs),
    ConsensusState(QueryConsensusStateArgs),
}

pub struct RunQuerySubCommand;

impl<App> CommandRunner<App, QuerySubCommand> for RunQuerySubCommand
where
    App: CanRunCommand<QueryClientStateArgs> + CanRunCommand<QueryConsensusStateArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &QuerySubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            QuerySubCommand::ClientState(args) => app.run_command(args).await,
            QuerySubCommand::ConsensusState(args) => app.run_command(args).await,
        }
    }
}
