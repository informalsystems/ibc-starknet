use core::pin::Pin;
use std::sync::Arc;

use cgp::prelude::*;
use futures::channel::mpsc::{unbounded, UnboundedSender};
use futures::Stream;
use hermes_async_runtime_components::channel::types::ChannelClosedError;
use hermes_async_runtime_components::subscription::impls::closure::CanCreateClosureSubscription;
use hermes_async_runtime_components::subscription::traits::subscription::Subscription;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::spawn::CanSpawnTask;
use hermes_runtime_components::traits::task::Task;

use crate::traits::queries::block_events::CanQueryBlockEvents;

#[async_trait]
pub trait CanCreateStarknetSubscription: HasHeightType + HasEventType + HasAsyncErrorType {
    async fn create_event_subscription(
        self,
        start_height: Self::Height,
    ) -> Result<Arc<dyn Subscription<Item = (Self::Height, Arc<Self::Event>)>>, Self::Error>;
}

#[async_trait]
pub trait CanSendStarknetEvents: HasHeightType + HasEventType + HasAsyncErrorType {
    async fn send_starknet_events(
        &self,
        start_height: Self::Height,
        sender: UnboundedSender<(Self::Height, Arc<Self::Event>)>,
    ) -> Result<(), Self::Error>;
}

impl<Chain> CanSendStarknetEvents for Chain
where
    Chain: HasHeightType<Height = u64> + CanQueryBlockEvents + CanRaiseError<ChannelClosedError>,
{
    async fn send_starknet_events(
        &self,
        mut height: u64,
        sender: UnboundedSender<(u64, Arc<Self::Event>)>,
    ) -> Result<(), Self::Error> {
        loop {
            let events = self.query_block_events(&height).await?;
            for event in events {
                sender
                    .unbounded_send((height, Arc::new(event)))
                    .map_err(|_| Chain::raise_error(ChannelClosedError))?;
            }
            height += 1;
        }
    }
}

pub struct PollStarknetEventsTask<Chain>
where
    Chain: HasHeightType + HasEventType,
{
    pub chain: Chain,
    pub height: Chain::Height,
    pub sender: UnboundedSender<(Chain::Height, Arc<Chain::Event>)>,
}

impl<Chain> Task for PollStarknetEventsTask<Chain>
where
    Chain: CanSendStarknetEvents,
{
    async fn run(self) -> () {
        let _ = self
            .chain
            .send_starknet_events(self.height, self.sender)
            .await;
    }
}

impl<Chain> CanCreateStarknetSubscription for Chain
where
    Chain: Clone + HasRuntime + CanSendStarknetEvents,
    Chain::Runtime: CanCreateClosureSubscription + CanSpawnTask,
{
    async fn create_event_subscription(
        self,
        height: Chain::Height,
    ) -> Result<Arc<dyn Subscription<Item = (Chain::Height, Arc<Chain::Event>)>>, Chain::Error>
    {
        Ok(Chain::Runtime::new_closure_subscription(move || {
            let chain = self.clone();
            let height = height.clone();

            Box::pin(async move {
                let (sender, receiver) = unbounded();

                let task = PollStarknetEventsTask {
                    chain: chain.clone(),
                    sender,
                    height: height.clone(),
                };

                chain.runtime().spawn_task(task);

                let stream: Pin<
                    Box<
                        dyn Stream<Item = (Chain::Height, Arc<Chain::Event>)>
                            + Send
                            + Sync
                            + 'static,
                    >,
                > = Box::pin(receiver);

                Some(stream)
            })
        }))
    }
}
