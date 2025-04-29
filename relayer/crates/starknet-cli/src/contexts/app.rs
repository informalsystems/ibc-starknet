use std::path::PathBuf;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::core::field::Index;
use cgp::core::types::WithType;
use hermes_cli::commands::{CreateChannelArgs, CreateCosmosClientArgs};
use hermes_cli::impls::ParseInitCosmosChannelOptions;
use hermes_cli_components::impls::{
    CreateConnectionArgs, GetDefaultConfigField, LoadTomlConfig, ParseFromOptionalString,
    ParseFromString, QueryBalanceArgs, QueryChainStatusArgs, QueryClientStateArgs,
    QueryConsensusStateArgs, RunBootstrapChainCommand, RunCreateChannelCommand,
    RunCreateClientCommand, RunCreateConnectionCommand, RunQueryBalanceCommand,
    RunQueryChainStatusCommand, RunQueryClientStateCommand, RunQueryConsensusStateCommand,
    RunStartRelayerCommand, RunUpdateClientCommand, StartRelayerArgs, UpdateClientArgs,
    WriteTomlConfig,
};
use hermes_cli_components::traits::{
    AnyCounterpartyTypeProviderComponent, ArgParserComponent, BootstrapLoaderComponent,
    BootstrapTypeProviderComponent, BuilderLoaderComponent, BuilderTypeComponent,
    CommandRunnerComponent, ConfigLoaderComponent, ConfigPathGetterComponent, ConfigTypeComponent,
    ConfigWriterComponent, OutputProducer, OutputProducerComponent, OutputTypeComponent,
};
use hermes_core::logging_components::traits::LoggerComponent;
use hermes_core::relayer_components::error::traits::RetryableErrorComponent;
use hermes_core::runtime_components::traits::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_core::test_components::chain_driver::traits::{ConfigUpdater, ConfigUpdaterComponent};
use hermes_cosmos::error::types::HermesError;
use hermes_cosmos::integration_tests::contexts::CosmosChainDriver;
use hermes_cosmos::relayer::contexts::CosmosChain;
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use hermes_cosmos::tracing_logging_components::contexts::TracingLogger;
use hermes_prelude::*;
use hermes_starknet_chain_components::impls::{
    StarknetAddress, StarknetChainConfig, StarknetContractAddresses, StarknetContractClasses,
    StarknetRelayerConfig,
};
use hermes_starknet_chain_components::types::ClientId;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_integration_tests::contexts::chain_driver::StarknetChainDriver;
use hermes_starknet_integration_tests::contexts::osmosis_bootstrap::OsmosisBootstrap;
use hermes_starknet_integration_tests::contexts::starknet_bootstrap::StarknetBootstrap;
use hermes_starknet_relayer::contexts::builder::StarknetBuilder;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChainId, ClientId as CosmosClientId, PortId};
use toml::to_string_pretty;

use crate::commands::all::{AllSubCommands, RunAllSubCommand};
use crate::commands::bootstrap::{BootstrapSubCommand, RunBootstrapSubCommand};
use crate::commands::create::subcommand::{CreateSubCommand, RunCreateSubCommand};
use crate::commands::query::subcommand::{QuerySubCommand, RunQuerySubCommand};
use crate::commands::start::{RunStartSubCommand, StartSubCommand};
use crate::commands::update::subcommand::{RunUpdateSubCommand, UpdateSubCommand};
use crate::impls::bootstrap::osmosis_chain::{BootstrapOsmosisChainArgs, LoadOsmosisBootstrap};
use crate::impls::bootstrap::starknet_chain::{BootstrapStarknetChainArgs, LoadStarknetBootstrap};
use crate::impls::build::LoadStarknetBuilder;
use crate::impls::create_client::CreateStarknetClientArgs;
use crate::impls::error::ProvideCliError;

#[cgp_context(StarknetAppComponents)]
#[derive(HasField)]
pub struct StarknetApp {
    pub config_path: PathBuf,
    pub runtime: HermesRuntime,
}

pub struct StarknetParserComponents;

