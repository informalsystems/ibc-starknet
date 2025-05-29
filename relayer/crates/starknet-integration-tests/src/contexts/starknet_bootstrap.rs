use core::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use hermes_core::logging_components::traits::LoggerComponent;
use hermes_core::runtime_components::traits::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_core::test_components::bootstrap::traits::ChainBootstrapperComponent;
use hermes_core::test_components::chain_driver::traits::ChainTypeProviderComponent;
use hermes_core::test_components::driver::traits::ChainDriverTypeProviderComponent;
use hermes_cosmos::error::impls::UseHermesError;
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use hermes_cosmos::test_components::bootstrap::impls::BuildAndWaitChainDriver;
use hermes_cosmos::test_components::bootstrap::traits::{
    ChainCommandPathGetterComponent, ChainDriverBuilderComponent, ChainFullNodeStarterComponent,
    ChainGenesisConfigTypeComponent, ChainNodeConfigTypeComponent, ChainStoreDirGetterComponent,
};
use hermes_cosmos::tracing_logging_components::contexts::TracingLogger;
use hermes_prelude::*;
use hermes_starknet_chain_context::contexts::StarknetChain;
use hermes_starknet_chain_context::impls::HandleStarknetChainError;
use hermes_starknet_test_components::impls::{
    BootstrapStarknetDevnet, BuildChainAndDeployIbcContracts, DeployIbcContract,
    ProvideStarknetGenesisConfigType, ProvideStarknetNodeConfigType, StartStarknetDevnet,
};
use hermes_starknet_test_components::traits::IbcContractsDeployerComponent;
use starknet::core::types::contract::SierraClass;

use crate::contexts::StarknetChainDriver;
use crate::impls::BuildStarknetChainDriver;

#[cgp_context(StarknetBootstrapComponents)]
#[derive(Clone)]
pub struct StarknetBootstrap {
    pub fields: Arc<StarknetBootstrapFields>,
}

#[derive(HasField, Clone)]
pub struct StarknetBootstrapFields {
    pub runtime: HermesRuntime,
    pub chain_command_path: PathBuf,
    pub chain_store_dir: PathBuf,
    pub erc20_contract: SierraClass,
    pub ics20_contract: SierraClass,
    pub ibc_core_contract: SierraClass,
    pub comet_client_contract: SierraClass,
    pub cometbft_lib_contract: SierraClass,
    pub ics23_lib_contract: SierraClass,
    pub protobuf_lib_contract: SierraClass,
}

impl Deref for StarknetBootstrap {
    type Target = StarknetBootstrapFields;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

delegate_components! {
    StarknetBootstrapComponents {
        ErrorTypeProviderComponent:
            UseHermesError,
        ErrorRaiserComponent:
            UseDelegate<HandleStarknetChainError>,
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
            BootstrapStarknetDevnet,
        ChainFullNodeStarterComponent:
            StartStarknetDevnet,
        IbcContractsDeployerComponent:
            DeployIbcContract,
        ChainDriverBuilderComponent:
            BuildChainAndDeployIbcContracts<BuildAndWaitChainDriver<BuildStarknetChainDriver>>,
        ChainTypeProviderComponent:
            UseType<StarknetChain>,
        ChainDriverTypeProviderComponent:
            UseType<StarknetChainDriver>,
        ChainCommandPathGetterComponent:
            UseField<symbol!("chain_command_path")>,
        ChainStoreDirGetterComponent:
            UseField<symbol!("chain_store_dir")>,
    }
}

check_components! {
    CanUseStarknetBootstrap for StarknetBootstrap {
        ChainFullNodeStarterComponent,
        ChainBootstrapperComponent,
        ChainDriverBuilderComponent,
        IbcContractsDeployerComponent,
    }
}
