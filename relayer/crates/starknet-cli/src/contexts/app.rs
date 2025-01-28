use std::path::PathBuf;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::{Index, WithField};
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_cli::commands::client::create::CreateClientArgs;
use hermes_cli_components::impls::commands::bootstrap::chain::RunBootstrapChainCommand;
use hermes_cli_components::impls::commands::client::create::{
    CreateClientOptionsParser, RunCreateClientCommand,
};
use hermes_cli_components::impls::commands::client::update::{
    RunUpdateClientCommand, UpdateClientArgs,
};
use hermes_cli_components::impls::commands::queries::balance::{
    QueryBalanceArgs, RunQueryBalanceCommand,
};
use hermes_cli_components::impls::commands::queries::chain_status::{
    QueryChainStatusArgs, RunQueryChainStatusCommand,
};
use hermes_cli_components::impls::commands::queries::client_state::{
    QueryClientStateArgs, RunQueryClientStateCommand,
};
use hermes_cli_components::impls::commands::queries::consensus_state::{
    QueryConsensusStateArgs, RunQueryConsensusStateCommand,
};
use hermes_cli_components::impls::config::get_config_path::GetDefaultConfigField;
use hermes_cli_components::impls::config::load_toml_config::LoadTomlConfig;
use hermes_cli_components::impls::config::save_toml_config::WriteTomlConfig;
use hermes_cli_components::impls::parse::string::{ParseFromOptionalString, ParseFromString};
use hermes_cli_components::traits::any_counterparty::ProvideAnyCounterparty;
use hermes_cli_components::traits::bootstrap::{
    BootstrapLoaderComponent, BootstrapTypeComponent, CanLoadBootstrap,
};
use hermes_cli_components::traits::build::{
    BuilderLoaderComponent, BuilderTypeComponent, CanLoadBuilder,
};
use hermes_cli_components::traits::command::{CanRunCommand, CommandRunnerComponent};
use hermes_cli_components::traits::config::config_path::{
    ConfigPathGetterComponent, HasConfigPath,
};
use hermes_cli_components::traits::config::load_config::{CanLoadConfig, ConfigLoaderComponent};
use hermes_cli_components::traits::config::write_config::{CanWriteConfig, ConfigWriterComponent};
use hermes_cli_components::traits::output::{
    CanProduceOutput, OutputProducer, OutputTypeComponent,
};
use hermes_cli_components::traits::parse::ArgParserComponent;
use hermes_cli_components::traits::types::config::ConfigTypeComponent;
use hermes_cosmos_chain_components::types::payloads::client::CosmosCreateClientOptions;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::traits::wrap::CanWrapError;
use hermes_error::types::HermesError;
use hermes_logger::ProvideHermesLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, HasLogger, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::error::traits::retry::RetryableErrorComponent;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    HasRuntime, RuntimeGetterComponent, RuntimeTypeComponent,
};
use hermes_starknet_chain_components::impls::types::config::{
    StarknetChainConfig, StarknetRelayerConfig,
};
use hermes_starknet_chain_components::types::client_id::ClientId;
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_integration_tests::contexts::bootstrap::StarknetBootstrap;
use hermes_starknet_integration_tests::contexts::chain_driver::StarknetChainDriver;
use hermes_starknet_relayer::contexts::builder::StarknetBuilder;
use hermes_test_components::chain_driver::traits::config::ConfigUpdater;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChainId, ClientId as CosmosClientId};
use starknet::core::types::Felt;
use toml::to_string_pretty;

use crate::commands::create::subcommand::{CreateSubCommand, RunCreateSubCommand};
use crate::commands::query::subcommand::{QuerySubCommand, RunQuerySubCommand};
use crate::commands::update::subcommand::{RunUpdateSubCommand, UpdateSubCommand};
use crate::impls::bootstrap::starknet_chain::{BootstrapStarknetChainArgs, LoadStarknetBootstrap};
use crate::impls::bootstrap::subcommand::{BootstrapSubCommand, RunBootstrapSubCommand};
use crate::impls::build::LoadStarknetBuilder;
use crate::impls::error::ProvideCliError;
use crate::impls::subcommand::{AllSubCommands, RunAllSubCommand};

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
            UseDelegate<StarknetCommandRunnerComponents>,
        BuilderLoaderComponent:
            LoadStarknetBuilder,
        BuilderTypeComponent:
            WithType<StarknetBuilder>,
        ArgParserComponent:
            UseDelegate<StarknetParserComponents>,
    }
}

