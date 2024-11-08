use std::path::PathBuf;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::types::impls::WithType;
use cgp::prelude::*;
use hermes_cli_components::traits::bootstrap::{
    BootstrapLoaderComponent, BootstrapTypeComponent, CanLoadBootstrap,
};
use hermes_logger::ProvideHermesLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::error::traits::retry::RetryableErrorComponent;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use hermes_starknet_integration_tests::contexts::bootstrap::StarknetBootstrap;

use crate::impls::bootstrap::starknet_chain::{BootstrapStarknetChainArgs, LoadStarknetBootstrap};
use crate::impls::error::ProvideCliError;

#[derive(HasField)]
pub struct StarknetApp {
    pub config_path: PathBuf,
    pub runtime: HermesRuntime,
}

pub struct StarknetAppComponents;

pub struct StarknetParserComponents;

pub struct StarknetCommandRunnerComponents;

impl HasComponents for StarknetApp {
    type Components = StarknetAppComponents;
}

delegate_components! {
    StarknetAppComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
            RetryableErrorComponent,
        ]:
            ProvideCliError,
        [
            RuntimeTypeComponent,
            RuntimeGetterComponent,
        ]:
            ProvideDefaultRuntimeField,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideHermesLogger,
        BootstrapTypeComponent:
            WithType<StarknetBootstrap>,
        BootstrapLoaderComponent:
            LoadStarknetBootstrap,
    }
}

pub trait CanUseStarknetApp: CanLoadBootstrap<BootstrapStarknetChainArgs> {}

impl CanUseStarknetApp for StarknetApp {}
