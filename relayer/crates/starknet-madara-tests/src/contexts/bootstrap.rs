use core::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use hermes_cosmos_test_components::bootstrap::impls::chain::build_wait::BuildAndWaitChainDriver;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::ChainDriverBuilderComponent;
use hermes_cosmos_test_components::bootstrap::traits::chain::start_chain::ChainFullNodeStarterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_store_dir::ChainStoreDirGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::ChainNodeConfigTypeComponent;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::ChainGenesisConfigTypeComponent;
use hermes_error::impls::UseHermesError;
use hermes_logging_components::traits::logger::LoggerComponent;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_starknet_test_components::impls::bootstrap::deploy_contracts::DeployIbcContract;
use hermes_starknet_test_components::impls::bootstrap_madara::{
    BootstrapMadara, StartMadaraDevnet,
};
use hermes_starknet_test_components::impls::types::genesis_config::ProvideStarknetGenesisConfigType;
use hermes_starknet_test_components::impls::types::node_config::ProvideStarknetNodeConfigType;
use hermes_starknet_test_components::traits::IbcContractsDeployerComponent;
use hermes_test_components::bootstrap::traits::chain::ChainBootstrapperComponent;
use hermes_test_components::chain_driver::traits::types::chain::ChainTypeProviderComponent;
use hermes_test_components::driver::traits::types::chain_driver::ChainDriverTypeProviderComponent;
use hermes_tracing_logging_components::contexts::logger::TracingLogger;
use starknet_v13::core::types::contract::SierraClass;

use crate::contexts::{MadaraChain, MadaraChainDriver};
use crate::impls::{BuildMadaraChainDriver, HandleMadaraChainError};

#[cgp_context(MadaraBootstrapComponents)]
pub struct MadaraBootstrap {
    pub fields: Arc<MadaraBootstrapFields>,
}

impl Deref for MadaraBootstrap {
    type Target = MadaraBootstrapFields;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

#[derive(HasField, Clone)]
pub struct MadaraBootstrapFields {
    pub runtime: HermesRuntime,
    pub chain_command_path: PathBuf,
    pub chain_store_dir: PathBuf,
    pub erc20_contract: SierraClass,
    pub ics20_contract: SierraClass,
    pub ibc_core_contract: SierraClass,
    pub comet_client_contract: SierraClass,
}

delegate_components! {
    MadaraBootstrapComponents {
        ErrorTypeProviderComponent:
            UseHermesError,
        ErrorRaiserComponent:
            UseDelegate<HandleMadaraChainError>,
        RuntimeTypeProviderComponent:
            UseType<HermesRuntime>,
        RuntimeGetterComponent:
            UseField<symbol!("runtime")>,
        LoggerComponent:
            TracingLogger,
        ChainNodeConfigTypeComponent:
            ProvideStarknetNodeConfigType,
        ChainGenesisConfigTypeComponent:
            ProvideStarknetGenesisConfigType,
        ChainBootstrapperComponent:
            BootstrapMadara,
        ChainFullNodeStarterComponent:
            StartMadaraDevnet,
        IbcContractsDeployerComponent:
            DeployIbcContract,
        ChainDriverBuilderComponent:
            BuildAndWaitChainDriver<BuildMadaraChainDriver>,
            // FIXME: Deploying Cairo contracts with Madara fails with 500 Internal server error
            // Note: This might be caused by the contracts built with newer versions of Cairo
            // BuildChainAndDeployIbcContracts<BuildAndWaitChainDriver<BuildMadaraChainDriver>>,
        ChainTypeProviderComponent:
            UseType<MadaraChain>,
        ChainDriverTypeProviderComponent:
            UseType<MadaraChainDriver>,
        ChainCommandPathGetterComponent:
            UseField<symbol!("chain_command_path")>,
        ChainStoreDirGetterComponent:
            UseField<symbol!("chain_store_dir")>,
    }
}

check_components! {
    CanUseStarknetBootstrap for MadaraBootstrap {
        ChainFullNodeStarterComponent,
        ChainBootstrapperComponent,
        ChainDriverBuilderComponent,
        IbcContractsDeployerComponent,
    }
}
