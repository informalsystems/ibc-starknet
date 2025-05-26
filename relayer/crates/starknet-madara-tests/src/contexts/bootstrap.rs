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
use hermes_cosmos_core::test_components::bootstrap::impls::BuildAndWaitChainDriver;
use hermes_cosmos_core::test_components::bootstrap::traits::{
    ChainCommandPathGetterComponent, ChainDriverBuilderComponent, ChainFullNodeStarterComponent,
    ChainGenesisConfigTypeComponent, ChainNodeConfigTypeComponent, ChainStoreDirGetterComponent,
};
use hermes_cosmos_core::tracing_logging_components::contexts::TracingLogger;
use hermes_error::impls::UseHermesError;
use hermes_prelude::*;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_starknet_test_components::impls::{
    BootstrapMadara, BuildChainAndDeployIbcContracts, DeployIbcContract,
    ProvideStarknetGenesisConfigType, ProvideStarknetNodeConfigType, StartMadaraSequencer,
};
use hermes_starknet_test_components::traits::IbcContractsDeployerComponent;
use starknet::core::types::contract::SierraClass;

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
            StartMadaraSequencer, // Start only Madara as sequencer
            // StartMadaraStack, // Switch to this to start Madara, Pathfinder, and Anvil
        IbcContractsDeployerComponent:
            DeployIbcContract,
        ChainDriverBuilderComponent:
            BuildChainAndDeployIbcContracts<BuildAndWaitChainDriver<BuildMadaraChainDriver>>,
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
    }
}
