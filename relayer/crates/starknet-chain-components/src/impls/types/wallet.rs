use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::wallet::{
    HasWalletType, ProvideWalletType, WalletSignerComponent, WalletSignerProvider,
    WalletTypeComponent,
};

use crate::impls::types::address::StarknetAddress;
use crate::types::wallet::StarknetWallet;

pub struct ProvideStarknetWallet;

#[cgp_provider(WalletTypeComponent)]
impl<Bootstrap> ProvideWalletType<Bootstrap> for ProvideStarknetWallet
where
    Bootstrap: HasAddressType<Address = StarknetAddress>,
{
    type Wallet = StarknetWallet;

    fn wallet_address(wallet: &StarknetWallet) -> &StarknetAddress {
        &wallet.account_address
    }
}

#[cgp_provider(WalletSignerComponent)]
impl<Chain> WalletSignerProvider<Chain> for ProvideStarknetWallet
where
    Chain: HasWalletType<Wallet = StarknetWallet> + HasSignerType<Signer = StarknetWallet>,
{
    fn wallet_signer(wallet: &StarknetWallet) -> &StarknetWallet {
        wallet
    }
}
