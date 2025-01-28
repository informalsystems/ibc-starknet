use cgp::core::Async;
use hermes_cli_components::traits::command::{CanRunCommand, CommandRunner};

use crate::commands::starknet::transfer_args::TransferArgs;

#[derive(Debug, clap::Subcommand)]
pub enum StarknetSubCommand {
    TransferArgs(TransferArgs),
}

pub struct RunStarknetSubCommand;

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
