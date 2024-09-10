use cgp::prelude::*;
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::messages::client::update::CosmosUpdateClientMessage;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_encoding_components::types::AsBytes;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use prost_types::Any;

use crate::types::client_header::StarknetClientHeader;
use crate::types::payloads::client::StarknetUpdateClientPayload;

pub struct BuildStarknetUpdateClientMessage;

impl<Chain, Counterparty, Encoding> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildStarknetUpdateClientMessage
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + HasMessageType<Message = CosmosMessage>
        + CanRaiseError<Encoding::Error>,
    Counterparty: HasUpdateClientPayloadType<Chain, UpdateClientPayload = StarknetUpdateClientPayload>
        + HasDefaultEncoding<AsBytes, Encoding = Encoding>,
    Encoding: Async + CanConvert<StarknetClientHeader, Any>,
{
    async fn build_update_client_message(
        _chain: &Chain,
        client_id: &Chain::ClientId,
        payload: StarknetUpdateClientPayload,
    ) -> Result<Vec<CosmosMessage>, Chain::Error> {
        let encoding = Counterparty::default_encoding();

        let header_any: Any = encoding
            .convert(&payload.header)
            .map_err(Chain::raise_error)?;

        let update_client_message = CosmosUpdateClientMessage {
            client_id: client_id.clone(),
            header: header_any,
        }
        .to_cosmos_message();

        Ok(vec![update_client_message])
    }
}
