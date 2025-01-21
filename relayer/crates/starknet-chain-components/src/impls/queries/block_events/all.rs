use cgp::prelude::CanRaiseAsyncError;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;
use starknet::providers::ProviderError;

use crate::impls::queries::block_events::query::QueryStarknetBlockEvents;
use crate::impls::queries::block_events::retry::RetryQueryBlockEvents;
use crate::impls::queries::block_events::wait::WaitBlockHeightAndQueryEvents;
use crate::traits::provider::HasStarknetProvider;
use crate::traits::queries::block_events::BlockEventsQuerier;
use crate::types::event::StarknetEvent;

pub struct QueryBlockEventsWithWaitAndRetry;

impl<Chain> BlockEventsQuerier<Chain> for QueryBlockEventsWithWaitAndRetry
where
    Chain: HasRuntime
        + HasHeightType<Height = u64>
        + HasEventType<Event = StarknetEvent>
        + HasStarknetProvider
        + CanQueryChainHeight
        + CanRaiseAsyncError<ProviderError>,
    Chain::Runtime: CanSleep,
{
    async fn query_block_events(
        chain: &Chain,
        height: &Chain::Height,
    ) -> Result<Vec<Chain::Event>, Chain::Error> {
        <RetryQueryBlockEvents<WaitBlockHeightAndQueryEvents<QueryStarknetBlockEvents>>>::query_block_events(chain, height).await
    }
}
