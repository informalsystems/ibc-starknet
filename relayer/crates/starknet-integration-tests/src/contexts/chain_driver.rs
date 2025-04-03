use core::marker::PhantomData;
use std::collections::BTreeMap;
use std::path::PathBuf;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use hermes_error::impls::UseHermesError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::impls::types::amount::UseU256Amount;
use hermes_starknet_chain_components::types::wallet::StarknetWallet;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::impls::error::HandleStarknetChainError;
use hermes_starknet_test_components::types::genesis_config::StarknetGenesisConfig;
use hermes_starknet_test_components::types::node_config::StarknetNodeConfig;
use hermes_test_components::chain_driver::impls::wait::WaitChainReachHeight;
use hermes_test_components::chain_driver::traits::chain_process::{
    ChainProcessTaker, ChainProcessTakerComponent,
};
use hermes_test_components::chain_driver::traits::fields::amount::RandomAmountGeneratorComponent;
use hermes_test_components::chain_driver::traits::fields::denom::{
    DenomGetter, DenomGetterComponent, StakingDenom, TransferDenom,
};
use hermes_test_components::chain_driver::traits::fields::wallet::{
    RelayerWallet, UserWallet, WalletGetterComponent,
};
use hermes_test_components::chain_driver::traits::types::chain::{
    ChainGetterComponent, ChainTypeProviderComponent, HasChain,
};
use hermes_test_components::chain_driver::traits::wait::ChainStartupWaiterComponent;
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
        ErrorTypeProviderComponent:
            UseHermesError,
        ErrorRaiserComponent:
            UseDelegate<HandleStarknetChainError>,
        RuntimeTypeProviderComponent:
            UseType<HermesRuntime>,
        RuntimeGetterComponent:
            UseField<symbol!("runtime")>,
        ChainTypeProviderComponent:
            UseType<StarknetChain>,
        ChainGetterComponent:
            UseField<symbol!("chain")>,
        WalletGetterComponent<RelayerWallet>:
            UseField<symbol!("relayer_wallet")>,
        WalletGetterComponent<UserWallet<0>>:
            UseField<symbol!("user_wallet_a")>,
        WalletGetterComponent<UserWallet<1>>:
            UseField<symbol!("user_wallet_b")>,
        ChainStartupWaiterComponent:
            WaitChainReachHeight<1>,
        RandomAmountGeneratorComponent:
            UseU256Amount,
    }
}

#[cgp_provider(DenomGetterComponent<TransferDenom>)]
impl DenomGetter<StarknetChainDriver, TransferDenom> for StarknetChainDriverComponents {
    fn denom(driver: &StarknetChainDriver, _index: PhantomData<TransferDenom>) -> &StarknetAddress {
        &driver.genesis_config.transfer_denom
    }
}

#[cgp_provider(DenomGetterComponent<StakingDenom>)]
impl DenomGetter<StarknetChainDriver, StakingDenom> for StarknetChainDriverComponents {
    fn denom(driver: &StarknetChainDriver, _index: PhantomData<StakingDenom>) -> &StarknetAddress {
        &driver.genesis_config.staking_denom
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
