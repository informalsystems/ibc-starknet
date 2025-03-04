use std::collections::BTreeMap;
use std::path::PathBuf;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::WithField;
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_error::impls::ProvideHermesError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_starknet_chain_components::types::wallet::StarknetWallet;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::impls::error::HandleStarknetChainError;
use hermes_starknet_test_components::types::genesis_config::StarknetGenesisConfig;
use hermes_starknet_test_components::types::node_config::StarknetNodeConfig;
use hermes_test_components::chain_driver::traits::chain_process::{
    ChainProcessTaker, ChainProcessTakerComponent,
};
use hermes_test_components::chain_driver::traits::types::chain::{
    ChainGetter, ChainGetterComponent, ChainTypeComponent, HasChain, ProvideChainType,
};
use tokio::process::Child;

#[cgp_context(StarknetChainDriverComponents)]
#[derive(HasField)]
pub struct StarknetChainDriver {
    pub runtime: HermesRuntime,
    pub chain: StarknetChain,
    pub chain_store_dir: PathBuf,
    pub genesis_config: StarknetGenesisConfig,
    pub node_config: StarknetNodeConfig,
    pub wallets: BTreeMap<String, StarknetWallet>,
    pub chain_process: Option<Child>,
    pub relayer_wallet: StarknetWallet,
    pub user_wallet_a: StarknetWallet,
    pub user_wallet_b: StarknetWallet,
}

delegate_components! {
    StarknetChainDriverComponents {
        ErrorTypeProviderComponent: ProvideHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
        RuntimeTypeProviderComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
    }
}

#[cgp_provider(ChainTypeComponent)]
impl ProvideChainType<StarknetChainDriver> for StarknetChainDriverComponents {
    type Chain = StarknetChain;
}

#[cgp_provider(ChainGetterComponent)]
impl ChainGetter<StarknetChainDriver> for StarknetChainDriverComponents {
    fn chain(driver: &StarknetChainDriver) -> &StarknetChain {
        &driver.chain
    }
}

#[cgp_provider(ChainProcessTakerComponent)]
impl ChainProcessTaker<StarknetChainDriver> for StarknetChainDriverComponents {
    fn take_chain_process(chain_driver: &mut StarknetChainDriver) -> Option<Child> {
        chain_driver.chain_process.take()
    }
}

pub trait CanUseStarknetChainDriver: HasChain<Chain = StarknetChain> {}

impl CanUseStarknetChainDriver for StarknetChainDriver {}
