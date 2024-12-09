use alloc::sync::Arc;
use std::path::PathBuf;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::impls::use_field::WithField;
use cgp::core::types::impls::WithType;
use cgp::prelude::*;
use hermes_cosmos_integration_tests::impls::bootstrap::build_cosmos_chain::BuildCosmosChainWithNodeConfig;
use hermes_cosmos_integration_tests::impls::bootstrap::build_cosmos_chain_driver::BuildCosmosChainDriver;
use hermes_cosmos_integration_tests::impls::bootstrap::relayer_chain_config::BuildRelayerChainConfig;
use hermes_cosmos_integration_tests::impls::bootstrap::types::ProvideCosmosBootstrapChainTypes;
use hermes_cosmos_integration_tests::traits::bootstrap::build_chain::ChainBuilderWithNodeConfigComponent;
use hermes_cosmos_integration_tests::traits::bootstrap::compat_mode::{
    CompatModeGetter, CompatModeGetterComponent, UseCompatMode37,
};
use hermes_cosmos_integration_tests::traits::bootstrap::cosmos_builder::CosmosBuilderGetter;
use hermes_cosmos_integration_tests::traits::bootstrap::relayer_chain_config::RelayerChainConfigBuilderComponent;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk_legacy::*;
use hermes_cosmos_test_components::bootstrap::impls::generator::wallet_config::GenerateStandardWalletConfig;
use hermes_cosmos_test_components::bootstrap::impls::modifiers::no_modify_comet_config::NoModifyCometConfig;
use hermes_cosmos_test_components::bootstrap::impls::modifiers::no_modify_cosmos_sdk_config::NoModifyCosmosSdkConfig;
use hermes_cosmos_test_components::bootstrap::impls::modifiers::no_modify_genesis_config::NoModifyGenesisConfig;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::ChainDriverBuilderComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::account_prefix::AccountPrefixGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_store_dir::ChainStoreDirGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::denom::{
    DenomForStaking, DenomForTransfer, DenomPrefixGetter,
};
use hermes_cosmos_test_components::bootstrap::traits::fields::dynamic_gas_fee::DynamicGasGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::random_id::RandomIdFlagGetter;
use hermes_cosmos_test_components::bootstrap::traits::generator::generate_wallet_config::WalletConfigGeneratorComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_comet_config::{
    CometConfigModifier, CometConfigModifierComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_cosmos_sdk_config::CosmosSdkConfigModifierComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifierComponent;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetterComponent, RuntimeTypeComponent};
use hermes_test_components::chain_driver::traits::types::chain::ChainTypeComponent;
use hermes_test_components::driver::traits::types::chain_driver::ChainDriverTypeComponent;
use hermes_wasm_test_components::impls::bootstrap::build_chain_driver::BuildChainDriverAndInitWasmClient;
use hermes_wasm_test_components::impls::bootstrap::genesis_config::ModifyWasmGenesisConfig;
use hermes_wasm_test_components::impls::bootstrap::node_config::ModifyWasmNodeConfig;
use hermes_wasm_test_components::traits::bootstrap::client_byte_code::WasmClientByteCodeGetter;
use hermes_wasm_test_components::traits::bootstrap::gov_authority::GovernanceProposalAuthorityGetter;
use toml::Value;

use crate::impls::dynamic_gas::NoDynamicGas;

/**
   A bootstrap context for bootstrapping a new Cosmos chain, and builds
   a `CosmosChainDriver`.
*/
#[derive(HasField)]
pub struct CosmosWithWasmClientBootstrap {
    pub runtime: HermesRuntime,
    pub builder: Arc<CosmosBuilder>,
    pub should_randomize_identifiers: bool,
    pub chain_store_dir: PathBuf,
    pub chain_command_path: PathBuf,
    pub account_prefix: String,
    pub staking_denom: String,
    pub transfer_denom: String,
    pub wasm_client_byte_code: Vec<u8>,
    pub governance_proposal_authority: String,
}

impl CanUseLegacyCosmosSdkChainBootstrapper for CosmosWithWasmClientBootstrap {}

pub struct CosmosWithWasmClientBootstrapComponents;

impl HasComponents for CosmosWithWasmClientBootstrap {
    type Components = CosmosWithWasmClientBootstrapComponents;
}

with_legacy_cosmos_sdk_bootstrap_components! {
    | Components | {
        delegate_components! {
            CosmosWithWasmClientBootstrapComponents {
                Components: LegacyCosmosSdkBootstrapComponents,
            }
        }
    }
}

