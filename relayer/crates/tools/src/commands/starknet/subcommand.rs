use cgp::prelude::*;
use hermes_cli_components::traits::{CanRunCommand, CommandRunner, CommandRunnerComponent};

use crate::commands::starknet::transfer_args::TransferArgs;

#[derive(Debug, clap::Subcommand)]
pub enum StarknetSubCommand {
    /// This command generates the arguments, `<args>` to pass to
    /// `starkli invoke <contract addr> send_transfer <args>` in order
    /// to trigger a token transfer
    TransferArgs(TransferArgs),
}

pub struct RunStarknetSubCommand;

#[cgp_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, StarknetSubCommand> for RunStarknetSubCommand
where
    App: CanRunCommand<TransferArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &StarknetSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            StarknetSubCommand::TransferArgs(args) => app.run_command(args).await,
        }
    }
}

pub trait CanRunSubCommand: Async {}

impl CanRunSubCommand for StarknetSubCommand {}
