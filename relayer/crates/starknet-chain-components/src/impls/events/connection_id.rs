use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::extract_data::MessageResponseExtractor;
use hermes_chain_components::traits::types::ibc::HasConnectionIdType;
use hermes_chain_components::traits::types::ibc_events::connection::{
    ProvideConnectionOpenInitEvent, ProvideConnectionOpenTryEvent,
};
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::Felt;

use crate::impls::events::UseStarknetEvents;
use crate::impls::types::events::{
    StarknetConnectionOpenInitEvent, StarknetConnectionOpenTryEvent,
};
use crate::types::connection_id::ConnectionId;
use crate::types::message_response::StarknetMessageResponse;

impl<Chain, Counterparty> ProvideConnectionOpenInitEvent<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>,
{
    type ConnectionOpenInitEvent = ConnectionId;

    fn connection_open_init_event_connection_id(connection_id: &ConnectionId) -> &ConnectionId {
        connection_id
    }
}

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