delegate_components! {
    StarknetParserComponents {
        (QueryClientStateArgs, symbol!("chain_id")): ParseFromString<ChainId>,
        (QueryClientStateArgs, symbol!("client_id")): ParseFromString<ClientId>,
        (QueryClientStateArgs, symbol!("height")): ParseFromOptionalString<u64>,

        (QueryConsensusStateArgs, symbol!("chain_id")): ParseFromString<ChainId>,
        (QueryConsensusStateArgs, symbol!("client_id")): ParseFromString<ClientId>,
        (QueryConsensusStateArgs, symbol!("query_height")): ParseFromOptionalString<u64>,
        (QueryConsensusStateArgs, symbol!("consensus_height")): ParseFromOptionalString<Height>,

        (QueryChainStatusArgs, symbol!("chain_id")): ParseFromString<ChainId>,

        (QueryBalanceArgs, symbol!("chain_id")): ParseFromString<ChainId>,
        (QueryBalanceArgs, symbol!("address")): ParseFromString<Felt>,
        (QueryBalanceArgs, symbol!("denom")): ParseFromString<Felt>,

        (UpdateClientArgs, symbol!("host_chain_id")): ParseFromString<ChainId>,
        (UpdateClientArgs, symbol!("client_id")): ParseFromString<ClientId>,
        (UpdateClientArgs, symbol!("counterparty_client_id")): ParseFromString<CosmosClientId>,
        (UpdateClientArgs, symbol!("target_height")): ParseFromOptionalString<Height>,

        (CreateClientArgs, symbol!("target_chain_id")): ParseFromString<ChainId>,
        (CreateClientArgs, symbol!("counterparty_chain_id")): ParseFromString<ChainId>,
    }
}

delegate_components! {
    StarknetCommandRunnerComponents {
        AllSubCommands: RunAllSubCommand,
        BootstrapSubCommand: RunBootstrapSubCommand,

        QuerySubCommand: RunQuerySubCommand,
        QueryClientStateArgs: RunQueryClientStateCommand,
        QueryConsensusStateArgs: RunQueryConsensusStateCommand,
        QueryChainStatusArgs: RunQueryChainStatusCommand,
        QueryBalanceArgs: RunQueryBalanceCommand,

        CreateSubCommand: RunCreateSubCommand,
        UpdateSubCommand: RunUpdateSubCommand,

        UpdateClientArgs: RunUpdateClientCommand,
        CreateClientArgs: RunCreateClientCommand,

        BootstrapStarknetChainArgs: RunBootstrapChainCommand<UpdateStarknetConfig>,
    }
}

impl<App> ProvideAnyCounterparty<App> for StarknetAppComponents
where
    App: Async,
{
    type AnyCounterparty = CosmosChain;
}

impl<Value> OutputProducer<StarknetApp, Value> for StarknetAppComponents {
    fn produce_output(_app: &StarknetApp, _value: Value) {}
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

impl CreateClientOptionsParser<StarknetApp, CreateClientArgs, Index<0>, Index<1>>
    for StarknetAppComponents
{
    async fn parse_create_client_options(
        _app: &StarknetApp,
        args: &CreateClientArgs,
        _target_chain: &StarknetChain,
        counterparty_chain: &CosmosChain,
    ) -> Result<((), CosmosCreateClientOptions), HermesError> {
        let max_clock_drift = match args.clock_drift.map(|d| d.into()) {
            Some(input) => input,
            None => {
                counterparty_chain.chain_config.clock_drift
                    + counterparty_chain.chain_config.max_block_time
            }
        };

        let settings = CosmosCreateClientOptions {
            max_clock_drift,
            trusting_period: args.trusting_period.map(|d| d.into()).unwrap_or_default(),
            trust_threshold: args
                .trust_threshold
                .map(|threshold| threshold.into())
                .unwrap_or_default(),
        };

        Ok(((), settings))
    }
}

// TODO(seanchen1991): Implement Cosmos-to-Starknet client creation
pub struct CreateCosmosClientOnStarknetArgs;

impl CreateClientOptionsParser<StarknetApp, CreateCosmosClientOnStarknetArgs, Index<1>, Index<0>>
    for StarknetAppComponents
{
    async fn parse_create_client_options(
        _app: &StarknetApp,
        _args: &CreateCosmosClientOnStarknetArgs,
        _target_chain: &CosmosChain,
        _counterparty_chain: &StarknetChain,
    ) -> Result<((), StarknetCreateClientPayloadOptions), HermesError> {
        todo!()
    }
}

pub trait CanUseStarknetApp:
    HasRuntime
    + HasLogger
    + HasConfigPath
    + CanLoadConfig
    + CanWriteConfig
    + CanWrapError<&'static str>
    + CanProduceOutput<()>
    + CanLoadBootstrap<BootstrapStarknetChainArgs>
    + CanRunCommand<AllSubCommands>
    + CanRunCommand<BootstrapSubCommand>
    + CanRunCommand<BootstrapStarknetChainArgs>
    + CanLoadBuilder<Builder = StarknetBuilder>
    + CanRunCommand<QuerySubCommand>
    + CanRunCommand<QueryClientStateArgs>
    + CanRunCommand<QueryConsensusStateArgs>
    + CanRunCommand<QueryBalanceArgs>
    + CanRunCommand<CreateSubCommand>
    + CanRunCommand<UpdateSubCommand>
    + CanRunCommand<UpdateClientArgs>
    + CanRunCommand<CreateClientArgs>
{
}

impl CanUseStarknetApp for StarknetApp {}
