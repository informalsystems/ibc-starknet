use hermes_core::relayer_components::transaction::traits::{
    HasNonceType, HasSignerType, NonceQuerier, NonceQuerierComponent,
};
use hermes_prelude::*;
use hermes_starknet_chain_components::traits::{CanBuildAccountFromSigner, HasStarknetAccountType};
use starknet::accounts::ConnectedAccount;
use starknet::core::types::Felt;
use starknet::providers::ProviderError;

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
