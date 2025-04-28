use cgp::prelude::*;
use hermes_chain_type_components::traits::HasDenomType;

use crate::types::messages::ibc::denom::PrefixedDenom;

#[cgp_component {
    provider: CosmosTokenAddressOnStarknetQuerier,
}]
#[async_trait]
pub trait CanQueryCosmosTokenAddressOnStarknet: HasAsyncErrorType + HasDenomType {
    async fn query_cosmos_token_address_on_starknet(
        &self,
        prefixed_denom: &PrefixedDenom,
    ) -> Result<Option<Self::Denom>, Self::Error>;
}
