use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::default_signer::{
    DefaultSignerGetter, DefaultSignerGetterComponent,
};
use hermes_relayer_components::transaction::traits::types::signer::{
    HasSignerType, SignerTypeProvider, SignerTypeProviderComponent,
};

use crate::traits::account::{HasStarknetAccount, HasStarknetAccountType};

pub struct UseStarknetAccountSigner;

#[cgp_provider(SignerTypeProviderComponent)]
impl<Chain> SignerTypeProvider<Chain> for UseStarknetAccountSigner
where
    Chain: HasStarknetAccountType,
{
    type Signer = Chain::Account;
}

#[cgp_provider(DefaultSignerGetterComponent)]
impl<Chain> DefaultSignerGetter<Chain> for UseStarknetAccountSigner
where
    Chain: HasStarknetAccount + HasSignerType<Signer = Chain::Account>,
{
    fn get_default_signer(chain: &Chain) -> &Chain::Signer {
        chain.account()
    }
}
