use hermes_core::chain_components::traits::{
    HasIbcChainTypes, HasMessageType, HasUpdateClientPayloadType, UpdateClientMessageBuilder,
    UpdateClientMessageBuilderComponent,
};
use hermes_core::encoding_components::traits::{CanConvert, HasDefaultEncoding};
use hermes_core::encoding_components::types::AsBytes;
use hermes_cosmos_core::chain_components::traits::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_core::chain_components::types::CosmosUpdateClientMessage;
use hermes_prelude::*;
use ibc::core::host::types::identifiers::ClientId;
use ibc_client_starknet_types::header::StarknetHeader;
use prost_types::Any;

use crate::types::StarknetUpdateClientPayload;

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
    Encoding: Async + CanConvert<StarknetHeader, Any>,
{
    async fn build_update_client_message(
        _chain: &Chain,
        client_id: &Chain::ClientId,
        payload: StarknetUpdateClientPayload,
    ) -> Result<Vec<CosmosMessage>, Chain::Error> {
        let encoding = Counterparty::default_encoding();

        let signed_header_any: Any = encoding
            .convert(&payload.header)
            .map_err(Chain::raise_error)?;

        let update_client_message = CosmosUpdateClientMessage {
            client_id: client_id.clone(),
            header: signed_header_any,
        }
        .to_cosmos_message();

        Ok(vec![update_client_message])
    }
}
