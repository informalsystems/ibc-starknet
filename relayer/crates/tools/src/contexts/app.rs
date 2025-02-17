use std::path::PathBuf;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::WithField;
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_cli_components::impls::config::get_config_path::GetDefaultConfigField;
use hermes_cli_components::impls::config::load_toml_config::LoadTomlConfig;
use hermes_cli_components::impls::config::save_toml_config::WriteTomlConfig;
use hermes_cli_components::traits::bootstrap::{
    BootstrapLoaderComponent, BootstrapTypeComponent, CanLoadBootstrap,
};
use hermes_cli_components::traits::build::{
    BuilderLoaderComponent, BuilderTypeComponent, CanLoadBuilder,
};
use hermes_cli_components::traits::command::CommandRunnerComponent;
use hermes_cli_components::traits::config::config_path::{
    ConfigPathGetterComponent, HasConfigPath,
};
use hermes_cli_components::traits::config::load_config::{CanLoadConfig, ConfigLoaderComponent};
use hermes_cli_components::traits::config::write_config::{CanWriteConfig, ConfigWriterComponent};
use hermes_cli_components::traits::output::{
    CanProduceOutput, OutputProducer, OutputTypeComponent,
};
use hermes_cli_components::traits::types::config::{ConfigTypeComponent, HasConfigType};
use hermes_error::traits::wrap::CanWrapError;
use hermes_logger::ProvideHermesLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, HasLogger, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    HasRuntime, RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_starknet_chain_components::components::chain::RetryableErrorComponent;
use hermes_starknet_chain_components::impls::types::config::StarknetRelayerConfig;
use hermes_starknet_cli::impls::bootstrap::starknet_chain::{
    BootstrapStarknetChainArgs, LoadStarknetBootstrap,
};
use hermes_starknet_cli::impls::build::LoadStarknetBuilder;
use hermes_starknet_cli::impls::error::ProvideCliError;
use hermes_starknet_integration_tests::contexts::bootstrap::StarknetBootstrap;
use hermes_starknet_relayer::contexts::builder::StarknetBuilder;

use crate::commands::starknet::subcommand::{RunStarknetSubCommand, StarknetSubCommand};
use crate::commands::starknet::transfer_args::{RunTransferArgs, TransferArgs};
use crate::commands::subcommand::{AllSubCommands, RunAllSubCommand};

#[cgp_context(ToolAppComponents)]
#[derive(HasField)]
pub struct ToolApp {
    pub config_path: PathBuf,
    pub runtime: HermesRuntime,
}

pub struct ToolCommandRunnerComponents;

delegate_components! {
    ToolAppComponents {
        [
            ErrorTypeProviderComponent,
            ErrorRaiserComponent,
            RetryableErrorComponent,
        ]:
            ProvideCliError,
        RuntimeTypeProviderComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideHermesLogger,
        ConfigTypeComponent:
            WithType<StarknetRelayerConfig>,
        BootstrapTypeComponent:
            WithType<StarknetBootstrap>,
        OutputTypeComponent:
            WithType<()>,
        BootstrapLoaderComponent:
            LoadStarknetBootstrap,
        ConfigPathGetterComponent:
            GetDefaultConfigField,
        ConfigLoaderComponent:
            LoadTomlConfig,
        ConfigWriterComponent:
            WriteTomlConfig,
        CommandRunnerComponent:
            UseDelegate<ToolCommandRunnerComponents>,
        BuilderLoaderComponent:
            LoadStarknetBuilder,
        BuilderTypeComponent:
            WithType<StarknetBuilder>,
    }
}

delegate_components! {
    ToolCommandRunnerComponents {
        AllSubCommands: RunAllSubCommand,

        StarknetSubCommand: RunStarknetSubCommand,
        TransferArgs: RunTransferArgs,
    }
}

impl<Value> OutputProducer<ToolApp, Value> for ToolAppComponents {
    fn produce_output(_app: &ToolApp, _value: Value) {}
}

pub trait CanUseToolApp:
    HasRuntime
    + HasLogger
    + HasConfigPath
    + HasConfigType<Config = StarknetRelayerConfig>
    + CanLoadConfig
    + CanWriteConfig
    + CanWrapError<&'static str>
    + CanProduceOutput<()>
    + CanLoadBootstrap<BootstrapStarknetChainArgs>
    + CanLoadBuilder<Builder = StarknetBuilder>
{
}

impl CanUseToolApp for ToolApp {}
