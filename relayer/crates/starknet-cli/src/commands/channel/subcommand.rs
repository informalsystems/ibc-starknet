use hermes_cli_components::impls::commands::channel::create::CreateChannelArgs;
use hermes_cli_components::traits::command::{CanRunCommand, CommandRunner};

#[derive(Debug, clap::Subcommand)]
pub enum ChannelSubCommand {
    Create(CreateChannelArgs),
}

pub struct RunChannelSubCommand;

impl<App> CommandRunner<App, ChannelSubCommand> for RunChannelSubCommand
where
    App: CanRunCommand<CreateChannelArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &ChannelSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            ChannelSubCommand::Create(args) => app.run_command(args).await,
        }
    }
}
