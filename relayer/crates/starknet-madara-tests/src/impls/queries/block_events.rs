use core::marker::PhantomData;
use std::sync::Arc;

use cgp::prelude::*;
use hermes_chain_components::traits::queries::block_events::{
    BlockEventsQuerier, BlockEventsQuerierComponent,
};
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::traits::client::HasStarknetClient;
use hermes_starknet_chain_components::traits::queries::contract_address::CanQueryContractAddress;
use hermes_starknet_chain_components::types::event::{StarknetEvent, StarknetEventFields};
use starknet_v13::core::types::{BlockId, EmittedEvent, EventFilter};
use starknet_v13::providers::{Provider, ProviderError};

#[cgp_new_provider(BlockEventsQuerierComponent)]
impl<Chain> BlockEventsQuerier<Chain> for GetStarknetBlockEvents
where
    Chain: HasHeightType<Height = u64>
        + HasEventType<Event = StarknetEvent>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasAddressType<Address = StarknetAddress>
        + HasStarknetClient<Client: Provider>
        + CanRaiseAsyncError<ProviderError>,
{
    async fn query_block_events(
        chain: &Chain,
        height: &u64,
    ) -> Result<Vec<StarknetEvent>, Chain::Error> {
        let provider = chain.provider();
        let address = chain.query_contract_address(PhantomData).await?;

        let raw_events = provider
            .get_events(
                EventFilter {
                    from_block: Some(BlockId::Number(*height)),
                    to_block: Some(BlockId::Number(*height)),
                    address: Some(*address),
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
            .map(parse_emitted_events)
            .collect();

        Ok(events)
    }
}

fn parse_emitted_events(event: EmittedEvent) -> StarknetEvent {
    let mut keys = event.keys.into_iter();
    let selector = keys.next();

    StarknetEvent {
        fields: Arc::new(StarknetEventFields {
            contract_address: event.from_address.into(),
            class_hash: None,
            selector,
            keys: keys.collect(),
            data: event.data,
        }),
    }
}
