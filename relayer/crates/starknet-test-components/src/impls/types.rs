use cgp_core::Async;
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk::{
    ProvideChainGenesisConfigType, ProvideChainNodeConfigType,
};
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::wallet::ProvideWalletType;
use starknet::core::types::Felt;

use crate::types::genesis_config::StarknetGenesisConfig;
use crate::types::node_config::StarknetNodeConfig;
use crate::types::wallet::StarknetWallet;

pub struct ProvideStarknetTestTypes;

impl<Bootstrap: Async> ProvideChainNodeConfigType<Bootstrap> for ProvideStarknetTestTypes {
    type ChainNodeConfig = StarknetNodeConfig;
}

impl<Bootstrap: Async> ProvideChainGenesisConfigType<Bootstrap> for ProvideStarknetTestTypes {
    type ChainGenesisConfig = StarknetGenesisConfig;
}

impl<Bootstrap> ProvideWalletType<Bootstrap> for ProvideStarknetTestTypes
where
    Bootstrap: HasAddressType<Address = Felt>,
{
    type Wallet = StarknetWallet;

    fn wallet_address(wallet: &StarknetWallet) -> &Felt {
        &wallet.account_address
    }
}
