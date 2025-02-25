use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::nonce::query_nonce::{
    NonceQuerier, NonceQuerierComponent,
};
use hermes_relayer_components::transaction::traits::types::nonce::HasNonceType;
use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use starknet::accounts::ConnectedAccount;
use starknet::core::types::Felt;
use starknet::providers::ProviderError;

use crate::traits::account::HasStarknetAccountType;

#[cgp_new_provider(NonceQuerierComponent)]
impl<Chain> NonceQuerier<Chain> for QueryStarknetNonce
where
    Chain: HasStarknetAccountType
        + HasSignerType<Signer = Chain::Account>
        + HasNonceType<Nonce = Felt>
        + CanRaiseAsyncError<ProviderError>,
{
    async fn query_nonce(chain: &Chain, account: &Chain::Account) -> Result<Felt, Chain::Error> {
        let nonce = account.get_nonce().await.map_err(Chain::raise_error)?;
        Ok(nonce)
    }
}
