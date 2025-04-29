use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    ConnectionOpenInitEventComponent, ConnectionOpenTryEventComponent, HasConnectionIdType,
    MessageResponseExtractor, MessageResponseExtractorComponent, ProvideConnectionOpenInitEvent,
    ProvideConnectionOpenTryEvent,
};
use hermes_core::chain_type_components::traits::HasMessageResponseType;
use hermes_core::encoding_components::traits::{CanDecode, HasDefaultEncoding, HasEncodedType};
use hermes_prelude::*;
use starknet::core::types::Felt;

use crate::impls::{
    StarknetConnectionOpenInitEvent, StarknetConnectionOpenTryEvent, UseStarknetEvents,
};
use crate::types::{ConnectionId, StarknetMessageResponse};

#[cgp_provider(ConnectionOpenInitEventComponent)]
impl<Chain, Counterparty> ProvideConnectionOpenInitEvent<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>,
{
    type ConnectionOpenInitEvent = StarknetConnectionOpenInitEvent;

    fn connection_open_init_event_connection_id(
        event: &StarknetConnectionOpenInitEvent,
    ) -> &ConnectionId {
        &event.connection_id
    }
}

#[cgp_provider(MessageResponseExtractorComponent)]
impl<Chain, Encoding> MessageResponseExtractor<Chain, StarknetConnectionOpenInitEvent>
    for UseStarknetEvents
where
    Chain: HasMessageResponseType<MessageResponse = StarknetMessageResponse>
        + HasDefaultEncoding<AsFelt, Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>> + CanDecode<ViaCairo, ConnectionId>,
{
    fn try_extract_from_message_response(
        _chain: &Chain,
        _tag: PhantomData<StarknetConnectionOpenInitEvent>,
        message_response: &Chain::MessageResponse,
    ) -> Option<StarknetConnectionOpenInitEvent> {
        let connection_id = Chain::default_encoding()
            .decode(&message_response.result)
            .ok()?;

        Some(StarknetConnectionOpenInitEvent { connection_id })
    }
}

#[cgp_provider(ConnectionOpenTryEventComponent)]
impl<Chain, Counterparty> ProvideConnectionOpenTryEvent<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>,
{
    type ConnectionOpenTryEvent = StarknetConnectionOpenTryEvent;

    fn connection_open_try_event_connection_id(
        event: &StarknetConnectionOpenTryEvent,
    ) -> &ConnectionId {
        &event.connection_id
    }
}

#[cgp_provider(MessageResponseExtractorComponent)]
impl<Chain, Encoding> MessageResponseExtractor<Chain, StarknetConnectionOpenTryEvent>
    for UseStarknetEvents
where
    Chain: HasMessageResponseType<MessageResponse = StarknetMessageResponse>
        + HasDefaultEncoding<AsFelt, Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>> + CanDecode<ViaCairo, ConnectionId>,
{
    fn try_extract_from_message_response(
        _chain: &Chain,
        _tag: PhantomData<StarknetConnectionOpenTryEvent>,
        message_response: &Chain::MessageResponse,
    ) -> Option<StarknetConnectionOpenTryEvent> {
        let connection_id = Chain::default_encoding()
            .decode(&message_response.result)
            .ok()?;

        Some(StarknetConnectionOpenTryEvent { connection_id })
    }
}
