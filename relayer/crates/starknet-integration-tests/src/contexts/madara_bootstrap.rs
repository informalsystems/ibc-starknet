use std::path::PathBuf;

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
use hermes_logger::UseHermesLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeProviderComponent,
};
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::impls::error::HandleStarknetChainError;
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
use starknet::core::types::contract::SierraClass;

use crate::contexts::chain_driver::StarknetChainDriver;
use crate::impls::BuildStarknetChainDriver;

#[cgp_context(MadaraBootstrapComponents)]
#[derive(HasField)]
pub struct MadaraBootstrap {
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
            UseDelegate<HandleStarknetChainError>,
        RuntimeTypeProviderComponent:
            UseType<HermesRuntime>,
        RuntimeGetterComponent:
            UseField<symbol!("runtime")>,
        [
            LoggerTypeProviderComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            UseHermesLogger,
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
            BuildAndWaitChainDriver<BuildStarknetChainDriver>,
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
    CanUseStarknetBootstrap for MadaraBootstrap {
        ChainFullNodeStarterComponent,
        ChainBootstrapperComponent,
        ChainDriverBuilderComponent,
        IbcContractsDeployerComponent,
    }
}
