use cgp::prelude::*;
use hermes_chain_components::traits::types::ibc::CounterpartyMessageHeightGetterComponent;
use hermes_cosmos_chain_components::traits::message::CosmosMessage;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::CounterpartyMessageHeightGetter;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;

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
