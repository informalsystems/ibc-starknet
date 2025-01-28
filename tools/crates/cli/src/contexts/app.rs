use std::path::PathBuf;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::WithField;
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_cli_components::impls::config::get_config_path::GetDefaultConfigField;
use hermes_cli_components::impls::config::load_toml_config::LoadTomlConfig;
use hermes_cli_components::impls::config::save_toml_config::WriteTomlConfig;
use hermes_cli_components::traits::any_counterparty::ProvideAnyCounterparty;
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
use hermes_cli_components::traits::parse::ArgParserComponent;
use hermes_cli_components::traits::types::config::{ConfigTypeComponent, HasConfigType};
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::traits::wrap::CanWrapError;
use hermes_error::types::HermesError;
use hermes_logger::ProvideHermesLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, HasLogger, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    HasRuntime, RuntimeGetterComponent, RuntimeTypeComponent,
};
use hermes_starknet_chain_components::components::chain::RetryableErrorComponent;
use hermes_starknet_chain_components::impls::types::config::{
    StarknetChainConfig, StarknetRelayerConfig,
};
use hermes_starknet_cli::impls::bootstrap::starknet_chain::{
    BootstrapStarknetChainArgs, LoadStarknetBootstrap,
};
use hermes_starknet_cli::impls::build::LoadStarknetBuilder;
use hermes_starknet_cli::impls::error::ProvideCliError;
use hermes_starknet_integration_tests::contexts::bootstrap::StarknetBootstrap;
use hermes_starknet_integration_tests::contexts::chain_driver::StarknetChainDriver;
use hermes_starknet_relayer::contexts::builder::StarknetBuilder;
use hermes_test_components::chain_driver::traits::config::ConfigUpdater;
use toml::to_string_pretty;

use crate::commands::starknet::subcommand::{RunStarknetSubCommand, StarknetSubCommand};
use crate::commands::starknet::transfer_args::{RunTransferArgs, TransferArgs};
use crate::commands::subcommand::{AllSubCommands, RunAllSubCommand};

#[derive(HasField)]
pub struct ToolApp {
    pub config_path: PathBuf,
    pub runtime: HermesRuntime,
}

pub struct ToolAppComponents;

pub struct ToolParserComponents;

pub struct ToolCommandRunnerComponents;

pub struct UpdateToolConfig;

impl HasComponents for ToolApp {
    type Components = ToolAppComponents;
}

delegate_components! {
    ToolAppComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
            RetryableErrorComponent,
        ]:
            ProvideCliError,
        RuntimeTypeComponent: WithType<HermesRuntime>,
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
        ArgParserComponent:
            UseDelegate<ToolParserComponents>,
    }
}

delegate_components! {
    ToolCommandRunnerComponents {
        AllSubCommands: RunAllSubCommand,

        StarknetSubCommand: RunStarknetSubCommand,
        TransferArgs: RunTransferArgs,
    }
}

impl<App> ProvideAnyCounterparty<App> for ToolAppComponents
where
    App: Async,
{
    type AnyCounterparty = CosmosChain;
}

impl<Value> OutputProducer<ToolApp, Value> for ToolAppComponents {
    fn produce_output(_app: &ToolApp, _value: Value) {}
}

impl ConfigUpdater<StarknetChainDriver, StarknetRelayerConfig> for UpdateToolConfig {
    fn update_config(
        chain_driver: &StarknetChainDriver,
        config: &mut StarknetRelayerConfig,
    ) -> Result<String, HermesError> {
        if config.starknet_chain_config.is_some() {
            return Err(StarknetChainDriver::raise_error(
                "starknet chain config is already present in config file",
            ));
        }

        let relayer_wallet = chain_driver
            .wallets
            .get("relayer")
            .ok_or_else(|| StarknetChainDriver::raise_error("expect relayer wallet to be present"))?
            .clone();

        let chain_config = StarknetChainConfig {
            json_rpc_url: format!("http://localhost:{}/", chain_driver.node_config.rpc_port),
            relayer_wallet,
        };

        let chain_config_str = to_string_pretty(&chain_config)?;

        config.starknet_chain_config = Some(chain_config);

        Ok(chain_config_str)
    }
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
