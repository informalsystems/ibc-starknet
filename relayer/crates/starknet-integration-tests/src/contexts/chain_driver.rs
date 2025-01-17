use std::collections::BTreeMap;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::WithField;
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_error::impls::ProvideHermesError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetterComponent, RuntimeTypeComponent};
use hermes_starknet_chain_components::types::wallet::StarknetWallet;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::impls::error::HandleStarknetChainError;
use hermes_starknet_test_components::types::genesis_config::StarknetGenesisConfig;
use hermes_starknet_test_components::types::node_config::StarknetNodeConfig;
use hermes_test_components::chain_driver::traits::chain_process::ChainProcessTaker;
use hermes_test_components::chain_driver::traits::types::chain::{
    ChainGetter, HasChain, ProvideChainType,
};
use tokio::process::Child;

#[derive(HasField)]
pub struct StarknetChainDriver {
    pub runtime: HermesRuntime,
    pub chain: StarknetChain,
    pub genesis_config: StarknetGenesisConfig,
    pub node_config: StarknetNodeConfig,
    pub wallets: BTreeMap<String, StarknetWallet>,
    pub chain_process: Option<Child>,
    pub relayer_wallet: StarknetWallet,
    pub user_wallet_a: StarknetWallet,
    pub user_wallet_b: StarknetWallet,
}

pub struct StarknetChainDriverComponents;

impl HasComponents for StarknetChainDriver {
    type Components = StarknetChainDriverComponents;
}

delegate_components! {
    StarknetChainDriverComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
        RuntimeTypeComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
    }
}

impl ProvideChainType<StarknetChainDriver> for StarknetChainDriverComponents {
    type Chain = StarknetChain;
}

impl ChainGetter<StarknetChainDriver> for StarknetChainDriverComponents {
    fn chain(driver: &StarknetChainDriver) -> &StarknetChain {
        &driver.chain
    }
}

impl ChainProcessTaker<StarknetChainDriver> for StarknetChainDriverComponents {
    fn take_chain_process(chain_driver: &mut StarknetChainDriver) -> Option<Child> {
        chain_driver.chain_process.take()
    }
}

pub trait CanUseStarknetChainDriver: HasChain<Chain = StarknetChain> {}

impl CanUseStarknetChainDriver for StarknetChainDriver {}
