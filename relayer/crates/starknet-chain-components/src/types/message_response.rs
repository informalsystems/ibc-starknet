use hermes_core::chain_components::traits::HasEventType;
use hermes_core::chain_type_components::traits::{
    HasMessageResponseType, MessageResponseEventsGetter, MessageResponseEventsGetterComponent,
    MessageResponseTypeComponent, ProvideMessageResponseType,
};
use hermes_prelude::*;
use starknet::core::types::Felt;

use crate::types::StarknetEvent;

#[derive(Debug)]
pub struct StarknetMessageResponse {
    pub result: Vec<Felt>,
    pub events: Vec<StarknetEvent>,
}

pub struct UseStarknetMessageResponse;

#[cgp_provider(MessageResponseTypeComponent)]
impl<Chain: Async> ProvideMessageResponseType<Chain> for UseStarknetMessageResponse {
    type MessageResponse = StarknetMessageResponse;
}

#[cgp_provider(MessageResponseEventsGetterComponent)]
impl<Chain> MessageResponseEventsGetter<Chain> for UseStarknetMessageResponse
where
    Chain: HasEventType<Event = StarknetEvent>
        + HasMessageResponseType<MessageResponse = StarknetMessageResponse>,
{
    fn message_response_events(message_response: &StarknetMessageResponse) -> &[StarknetEvent] {
        &message_response.events
    }
}
