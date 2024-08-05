use std::collections::BTreeMap;

use cgp_core::error::{DelegateErrorRaiser, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_error::impls::ProvideHermesError;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::impls::error::HandleStarknetError;
use hermes_starknet_test_components::types::genesis_config::StarknetGenesisConfig;
use hermes_starknet_test_components::types::node_config::StarknetNodeConfig;
use hermes_starknet_test_components::types::wallet::StarknetWallet;
use hermes_test_components::chain_driver::traits::types::chain::ProvideChainType;
use tokio::process::Child;

pub struct StarknetChainDriver {
    pub chain: StarknetChain,
    pub genesis_config: StarknetGenesisConfig,
    pub node_config: StarknetNodeConfig,
    pub wallets: BTreeMap<String, StarknetWallet>,
    pub chain_process: Child,
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
        ErrorRaiserComponent: DelegateErrorRaiser<HandleStarknetError>,
    }
}

impl ProvideChainType<StarknetChainDriver> for StarknetChainDriverComponents {
    type Chain = StarknetChain;
}
