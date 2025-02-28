use cgp::prelude::*;
use hermes_chain_components::traits::types::status::HasChainStatusType;
use hermes_chain_type_components::traits::types::height::HasHeightType;

#[cgp_component {
  name: ChainStatusAtHeightQuerierComponent,
  provider: ChainStatusAtHeightQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryChainStatusAtHeight:
    HasChainStatusType + HasHeightType + HasAsyncErrorType
{
    async fn query_chain_status_at_height(
        &self,
        height: &Self::Height,
    ) -> Result<Self::ChainStatus, Self::Error>;
}
