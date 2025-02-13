use cgp::prelude::*;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use starknet::core::types::{BlockId, EventFilter};
use starknet::providers::{Provider, ProviderError};

use crate::impls::types::address::StarknetAddress;
use crate::traits::provider::HasStarknetProvider;
use crate::traits::queries::block_events::BlockEventsQuerier;
use crate::types::event::StarknetEvent;

pub struct GetStarknetBlockEvents;

impl<Chain> BlockEventsQuerier<Chain> for GetStarknetBlockEvents
where
    Chain: HasHeightType<Height = u64>
        + HasEventType<Event = StarknetEvent>
        + HasAddressType<Address = StarknetAddress>
        + HasStarknetProvider
        + CanRaiseAsyncError<ProviderError>,
{
    async fn query_block_events(
        chain: &Chain,
        height: &u64,
        address: &StarknetAddress,
    ) -> Result<Vec<StarknetEvent>, Chain::Error> {
        let provider = chain.provider();

        let raw_events = provider
            .get_events(
                EventFilter {
                    from_block: Some(BlockId::Number(*height)),
                    to_block: Some(BlockId::Number(*height)),
                    address: Some(**address),
                    keys: None,
                },
                None,
                1000,
            )
            .await
            .map_err(Chain::raise_error)?;

        let events = raw_events
            .events
            .into_iter()
            .map(StarknetEvent::from)
            .collect();

        Ok(events)
    }
}
