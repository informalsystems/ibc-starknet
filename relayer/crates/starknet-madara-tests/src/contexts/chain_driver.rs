use core::marker::PhantomData;
use std::collections::BTreeMap;
use std::path::PathBuf;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use hermes_core::runtime_components::traits::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_core::test_components::chain_driver::impls::WaitChainReachHeight;
use hermes_core::test_components::chain_driver::traits::{
    ChainGetterComponent, ChainProcessTaker, ChainProcessTakerComponent,
    ChainStartupWaiterComponent, ChainTypeProviderComponent, DenomGetter, DenomGetterComponent,
    RandomAmountGeneratorComponent, RelayerWallet, StakingDenom, TransferDenom, UserWallet,
    WalletGetterComponent,
};
use hermes_error::impls::UseHermesError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::impls::types::amount::UseU256Amount;
use hermes_starknet_chain_components::types::wallet::StarknetWallet;
use hermes_starknet_test_components::types::genesis_config::StarknetGenesisConfig;
use hermes_starknet_test_components::types::node_config::StarknetNodeConfig;
use tokio::process::Child;

use crate::contexts::MadaraChain;
use crate::impls::HandleMadaraChainError;

#[cgp_context(MadaraChainDriverComponents)]
#[derive(HasField)]
pub struct MadaraChainDriver {
    pub runtime: HermesRuntime,
    pub chain: MadaraChain,
    pub chain_store_dir: PathBuf,
    pub genesis_config: StarknetGenesisConfig,
    pub node_config: StarknetNodeConfig,
    pub wallets: BTreeMap<String, StarknetWallet>,
    pub chain_processes: Vec<Child>,
    pub relayer_wallet: StarknetWallet,
    pub user_wallet_a: StarknetWallet,
    pub user_wallet_b: StarknetWallet,
}

delegate_components! {
    MadaraChainDriverComponents {
        ErrorTypeProviderComponent:
            UseHermesError,
        ErrorRaiserComponent:
            UseDelegate<HandleMadaraChainError>,
        RuntimeTypeProviderComponent:
            UseType<HermesRuntime>,
        RuntimeGetterComponent:
            UseField<symbol!("runtime")>,
        ChainTypeProviderComponent:
            UseType<MadaraChain>,
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
impl DenomGetter<MadaraChainDriver, TransferDenom> for MadaraChainDriverComponents {
    fn denom(driver: &MadaraChainDriver, _index: PhantomData<TransferDenom>) -> &StarknetAddress {
        &driver.genesis_config.transfer_denom
    }
}

#[cgp_provider(DenomGetterComponent<StakingDenom>)]
impl DenomGetter<MadaraChainDriver, StakingDenom> for MadaraChainDriverComponents {
    fn denom(driver: &MadaraChainDriver, _index: PhantomData<StakingDenom>) -> &StarknetAddress {
        &driver.genesis_config.staking_denom
    }
}

#[cgp_provider(ChainProcessTakerComponent)]
impl ChainProcessTaker<MadaraChainDriver> for MadaraChainDriverComponents {
    fn take_chain_process(chain_driver: &mut MadaraChainDriver) -> Vec<Child> {
        core::mem::take(&mut chain_driver.chain_processes)
    }
}
