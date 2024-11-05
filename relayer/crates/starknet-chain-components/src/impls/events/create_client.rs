use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::types::create_client::ProvideCreateClientEvent;
use hermes_chain_components::traits::types::ibc::HasClientIdType;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::Felt;

use crate::types::client_id::ClientId;
use crate::types::message_response::StarknetMessageResponse;
use crate::types::message_responses::create_client::CreateClientResponse;

pub struct UseStarknetCreateClientEvent;

impl<Chain, Counterparty, Encoding> ProvideCreateClientEvent<Chain, Counterparty>
    for UseStarknetCreateClientEvent
where
    Chain: HasMessageResponseType<MessageResponse = StarknetMessageResponse>
        + HasClientIdType<Counterparty, ClientId = ClientId>
        + HasDefaultEncoding<AsFelt, Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>> + CanDecode<ViaCairo, CreateClientResponse>,
{
    type CreateClientEvent = ClientId;

    fn try_extract_create_client_event(
        response: &StarknetMessageResponse,
    ) -> Option<Self::CreateClientEvent> {
        let create_client_response = Chain::default_encoding().decode(&response.result).ok()?;

        Some(create_client_response.client_id)
    }

    fn create_client_event_client_id(client_id: &ClientId) -> &ClientId {
        client_id
    }
}
