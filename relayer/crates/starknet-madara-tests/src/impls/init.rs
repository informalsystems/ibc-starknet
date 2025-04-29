use std::sync::Arc;

use hermes_cosmos_core::tracing_logging_components::subscriber::init_tracing_subscriber;
use hermes_runtime::types::runtime::HermesRuntime;
use tokio::runtime::Builder;
use tracing::info;

pub fn init_test_runtime() -> HermesRuntime {
    let _ = stable_eyre::install();

    init_tracing_subscriber();

    let tokio_runtime = Arc::new(Builder::new_multi_thread().enable_all().build().unwrap());

    let runtime = HermesRuntime::new(tokio_runtime);

    info!("initialized Hermes test runtime");

    runtime
}
