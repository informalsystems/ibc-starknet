use std::path::PathBuf;

use cgp::core::component::{UseContext, UseDelegate};
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::types::impls::WithType;
use cgp::prelude::*;
use hermes_cli_components::impls::commands::bootstrap::chain::RunBootstrapChainCommand;
use hermes_cli_components::impls::config::get_config_path::GetDefaultConfigField;
use hermes_cli_components::traits::bootstrap::{
    BootstrapLoaderComponent, BootstrapTypeComponent, CanLoadBootstrap,
};
use hermes_cli_components::traits::command::CommandRunnerComponent;
use hermes_cli_components::traits::config::config_path::ConfigPathGetterComponent;
use hermes_error::types::HermesError;
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
use hermes_starknet_integration_tests::contexts::chain_driver::StarknetChainDriver;
use hermes_test_components::chain_driver::traits::config::ConfigUpdater;
use toml::to_string_pretty;

use crate::impls::bootstrap::starknet_chain::{BootstrapStarknetChainArgs, LoadStarknetBootstrap};
use crate::impls::bootstrap::subcommand::{BootstrapSubCommand, RunBootstrapSubCommand};
use crate::impls::error::ProvideCliError;
use crate::types::config::{StarknetChainConfig, StarknetRelayerConfig};

#[derive(HasField)]
pub struct StarknetApp {
    pub config_path: PathBuf,
    pub runtime: HermesRuntime,
}

pub struct StarknetAppComponents;

pub struct StarknetParserComponents;

pub struct StarknetCommandRunnerComponents;

pub struct UpdateStarknetConfig;

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
        ConfigPathGetterComponent:
            GetDefaultConfigField,
        CommandRunnerComponent:
            UseDelegate<StarknetCommandRunnerComponents>,
    }
}

delegate_components! {
    StarknetCommandRunnerComponents {
        BootstrapSubCommand: RunBootstrapSubCommand,
        BootstrapStarknetChainArgs: RunBootstrapChainCommand<UseContext>,
    }
}

impl ConfigUpdater<StarknetChainDriver, StarknetRelayerConfig> for UpdateStarknetConfig {
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

pub trait CanUseStarknetApp: CanLoadBootstrap<BootstrapStarknetChainArgs> {}

impl CanUseStarknetApp for StarknetApp {}
