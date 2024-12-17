use hermes_starknet_chain_components::types::wallet::StarknetWallet;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::wallet::ProvideWalletType;
use starknet::core::types::Felt;

pub struct ProvideStarknetWalletType;

impl<Bootstrap> ProvideWalletType<Bootstrap> for ProvideStarknetWalletType
where
    Bootstrap: HasAddressType<Address = Felt>,
{
    type Wallet = StarknetWallet;

    fn wallet_address(wallet: &StarknetWallet) -> &Felt {
        &wallet.account_address
    }
}
