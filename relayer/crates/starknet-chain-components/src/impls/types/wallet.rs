use cgp::prelude::*;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::wallet::{
    ProvideWalletType, WalletTypeComponent,
};

use crate::impls::types::address::StarknetAddress;
use crate::types::wallet::StarknetWallet;

pub struct UseStarknetWallet;

#[cgp_provider(WalletTypeComponent)]
impl<Bootstrap> ProvideWalletType<Bootstrap> for UseStarknetWallet
where
    Bootstrap: HasAddressType<Address = StarknetAddress>,
{
    type Wallet = StarknetWallet;

    fn wallet_address(wallet: &StarknetWallet) -> &StarknetAddress {
        &wallet.account_address
    }
}
