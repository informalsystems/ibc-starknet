#![recursion_limit = "256"]

use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use eyre::{eyre, Error};
use hermes_cli_components::traits::command::CanRunCommand;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_starknet_cli::commands::all::AllSubCommands;
use hermes_starknet_cli::contexts::app::StarknetApp;
use tokio::runtime::Builder;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    let _ = stable_eyre::install();

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(env_filter)
        .init();

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

pub fn install_logger() {
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::{fmt, registry, EnvFilter};

    // Use log level INFO by default if RUST_LOG is not set.
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let fmt_layer = fmt::layer().with_ansi(true).with_target(false).compact();

    registry().with(env_filter).with(fmt_layer).init();
}
