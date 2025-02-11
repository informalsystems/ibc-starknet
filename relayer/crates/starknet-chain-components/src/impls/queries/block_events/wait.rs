use core::marker::PhantomData;
use core::time::Duration;

use cgp::prelude::*;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;

use crate::traits::queries::block_events::{BlockEventsQuerier, BlockEventsQuerierComponent};

pub struct WaitBlockHeightAndQueryEvents<InQuerier>(pub PhantomData<InQuerier>);

#[cgp_provider(BlockEventsQuerierComponent)]
impl<Chain, InQuerier> BlockEventsQuerier<Chain> for WaitBlockHeightAndQueryEvents<InQuerier>
where
    Chain: HasRuntime + HasAddressType + HasEventType + CanQueryChainHeight,
    InQuerier: BlockEventsQuerier<Chain>,
    Chain::Runtime: CanSleep,
{
    async fn query_block_events(
        chain: &Chain,
        height: &Chain::Height,
        address: &Chain::Address,
    ) -> Result<Vec<Chain::Event>, Chain::Error> {
        let runtime = chain.runtime();

        loop {
            let current_height = chain.query_chain_height().await?;
            if &current_height >= height {
                break;
            } else {
                runtime.sleep(Duration::from_millis(200)).await;
            }
        }

        InQuerier::query_block_events(chain, height, address).await
    }
}
