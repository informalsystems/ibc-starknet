use cgp::prelude::*;
use hermes_chain_components::traits::UpdateClientMessageBuilderComponent;
use hermes_core::chain_components::traits::{
    HasIbcChainTypes, HasMessageType, HasUpdateClientPayloadType, UpdateClientMessageBuilder,
};
use hermes_cosmos_chain_components::traits::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::CosmosUpdateClientMessage;
use hermes_encoding_components::traits::{CanConvert, CanEncode, HasDefaultEncoding};
use hermes_encoding_components::types::AsBytes;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use ibc::core::host::types::identifiers::ClientId;
use ibc_client_starknet_types::header::{SignedStarknetHeader, StarknetHeader};
use prost_types::Any;

use crate::types::payloads::client::StarknetUpdateClientPayload;

pub struct BuildStarknetUpdateClientMessage;

#[cgp_provider(UpdateClientMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildStarknetUpdateClientMessage
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + HasMessageType<Message = CosmosMessage>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasUpdateClientPayloadType<Chain, UpdateClientPayload = StarknetUpdateClientPayload>
        + HasDefaultEncoding<AsBytes, Encoding = Encoding>,
    Encoding: Async
        + CanEncode<ViaProtobuf, StarknetHeader, Encoded = Vec<u8>>
        + CanConvert<SignedStarknetHeader, Any>,
{
    async fn build_update_client_message(
        _chain: &Chain,
        client_id: &Chain::ClientId,
        payload: StarknetUpdateClientPayload,
    ) -> Result<Vec<CosmosMessage>, Chain::Error> {
        let encoding = Counterparty::default_encoding();

        let encoded_header = encoding
            .encode(&payload.header)
            .map_err(Chain::raise_error)?;

        let signed_header = SignedStarknetHeader {
            header: encoded_header,
            signature: payload.signature,
        };

        let signed_header_any: Any = encoding
            .convert(&signed_header)
            .map_err(Chain::raise_error)?;

        let update_client_message = CosmosUpdateClientMessage {
            client_id: client_id.clone(),
            header: signed_header_any,
        }
        .to_cosmos_message();

        Ok(vec![update_client_message])
    }
}
