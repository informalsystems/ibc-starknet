use hermes_cli_components::impls::commands::queries::client_state::QueryClientStateArgs;
use hermes_cli_components::traits::command::{CanRunCommand, CommandRunner};

#[derive(Debug, clap::Subcommand)]
pub enum QuerySubCommands {
    Query(QueryClientStateArgs),
}

pub struct RunQuerySubCommand;

impl<App> CommandRunner<App, QuerySubCommands> for RunQuerySubCommand
where
    App: CanRunCommand<QueryClientStateArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &QuerySubCommands,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            QuerySubCommands::Query(args) => app.run_command(args).await,
        }
    }
}
