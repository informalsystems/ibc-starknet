use cgp::prelude::*;
use hermes_core::chain_components::traits::{
    CounterpartyMessageHeightGetter, CounterpartyMessageHeightGetterComponent, HasHeightType,
    HasMessageType,
};
use hermes_cosmos_core::chain_components::traits::CosmosMessage;

pub struct GetCosmosCounterpartyMessageStarknetHeight;

#[cgp_provider(CounterpartyMessageHeightGetterComponent)]
impl<Chain, Counterparty> CounterpartyMessageHeightGetter<Chain, Counterparty>
    for GetCosmosCounterpartyMessageStarknetHeight
where
    Chain: HasMessageType<Message = CosmosMessage>,
    Counterparty: HasHeightType<Height = u64>,
{
    fn counterparty_message_height_for_update_client(message: &CosmosMessage) -> Option<u64> {
        message
            .message
            .counterparty_message_height_for_update_client()
            .map(|h| h.revision_height())
    }
}