delegate_components! {
    StarknetAppComponents {
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
        AnyCounterpartyTypeProviderComponent:
            UseType<CosmosChain>,
        ConfigTypeComponent:
            WithType<StarknetRelayerConfig>,
        BootstrapTypeProviderComponent:
            UseDelegate<StarknetBootstrapTypes>,
        OutputTypeComponent:
            WithType<()>,
        BootstrapLoaderComponent:
            UseDelegate<StarknetBoostrapLoaders>,
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
        (QueryBalanceArgs, symbol!("address")): ParseFromString<StarknetAddress>,
        (QueryBalanceArgs, symbol!("denom")): ParseFromString<StarknetAddress>,

        (UpdateClientArgs, symbol!("host_chain_id")): ParseFromString<ChainId>,
        (UpdateClientArgs, symbol!("client_id")): ParseFromString<ClientId>,
        (UpdateClientArgs, symbol!("counterparty_client_id")): ParseFromString<CosmosClientId>,
        (UpdateClientArgs, symbol!("target_height")): ParseFromOptionalString<Height>,

        (CreateCosmosClientArgs, symbol!("target_chain_id")): ParseFromString<ChainId>,
        (CreateCosmosClientArgs, symbol!("counterparty_chain_id")): ParseFromString<ChainId>,

        (CreateStarknetClientArgs, symbol!("target_chain_id")): ParseFromString<ChainId>,
        (CreateStarknetClientArgs, symbol!("counterparty_chain_id")): ParseFromString<ChainId>,

        (CreateConnectionArgs, symbol!("target_chain_id")): ParseFromString<ChainId>,
        (CreateConnectionArgs, symbol!("target_client_id")): ParseFromString<ClientId>,
        (CreateConnectionArgs, symbol!("counterparty_chain_id")): ParseFromString<ChainId>,
        (CreateConnectionArgs, symbol!("counterparty_client_id")): ParseFromString<ClientId>,

        (CreateChannelArgs, symbol!("target_chain_id")): ParseFromString<ChainId>,
        (CreateChannelArgs, symbol!("target_client_id")): ParseFromString<ClientId>,
        (CreateChannelArgs, symbol!("target_port_id")): ParseFromString<PortId>,
        (CreateChannelArgs, symbol!("counterparty_chain_id")): ParseFromString<ChainId>,
        (CreateChannelArgs, symbol!("counterparty_client_id")): ParseFromString<ClientId>,
        (CreateChannelArgs, symbol!("counterparty_port_id")): ParseFromString<PortId>,
        (CreateChannelArgs, symbol!("init_channel_options")): ParseInitCosmosChannelOptions,

        (StartRelayerArgs, symbol!("chain_id_a")): ParseFromString<ChainId>,
        (StartRelayerArgs, symbol!("client_id_a")): ParseFromString<ClientId>,
        (StartRelayerArgs, symbol!("chain_id_b")): ParseFromString<ChainId>,
        (StartRelayerArgs, symbol!("client_id_b")): ParseFromString<ClientId>,
    }
}

pub struct StarknetBootstrapTypes;

delegate_components! {
    StarknetBootstrapTypes {
        StarknetChain: UseType<StarknetBootstrap>,
        CosmosChain: UseType<OsmosisBootstrap>,
    }
}

pub struct StarknetBoostrapLoaders;

delegate_components! {
    StarknetBoostrapLoaders {
        StarknetChain: LoadStarknetBootstrap,
        CosmosChain: LoadOsmosisBootstrap,
    }
}

pub struct StarknetCommandRunnerComponents;

delegate_components! {
    StarknetCommandRunnerComponents {
        AllSubCommands: RunAllSubCommand,
        BootstrapSubCommand: RunBootstrapSubCommand,

        StartRelayerArgs: RunStartRelayerCommand<Index<0>, Index<1>>,
        StartSubCommand: RunStartSubCommand,

        QuerySubCommand: RunQuerySubCommand,
        QueryClientStateArgs: RunQueryClientStateCommand,
        QueryConsensusStateArgs: RunQueryConsensusStateCommand,
        QueryChainStatusArgs: RunQueryChainStatusCommand,
        QueryBalanceArgs: RunQueryBalanceCommand,

        CreateSubCommand: RunCreateSubCommand,
        UpdateSubCommand: RunUpdateSubCommand,

        UpdateClientArgs: RunUpdateClientCommand,
        CreateCosmosClientArgs: RunCreateClientCommand<Index<0>, Index<1>>,
        CreateStarknetClientArgs: RunCreateClientCommand<Index<1>, Index<0>>,

        CreateConnectionArgs: RunCreateConnectionCommand,
        CreateChannelArgs: RunCreateChannelCommand,

        BootstrapStarknetChainArgs: RunBootstrapChainCommand<StarknetChain, UpdateStarknetConfig>,
        BootstrapOsmosisChainArgs: RunBootstrapChainCommand<CosmosChain, UpdateStarknetConfig>,
    }
}

