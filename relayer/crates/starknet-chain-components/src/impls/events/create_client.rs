use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CreateClientEventComponent, HasClientIdType, MessageResponseExtractor,
    MessageResponseExtractorComponent, ProvideCreateClientEvent,
};
use hermes_core::chain_type_components::traits::HasMessageResponseType;
use hermes_core::encoding_components::traits::{CanDecode, HasDefaultEncoding, HasEncodedType};
use hermes_prelude::*;
use starknet::core::types::Felt;

use crate::impls::{StarknetCreateClientEvent, UseStarknetEvents};
use crate::types::{ClientId, CreateClientResponse, StarknetMessageResponse};

#[cgp_provider(CreateClientEventComponent)]
impl<Chain, Counterparty> ProvideCreateClientEvent<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasMessageResponseType<MessageResponse = StarknetMessageResponse>
        + HasClientIdType<Counterparty, ClientId = ClientId>,
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
