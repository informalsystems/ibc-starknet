use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::extract_data::{
    MessageResponseExtractor, MessageResponseExtractorComponent,
};
use hermes_chain_components::traits::types::create_client::{
    CreateClientEventComponent, ProvideCreateClientEvent,
};
use hermes_chain_components::traits::types::ibc::HasClientIdType;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::Felt;

use crate::impls::events::UseStarknetEvents;
use crate::impls::types::events::StarknetCreateClientEvent;
use crate::types::client_id::ClientId;
use crate::types::message_response::StarknetMessageResponse;
use crate::types::message_responses::create_client::CreateClientResponse;

#[cgp_provider(CreateClientEventComponent)]
impl<Chain, Counterparty, Encoding> ProvideCreateClientEvent<Chain, Counterparty>
    for UseStarknetEvents
where
    Chain: HasMessageResponseType<MessageResponse = StarknetMessageResponse>
        + HasClientIdType<Counterparty, ClientId = ClientId>
        + HasDefaultEncoding<AsFelt, Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>> + CanDecode<ViaCairo, CreateClientResponse>,
{
    type CreateClientEvent = StarknetCreateClientEvent;

    fn create_client_event_client_id(event: &StarknetCreateClientEvent) -> &ClientId {
        &event.client_id
    }
}

#[cgp_provider(MessageResponseExtractorComponent)]
impl<Chain, Encoding> MessageResponseExtractor<Chain, StarknetCreateClientEvent>
    for UseStarknetEvents
where
    Chain: HasMessageResponseType<MessageResponse = StarknetMessageResponse>
        + HasDefaultEncoding<AsFelt, Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>> + CanDecode<ViaCairo, CreateClientResponse>,
{
    fn try_extract_from_message_response(
        _chain: &Chain,
        _tag: PhantomData<StarknetCreateClientEvent>,
        message_response: &StarknetMessageResponse,
    ) -> Option<StarknetCreateClientEvent> {
        let create_client_response = Chain::default_encoding()
            .decode(&message_response.result)
            .ok()?;

        Some(StarknetCreateClientEvent {
            client_id: create_client_response.client_id,
        })
    }
}
