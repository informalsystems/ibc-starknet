#![recursion_limit = "512"]

use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use eyre::{eyre, Error};
use hermes_cli_components::traits::CanRunCommand;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_starknet_cli::commands::all::AllSubCommands;
use hermes_starknet_cli::contexts::app::StarknetApp;
use hermes_tracing_logging_components::subscriber::init_tracing_subscriber;
use tokio::runtime::Builder;

fn main() {
    let _ = stable_eyre::install();

    init_tracing_subscriber();

    let res = run_main();

    if let Err(e) = res {
        tracing::error!("{e:?}");
        std::process::exit(1);
    }
}

#[derive(clap::Parser)]
pub struct MainCommand {
    #[clap(short = 'c', long = "config", default_value = "config.toml")]
    pub config_path: PathBuf,

    #[command(subcommand)]
    pub bootstrap: AllSubCommands,
}

pub fn run_main() -> Result<(), Error> {
    let tokio_runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| eyre!("failed to initialized tokio runtime: {e}"))?;

    let runtime = HermesRuntime::new(Arc::new(tokio_runtime));

    let command = MainCommand::parse();

    let app = StarknetApp {
        runtime: runtime.clone(),
        config_path: command.config_path,
    };

    runtime.runtime.block_on(async {
        app.run_command(&command.bootstrap)
            .await
            .map_err(|e| eyre!("{e}"))?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