delegate_components! {
    CosmosWithWasmClientBootstrapComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DebugError,
        RuntimeTypeComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        WalletConfigGeneratorComponent: GenerateStandardWalletConfig,
        [
            ChainTypeComponent,
            ChainDriverTypeComponent,
        ]:
            ProvideCosmosBootstrapChainTypes,
        RelayerChainConfigBuilderComponent:
            BuildRelayerChainConfig,
        ChainBuilderWithNodeConfigComponent:
            BuildCosmosChainWithNodeConfig,
        ChainDriverBuilderComponent:
            BuildChainDriverAndInitWasmClient<BuildCosmosChainDriver>,
        CosmosGenesisConfigModifierComponent:
            ModifyWasmGenesisConfig<NoModifyGenesisConfig>,
        CometConfigModifierComponent:
            ModifyWasmNodeConfig<NoModifyCometConfig>,
        CosmosSdkConfigModifierComponent:
            NoModifyCosmosSdkConfig,
        CompatModeGetterComponent:
            UseCompatMode37,
        DynamicGasGetterComponent:
            NoDynamicGas,
    }
}

pub struct ModifyOsmosisConfig;

impl<Bootstrap> CometConfigModifier<Bootstrap> for ModifyOsmosisConfig
where
    Bootstrap: CanRaiseError<&'static str>,
{
    fn modify_comet_config(
        _bootstrap: &Bootstrap,
        _comet_config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        Ok(())
    }
}

impl ChainStoreDirGetter<CosmosWithWasmClientBootstrap>
    for CosmosWithWasmClientBootstrapComponents
{
    fn chain_store_dir(bootstrap: &CosmosWithWasmClientBootstrap) -> &PathBuf {
        &bootstrap.chain_store_dir
    }
}

impl ChainCommandPathGetter<CosmosWithWasmClientBootstrap>
    for CosmosWithWasmClientBootstrapComponents
{
    fn chain_command_path(bootstrap: &CosmosWithWasmClientBootstrap) -> &PathBuf {
        &bootstrap.chain_command_path
    }
}

impl RandomIdFlagGetter<CosmosWithWasmClientBootstrap> for CosmosWithWasmClientBootstrapComponents {
    fn should_randomize_identifiers(bootstrap: &CosmosWithWasmClientBootstrap) -> bool {
        bootstrap.should_randomize_identifiers
    }
}

impl DenomPrefixGetter<CosmosWithWasmClientBootstrap, DenomForStaking>
    for CosmosWithWasmClientBootstrapComponents
{
    fn denom_prefix(bootstrap: &CosmosWithWasmClientBootstrap, _label: DenomForStaking) -> &str {
        &bootstrap.staking_denom
    }
}

impl DenomPrefixGetter<CosmosWithWasmClientBootstrap, DenomForTransfer>
    for CosmosWithWasmClientBootstrapComponents
{
    fn denom_prefix(bootstrap: &CosmosWithWasmClientBootstrap, _label: DenomForTransfer) -> &str {
        &bootstrap.transfer_denom
    }
}

impl AccountPrefixGetter<CosmosWithWasmClientBootstrap>
    for CosmosWithWasmClientBootstrapComponents
{
    fn account_prefix(bootstrap: &CosmosWithWasmClientBootstrap) -> &str {
        &bootstrap.account_prefix
    }
}

impl GasDenomGetter<CosmosWithWasmClientBootstrap> for CosmosWithWasmClientBootstrapComponents {
    fn gas_denom(bootstrap: &CosmosWithWasmClientBootstrap) -> &str {
        &bootstrap.staking_denom
    }
}

impl CosmosBuilderGetter<CosmosWithWasmClientBootstrap>
    for CosmosWithWasmClientBootstrapComponents
{
    fn cosmos_builder(bootstrap: &CosmosWithWasmClientBootstrap) -> &CosmosBuilder {
        &bootstrap.builder
    }
}

impl WasmClientByteCodeGetter<CosmosWithWasmClientBootstrap>
    for CosmosWithWasmClientBootstrapComponents
{
    fn wasm_client_byte_code(bootstrap: &CosmosWithWasmClientBootstrap) -> &Vec<u8> {
        &bootstrap.wasm_client_byte_code
    }
}

impl GovernanceProposalAuthorityGetter<CosmosWithWasmClientBootstrap>
    for CosmosWithWasmClientBootstrapComponents
{
    fn governance_proposal_authority(bootstrap: &CosmosWithWasmClientBootstrap) -> &String {
        &bootstrap.governance_proposal_authority
    }
}
