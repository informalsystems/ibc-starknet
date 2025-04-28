use cgp::prelude::*;
use hermes_core::chain_type_components::traits::HasDenomType;

use crate::impls::queries::token_address::{
    CreateCosmosTokenAddressOnStarknet, GetCosmosTokenAddressOnStarknet,
};
use crate::traits::queries::token_address::{
    CosmosTokenAddressOnStarknetQuerier, CosmosTokenAddressOnStarknetQuerierComponent,
};
use crate::types::messages::ibc::denom::PrefixedDenom;

#[cgp_new_provider(CosmosTokenAddressOnStarknetQuerierComponent)]
impl<Chain> CosmosTokenAddressOnStarknetQuerier<Chain> for GetOrCreateCosmosTokenAddressOnStarknet
where
    Chain: HasAsyncErrorType + HasDenomType,
    GetCosmosTokenAddressOnStarknet: CosmosTokenAddressOnStarknetQuerier<Chain>,
    CreateCosmosTokenAddressOnStarknet: CosmosTokenAddressOnStarknetQuerier<Chain>,
{
    async fn query_cosmos_token_address_on_starknet(
        chain: &Chain,
        prefixed_denom: &PrefixedDenom,
    ) -> Result<Option<Chain::Denom>, Chain::Error> {
        let m_denom = GetCosmosTokenAddressOnStarknet::query_cosmos_token_address_on_starknet(
            chain,
            prefixed_denom,
        )
        .await?;

        match m_denom {
            Some(denom) => Ok(Some(denom)),
            None => {
                CreateCosmosTokenAddressOnStarknet::query_cosmos_token_address_on_starknet(
                    chain,
                    prefixed_denom,
                )
                .await
            }
        }
    }
}
