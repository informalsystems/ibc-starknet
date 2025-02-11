use core::marker::PhantomData;
use core::time::Duration;

use cgp::prelude::*;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;

use crate::traits::queries::block_events::{BlockEventsQuerier, BlockEventsQuerierComponent};

pub struct RetryQueryBlockEvents<InQuerier>(pub PhantomData<InQuerier>);

#[cgp_provider(BlockEventsQuerierComponent)]
impl<Chain, InQuerier> BlockEventsQuerier<Chain> for RetryQueryBlockEvents<InQuerier>
where
    Chain: HasRuntime + HasHeightType + HasAddressType + HasEventType + HasAsyncErrorType,
    InQuerier: BlockEventsQuerier<Chain>,
    Chain::Runtime: CanSleep,
{
    async fn query_block_events(
        chain: &Chain,
        height: &Chain::Height,
        address: &Chain::Address,
    ) -> Result<Vec<Chain::Event>, Chain::Error> {
        let runtime = chain.runtime();
        let mut sleep_time = Duration::from_millis(500);

        for _ in 0..5 {
            let res = InQuerier::query_block_events(chain, height, address).await;
            if let Ok(events) = res {
                return Ok(events);
            }

            runtime.sleep(sleep_time).await;
            sleep_time *= 2;
        }

        InQuerier::query_block_events(chain, height, address).await
    }
}
