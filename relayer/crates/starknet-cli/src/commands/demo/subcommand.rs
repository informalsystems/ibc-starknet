use cgp::core::Async;
use hermes_cli_components::traits::command::{CanRunCommand, CommandRunner};

use crate::commands::demo::transfer::TransferArgs;

#[derive(Debug, clap::Subcommand)]
pub enum DemoSubCommand {
    Transfer(TransferArgs),
}

pub struct RunDemoSubCommand;

impl<App> CommandRunner<App, DemoSubCommand> for RunDemoSubCommand
where
    App: CanRunCommand<TransferArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &DemoSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            DemoSubCommand::Transfer(args) => app.run_command(args).await,
        }
    }
}

pub trait CanRunSubCommand: Async {}

impl CanRunSubCommand for DemoSubCommand {}
