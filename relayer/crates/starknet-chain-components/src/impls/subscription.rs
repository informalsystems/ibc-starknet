use core::pin::Pin;
use std::sync::Arc;

use cgp::prelude::*;
use futures::channel::mpsc::{unbounded, UnboundedSender};
use futures::lock::Mutex;
use futures::Stream;
use hermes_async_runtime_components::subscription::impls::closure::CanCreateClosureSubscription;
use hermes_async_runtime_components::subscription::impls::multiplex::CanMultiplexSubscription;
use hermes_async_runtime_components::subscription::traits::subscription::Subscription;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::spawn::CanSpawnTask;
use hermes_runtime_components::traits::task::Task;

use crate::traits::queries::block_events::CanQueryBlockEvents;

pub trait CanCreateStarknetEventSubscription:
    HasHeightType + HasAddressType + HasEventType + HasAsyncErrorType
{
    fn create_starknet_event_subscription(
        self,
        start_height: Self::Height,
        address: Self::Address,
    ) -> Arc<dyn Subscription<Item = (Self::Height, Self::Event)>>;
}

#[async_trait]
pub trait CanSendStarknetEvents:
    HasHeightType + HasAddressType + HasEventType + HasAsyncErrorType
{
    async fn send_starknet_events(
        &self,
        address: &Self::Address,
        start_height: Arc<Mutex<Self::Height>>,
        sender: UnboundedSender<(Self::Height, Self::Event)>,
    ) -> Result<(), Self::Error>;
}

impl<Chain> CanSendStarknetEvents for Chain
where
    Chain: HasHeightType<Height = u64>
        + HasAddressType
        + CanQueryBlockEvents
        + CanRaiseError<&'static str>,
{
    async fn send_starknet_events(
        &self,
        address: &Self::Address,
        height_mutex: Arc<Mutex<u64>>,
        sender: UnboundedSender<(u64, Self::Event)>,
    ) -> Result<(), Self::Error> {
        loop {
            let mut height_ref = height_mutex.lock().await;
            let height = *height_ref;

            let events = self.query_block_events(&height, address).await?;
            for event in events {
                sender
                    .unbounded_send((height, event))
                    .map_err(|_| Chain::raise_error("channel closed"))?;
            }

            *height_ref = height + 1;
        }
    }
}

pub struct PollStarknetEventsTask<Chain>
where
    Chain: HasHeightType + HasAddressType + HasEventType,
{
    pub chain: Chain,
    pub address: Chain::Address,
    pub height: Arc<Mutex<Chain::Height>>,
    pub sender: UnboundedSender<(Chain::Height, Chain::Event)>,
}

impl<Chain> Task for PollStarknetEventsTask<Chain>
where
    Chain: CanSendStarknetEvents,
{
    async fn run(self) {
        let _ = self
            .chain
            .send_starknet_events(&self.address, self.height, self.sender)
            .await;
    }
}

impl<Chain> CanCreateStarknetEventSubscription for Chain
where
    Chain: Clone + HasRuntime + CanSendStarknetEvents,
    Chain::Runtime: Clone + CanCreateClosureSubscription + CanMultiplexSubscription + CanSpawnTask,
    Chain::Address: Clone,
    Chain::Event: Clone,
{
    fn create_starknet_event_subscription(
        self,
        height: Chain::Height,
        address: Chain::Address,
    ) -> Arc<dyn Subscription<Item = (Chain::Height, Chain::Event)>> {
        let runtime = self.runtime().clone();
        let height_mutex = Arc::new(Mutex::new(height));

        let subscription = Chain::Runtime::new_closure_subscription(move || {
            let chain = self.clone();
            let address = address.clone();
            let height_mutex = height_mutex.clone();

            Box::pin(async move {
                let (sender, receiver) = unbounded();

                let task = PollStarknetEventsTask {
                    chain: chain.clone(),
                    sender,
                    address,
                    height: height_mutex.clone(),
                };

                chain.runtime().spawn_task(task);

                let stream: Pin<
                    Box<dyn Stream<Item = (Chain::Height, Chain::Event)> + Send + Sync + 'static>,
                > = Box::pin(receiver);

                Some(stream)
            })
        });

        runtime.multiplex_subscription(subscription, |e| e)
    }
}
