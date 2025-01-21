use cgp::prelude::*;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::height::HasHeightType;
use starknet::core::types::{BlockId, ExecuteInvocation, TransactionTrace};
use starknet::providers::{Provider, ProviderError};

use crate::impls::send_message::extract_events_from_function_invocation;
use crate::traits::provider::HasStarknetProvider;
use crate::traits::queries::block_events::BlockEventsQuerier;
use crate::types::event::StarknetEvent;

pub struct QueryStarknetBlockEvents;

impl<Chain> BlockEventsQuerier<Chain> for QueryStarknetBlockEvents
where
    Chain: HasHeightType<Height = u64>
        + HasEventType<Event = StarknetEvent>
        + HasStarknetProvider
        + CanRaiseAsyncError<ProviderError>,
{
    async fn query_block_events(
        chain: &Chain,
        height: &u64,
    ) -> Result<Vec<Chain::Event>, Chain::Error> {
        let provider = chain.provider();

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
