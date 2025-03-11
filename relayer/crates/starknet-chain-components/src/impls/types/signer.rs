use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::default_signer::{
    DefaultSignerGetter, DefaultSignerGetterComponent,
};
use hermes_relayer_components::transaction::traits::types::signer::{
    HasSignerType, SignerTypeProvider, SignerTypeProviderComponent,
};

use crate::traits::account::HasStarknetAccountType;
use crate::traits::signer::HasStarknetSigner;
use crate::types::wallet::StarknetWallet;

pub struct UseStarknetAccountSigner;

#[cgp_provider(SignerTypeProviderComponent)]
impl<Chain> SignerTypeProvider<Chain> for UseStarknetAccountSigner
where
    Chain: HasStarknetAccountType,
{
    type Signer = StarknetWallet;
}

#[cgp_provider(DefaultSignerGetterComponent)]
impl<Chain> DefaultSignerGetter<Chain> for UseStarknetAccountSigner
where
    Chain: HasStarknetSigner + HasSignerType<Signer = StarknetWallet>,
{
    fn get_default_signer(chain: &Chain) -> &Chain::Signer {
        chain.signer()
    }
}