#[cgp_provider(OutputProducerComponent)]
impl<Value> OutputProducer<StarknetApp, Value> for StarknetAppComponents {
    fn produce_output(_app: &StarknetApp, _value: Value) {}
}

#[cgp_new_provider(ConfigUpdaterComponent)]
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

        let contract_addresses = StarknetContractAddresses {
            ibc_client: chain_driver
                .chain
                .ibc_client_contract_address
                .get()
                .cloned(),
            ibc_core: chain_driver.chain.ibc_core_contract_address.get().cloned(),
            ibc_ics20: chain_driver.chain.ibc_ics20_contract_address.get().cloned(),
        };

        let event_encoding = &chain_driver.chain.event_encoding;

        let contract_classes = StarknetContractClasses {
            erc20: event_encoding
                .erc20_hashes
                .get()
                .and_then(|hashes| hashes.iter().cloned().next()),
            ics20: event_encoding
                .ics20_hashes
                .get()
                .and_then(|hashes| hashes.iter().cloned().next()),
            ibc_client: event_encoding
                .ibc_client_hashes
                .get()
                .and_then(|hashes| hashes.iter().cloned().next()),
        };

        let relayer_wallet_path = chain_driver
            .chain_store_dir
            .join("wallets/relayer.toml")
            .display()
            .to_string();

        let chain_config = StarknetChainConfig {
            json_rpc_url: format!(
                "http://{}:{}/",
                chain_driver.node_config.rpc_addr, chain_driver.node_config.rpc_port
            ),
            relayer_wallet: relayer_wallet_path,
            poll_interval: chain_driver.chain.poll_interval,
            block_time: chain_driver.chain.block_time,
            contract_addresses,
            contract_classes,
        };

        let chain_config_str = to_string_pretty(&chain_config)?;

        config.starknet_chain_config = Some(chain_config);

        Ok(chain_config_str)
    }
}

#[cgp_provider(ConfigUpdaterComponent)]
impl ConfigUpdater<CosmosChainDriver, StarknetRelayerConfig> for UpdateStarknetConfig {
    fn update_config(
        chain_driver: &CosmosChainDriver,
        config: &mut StarknetRelayerConfig,
    ) -> Result<String, HermesError> {
        let chain_config = chain_driver.chain.chain_config.clone();
        let chain_config_str = to_string_pretty(&chain_driver.chain.chain_config)?;
        config.cosmos_chain_config = Some(chain_config);

        Ok(chain_config_str)
    }
}

check_components! {
    CanUseStarknetApp for StarknetApp {
        RuntimeGetterComponent,
        ConfigPathGetterComponent,
        ConfigLoaderComponent,
        ConfigWriterComponent,
        OutputProducerComponent: (),
        BootstrapTypeProviderComponent: StarknetChain,
        BootstrapLoaderComponent: [
            (StarknetChain, BootstrapStarknetChainArgs),
            (CosmosChain, BootstrapOsmosisChainArgs),
        ],
        CommandRunnerComponent: [
            AllSubCommands,
            StartSubCommand,
            BootstrapSubCommand,
            BootstrapStarknetChainArgs,
            BootstrapOsmosisChainArgs,
            QuerySubCommand,
            QueryClientStateArgs,
            QueryBalanceArgs,
            CreateSubCommand,
            UpdateSubCommand,
            UpdateClientArgs,
            CreateCosmosClientArgs,
            CreateStarknetClientArgs,
            StartRelayerArgs,
            CreateConnectionArgs,
            CreateChannelArgs,
        ],
    }
}
