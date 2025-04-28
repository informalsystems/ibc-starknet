use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::{
    BlockEventsQuerier, BlockEventsQuerierComponent, HasEventType, HasHeightType,
};
use hermes_chain_type_components::traits::HasAddressType;
use starknet::core::types::{BlockId, EventFilter};
use starknet::providers::{Provider, ProviderError};

use crate::impls::types::address::StarknetAddress;
use crate::traits::client::HasStarknetClient;
use crate::traits::queries::contract_address::CanQueryContractAddress;
use crate::types::event::StarknetEvent;

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
            .map(StarknetEvent::from)
            .collect();

        Ok(events)
    }
}
