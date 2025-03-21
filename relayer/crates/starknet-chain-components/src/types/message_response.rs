use cgp::prelude::*;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_type_components::traits::fields::message_response_events::{
    MessageResponseEventsGetter, MessageResponseEventsGetterComponent,
};
use hermes_chain_type_components::traits::types::message_response::{
    HasMessageResponseType, MessageResponseTypeComponent, ProvideMessageResponseType,
};
use starknet::core::types::Felt;

use crate::types::event::StarknetEvent;

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
