use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::types::ibc::HasConnectionIdType;
use hermes_chain_components::traits::types::ibc_events::connection::{
    ProvideConnectionOpenInitEvent, ProvideConnectionOpenTryEvent,
};
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::Felt;

use crate::types::connection_id::ConnectionId;
use crate::types::message_response::StarknetMessageResponse;

pub struct UseStarknetConnectionHandshakeEvents;

impl<Chain, Counterparty, Encoding> ProvideConnectionOpenInitEvent<Chain, Counterparty>
    for UseStarknetConnectionHandshakeEvents
where
    Chain: HasMessageResponseType<MessageResponse = StarknetMessageResponse>
        + HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>
        + HasDefaultEncoding<AsFelt, Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>> + CanDecode<ViaCairo, ConnectionId>,
{
    type ConnectionOpenInitEvent = ConnectionId;

    fn try_extract_connection_open_init_event(
        response: &StarknetMessageResponse,
    ) -> Option<ConnectionId> {
        Chain::default_encoding().decode(&response.result).ok()
    }

    fn connection_open_init_event_connection_id(connection_id: &ConnectionId) -> &ConnectionId {
        connection_id
    }
}

impl<Chain, Counterparty, Encoding> ProvideConnectionOpenTryEvent<Chain, Counterparty>
    for UseStarknetConnectionHandshakeEvents
where
    Chain: HasMessageResponseType<MessageResponse = StarknetMessageResponse>
        + HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>
        + HasDefaultEncoding<AsFelt, Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>> + CanDecode<ViaCairo, ConnectionId>,
{
    type ConnectionOpenTryEvent = ConnectionId;

    fn try_extract_connection_open_try_event(
        _chain: &Chain,
        response: &StarknetMessageResponse,
    ) -> Option<ConnectionId> {
        Chain::default_encoding().decode(&response.result).ok()
    }

    fn connection_open_try_event_connection_id(connection_id: &ConnectionId) -> &ConnectionId {
        connection_id
    }
}
