use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::nonce::query_nonce::{
    NonceQuerier, NonceQuerierComponent,
};
use hermes_relayer_components::transaction::traits::types::nonce::HasNonceType;
use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use hermes_starknet_chain_components::traits::account::{
    CanBuildAccountFromSigner, HasStarknetAccountType,
};
use starknet_v13::accounts::ConnectedAccount;
use starknet_v13::core::types::Felt;
use starknet_v13::providers::ProviderError;

#[cgp_new_provider(NonceQuerierComponent)]
impl<Chain> NonceQuerier<Chain> for QueryStarknetNonce
where
    Chain: HasStarknetAccountType<Account: ConnectedAccount>
        + HasSignerType
        + CanBuildAccountFromSigner
        + HasNonceType<Nonce = Felt>
        + CanRaiseAsyncError<ProviderError>,
{
    async fn query_nonce(chain: &Chain, signer: &Chain::Signer) -> Result<Felt, Chain::Error> {
        let account = chain.build_account_from_signer(signer);
        let nonce = account.get_nonce().await.map_err(Chain::raise_error)?;
        Ok(nonce)
    }
}
