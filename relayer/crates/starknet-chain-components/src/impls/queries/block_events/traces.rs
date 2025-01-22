use cgp::prelude::*;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use starknet::core::types::{BlockId, ExecuteInvocation, Felt, TransactionTrace};
use starknet::providers::{Provider, ProviderError};

use crate::impls::send_message::extract_events_from_function_invocation;
use crate::traits::provider::HasStarknetProvider;
use crate::traits::queries::block_events::BlockEventsQuerier;
use crate::types::event::StarknetEvent;

pub struct QueryStarknetBlockEventsFromTraces;

impl<Chain> BlockEventsQuerier<Chain> for QueryStarknetBlockEventsFromTraces
where
    Chain: HasHeightType<Height = u64>
        + HasEventType<Event = StarknetEvent>
        + HasAddressType<Address = Felt>
        + HasStarknetProvider
        + CanRaiseAsyncError<ProviderError>,
{
    async fn query_block_events(
        chain: &Chain,
        height: &u64,
        address: &Felt,
    ) -> Result<Vec<StarknetEvent>, Chain::Error> {
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
                        if &invoke.contract_address == address {
                            Some(extract_events_from_function_invocation(invoke))
                        } else {
                            None
                        }
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
