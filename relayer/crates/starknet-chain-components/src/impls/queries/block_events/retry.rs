use core::marker::PhantomData;
use core::time::Duration;

use cgp::prelude::HasAsyncErrorType;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;

use crate::traits::queries::block_events::BlockEventsQuerier;

pub struct RetryQueryBlockEvents<InQuerier>(pub PhantomData<InQuerier>);

impl<Chain, InQuerier> BlockEventsQuerier<Chain> for RetryQueryBlockEvents<InQuerier>
where
    Chain: HasRuntime + HasHeightType + HasEventType + HasAsyncErrorType,
    InQuerier: BlockEventsQuerier<Chain>,
    Chain::Runtime: CanSleep,
{
    async fn query_block_events(
        chain: &Chain,
        height: &Chain::Height,
    ) -> Result<Vec<Chain::Event>, Chain::Error> {
        let runtime = chain.runtime();
        let mut sleep_time = Duration::from_millis(500);

        for _ in 0..5 {
            let res = InQuerier::query_block_events(chain, height).await;
            if let Ok(events) = res {
                return Ok(events);
            }

            runtime.sleep(sleep_time).await;
            sleep_time *= 2;
        }

        InQuerier::query_block_events(chain, height).await
    }
}
