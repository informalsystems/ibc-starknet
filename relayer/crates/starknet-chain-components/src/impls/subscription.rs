use std::sync::Arc;

use cgp::prelude::*;
use hermes_async_runtime_components::subscription::traits::subscription::Subscription;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::height::HasHeightType;
use starknet::core::types::{BlockId, ExecuteInvocation, TransactionTrace};
use starknet::providers::{Provider, ProviderError};

use crate::impls::send_message::extract_events_from_function_invocation;
use crate::traits::provider::HasStarknetProvider;
use crate::traits::queries::address::CanQueryContractAddress;
use crate::types::event::StarknetEvent;

pub trait CanCreateStarknetSubscription: HasHeightType + HasEventType + HasAsyncErrorType {
    fn create_event_subscription(
        &self,
    ) -> Result<Arc<dyn Subscription<Item = (Self::Height, Arc<Self::Event>)>>, Self::Error>;
}

impl<Chain> CanCreateStarknetSubscription for Chain
where
    Chain: HasHeightType
        + HasEventType
        + HasAsyncErrorType
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>,
{
    fn create_event_subscription(
        &self,
    ) -> Result<Arc<dyn Subscription<Item = (Self::Height, Arc<Self::Event>)>>, Self::Error> {
        todo!()
    }
}

#[async_trait]
pub trait CanPollStarknetIbcEvents: HasHeightType + HasEventType + HasAsyncErrorType {
    async fn poll_starknet_ibc_events(
        &self,
        height: &Self::Height,
    ) -> Result<Vec<Self::Event>, Self::Error>;
}

impl<Chain> CanPollStarknetIbcEvents for Chain
where
    Chain: HasHeightType<Height = u64>
        + HasEventType<Event = StarknetEvent>
        + HasStarknetProvider
        + CanRaiseAsyncError<ProviderError>,
{
    async fn poll_starknet_ibc_events(
        &self,
        height: &u64,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        let provider = self.provider();

        let traces = provider
            .trace_block_transactions(BlockId::Number(*height))
            .await
            .map_err(Chain::raise_error)?;

        let events: Vec<StarknetEvent> = traces
            .into_iter()
            .filter_map(|trace| match trace.trace_root {
                TransactionTrace::Invoke(invoke) => match invoke.execute_invocation {
                    ExecuteInvocation::Success(invoke) => {
                        Some(extract_events_from_function_invocation(invoke))
                    }
                    _ => None,
                },
                _ => None,
            })
            .flatten()
            .collect();

        Ok(events)
    }
}
