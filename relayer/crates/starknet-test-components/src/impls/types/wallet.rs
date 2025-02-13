use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::types::wallet::StarknetWallet;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::wallet::ProvideWalletType;

pub struct ProvideStarknetWalletType;

impl<Bootstrap> ProvideWalletType<Bootstrap> for ProvideStarknetWalletType
where
    Bootstrap: HasAddressType<Address = StarknetAddress>,
{
    type Wallet = StarknetWallet;

    fn wallet_address(wallet: &StarknetWallet) -> &StarknetAddress {
        &wallet.account_address
    }
}
