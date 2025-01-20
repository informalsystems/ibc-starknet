use core::marker::PhantomData;
use core::pin::Pin;
use std::sync::Arc;

use cgp::prelude::*;
use futures::{stream, Stream, StreamExt, TryStreamExt};
use hermes_async_runtime_components::subscription::impls::closure::CanCreateClosureSubscription;
use hermes_async_runtime_components::subscription::traits::subscription::Subscription;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::traits::queries::address::CanQueryContractAddress;
use crate::traits::queries::block_events::CanQueryBlockEvents;
use crate::types::event::StarknetEvent;

#[async_trait]
pub trait CanCreateStarknetSubscription: HasHeightType + HasEventType + HasAsyncErrorType {
    async fn create_event_subscription(
        self,
        start_height: Self::Height,
    ) -> Result<Arc<dyn Subscription<Item = (Self::Height, Arc<Self::Event>)>>, Self::Error>;
}

impl<Chain> CanCreateStarknetSubscription for Chain
where
    Chain: Clone
        + Send
        + Sync
        + 'static
        + HasRuntime
        + HasHeightType<Height = u64>
        + HasEventType<Event = StarknetEvent>
        + CanQueryBlockEvents
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>,
    Chain::Runtime: CanCreateClosureSubscription,
{
    async fn create_event_subscription(
        self,
        mut next_height: u64,
    ) -> Result<Arc<dyn Subscription<Item = (u64, Arc<StarknetEvent>)>>, Self::Error> {
        let ibc_core_contract_address = self.query_contract_address(PhantomData).await?;

        Ok(Chain::Runtime::new_closure_subscription(move || {
            let chain = self.clone();
            Box::pin(async move {
                let height_stream = stream::repeat_with(move || {
                    let height = next_height.clone();
                    next_height += 1;
                    height
                });

                let event_stream = height_stream
                    .filter_map(|height| {
                        let chain = chain.clone();
                        async move {
                            // let chain = chain.clone();
                            // let events = chain.query_block_events(&height).await.ok()?;
                            let events_with_height =
                                [].into_iter().map(move |event| (height, Arc::new(event)));

                            Some(stream::iter(events_with_height))
                        }
                    })
                    .flatten();

                let boxed_stream: Pin<
                    Box<dyn Stream<Item = (u64, Arc<StarknetEvent>)> + Send + Sync>,
                > = Box::pin(event_stream);

                todo!()
                // Some(Box::pin(event_stream))
            })
        }))
    }
}
