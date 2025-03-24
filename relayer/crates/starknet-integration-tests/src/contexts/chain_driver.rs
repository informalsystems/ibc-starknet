use core::marker::PhantomData;
use std::collections::BTreeMap;
use std::path::PathBuf;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use hermes_error::impls::UseHermesError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::random::CanGenerateRandom;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::types::amount::StarknetAmount;
use hermes_starknet_chain_components::types::wallet::StarknetWallet;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::impls::error::HandleStarknetChainError;
use hermes_starknet_test_components::types::genesis_config::StarknetGenesisConfig;
use hermes_starknet_test_components::types::node_config::StarknetNodeConfig;
use hermes_test_components::chain_driver::impls::wait::WaitChainReachHeight;
use hermes_test_components::chain_driver::traits::chain_process::{
    ChainProcessTaker, ChainProcessTakerComponent,
};
use hermes_test_components::chain_driver::traits::fields::amount::{
    RandomAmountGenerator, RandomAmountGeneratorComponent,
};
use hermes_test_components::chain_driver::traits::fields::denom::{
    DenomGetter, DenomGetterComponent, StakingDenom, TransferDenom,
};
use hermes_test_components::chain_driver::traits::fields::wallet::{
    RelayerWallet, UserWallet, WalletGetterComponent,
};
use hermes_test_components::chain_driver::traits::types::chain::{
    ChainGetter, ChainGetterComponent, ChainTypeComponent, HasChain, ProvideChainType,
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
        WalletGetterComponent<RelayerWallet>:
            UseField<symbol!("relayer_wallet")>,
        WalletGetterComponent<UserWallet<0>>:
            UseField<symbol!("user_wallet_a")>,
        WalletGetterComponent<UserWallet<1>>:
            UseField<symbol!("user_wallet_b")>,
        ChainStartupWaiterComponent:
            WaitChainReachHeight<1>,
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

#[cgp_provider(RandomAmountGeneratorComponent)]
impl RandomAmountGenerator<StarknetChainDriver> for StarknetChainDriverComponents {
    async fn random_amount(
        chain_driver: &StarknetChainDriver,
        min: usize,
        max: &StarknetAmount,
    ) -> StarknetAmount {
        // FIXME: figure how to generate random U256 amount

        let max_quantity = max.quantity.low();

        let quantity = chain_driver
            .runtime
            .random_range(min as u128, max_quantity)
            .await;

        StarknetAmount {
            quantity: quantity.into(),
            token_address: max.token_address.clone(),
        }
    }
}

pub trait CanUseStarknetChainDriver: HasChain<Chain = StarknetChain> {}

impl CanUseStarknetChainDriver for StarknetChainDriver {}
