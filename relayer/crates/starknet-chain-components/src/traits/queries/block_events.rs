use cgp::prelude::*;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::address::HasAddressType;

#[cgp_component {
    provider: BlockEventsQuerier,
    context: Chain,
}]
#[async_trait]
pub trait CanQueryBlockEvents:
    HasHeightType + HasAddressType + HasEventType + HasAsyncErrorType
{
    async fn query_block_events(
        &self,
        height: &Self::Height,
        address: &Self::Address,
    ) -> Result<Vec<Self::Event>, Self::Error>;
}
