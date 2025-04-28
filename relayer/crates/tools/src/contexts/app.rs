use std::path::PathBuf;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_cli_components::impls::{GetDefaultConfigField, LoadTomlConfig, WriteTomlConfig};
use hermes_cli_components::traits::{
    BootstrapLoaderComponent, BootstrapTypeProviderComponent, BuilderLoaderComponent,
    BuilderTypeComponent, CanLoadBootstrap, CanLoadBuilder, CanLoadConfig, CanProduceOutput,
    CanWriteConfig, CommandRunnerComponent, ConfigLoaderComponent, ConfigPathGetterComponent,
    ConfigTypeComponent, ConfigWriterComponent, HasConfigPath, HasConfigType, OutputProducer,
    OutputProducerComponent, OutputTypeComponent,
};
use hermes_core::logging_components::traits::LoggerComponent;
use hermes_core::relayer_components::error::traits::RetryableErrorComponent;
use hermes_core::runtime_components::traits::{
    HasRuntime, RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_cosmos_core::tracing_logging_components::contexts::TracingLogger;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_starknet_chain_components::impls::types::config::StarknetRelayerConfig;
use hermes_starknet_cli::impls::bootstrap::starknet_chain::{
    BootstrapStarknetChainArgs, LoadStarknetBootstrap,
};
use hermes_starknet_cli::impls::build::LoadStarknetBuilder;
use hermes_starknet_cli::impls::error::ProvideCliError;
use hermes_starknet_integration_tests::contexts::starknet_bootstrap::StarknetBootstrap;
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
            ErrorWrapperComponent,
            RetryableErrorComponent,
        ]:
            ProvideCliError,
        RuntimeTypeProviderComponent:
            UseType<HermesRuntime>,
        RuntimeGetterComponent:
            UseField<symbol!("runtime")>,
        LoggerComponent:
            TracingLogger,
        ConfigTypeComponent:
            WithType<StarknetRelayerConfig>,
        BootstrapTypeProviderComponent:
            UseType<StarknetBootstrap>,
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

#[cgp_provider(OutputProducerComponent)]
impl<Value> OutputProducer<ToolApp, Value> for ToolAppComponents {
    fn produce_output(_app: &ToolApp, _value: Value) {}
}

pub trait CanUseToolApp:
    HasRuntime
    + HasConfigPath
    + HasConfigType<Config = StarknetRelayerConfig>
    + CanLoadConfig
    + CanWriteConfig
    + CanWrapError<&'static str>
    + CanProduceOutput<()>
    + CanLoadBootstrap<(), BootstrapStarknetChainArgs>
    + CanLoadBuilder<Builder = StarknetBuilder>
{
}

impl CanUseToolApp for ToolApp {}
