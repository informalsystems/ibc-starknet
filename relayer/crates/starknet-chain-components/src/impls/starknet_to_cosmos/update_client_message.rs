use cgp::prelude::*;
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::key_types::secp256k1::Secp256k1KeyPair;
use hermes_cosmos_chain_components::types::messages::client::update::CosmosUpdateClientMessage;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_encoding_components::types::AsBytes;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_relayer_components::transaction::traits::default_signer::HasDefaultSigner;
use ibc::core::host::types::identifiers::ClientId;
use ibc_client_starknet_types::header::{SignedStarknetHeader, StarknetHeader};
use prost_types::Any;

use crate::types::payloads::client::StarknetUpdateClientPayload;

pub struct BuildStarknetUpdateClientMessage;

impl<Chain, Counterparty, Encoding> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildStarknetUpdateClientMessage
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + HasMessageType<Message = CosmosMessage>
        + HasDefaultSigner<Signer = Secp256k1KeyPair>
        + CanRaiseAsyncError<Encoding::Error>
        + CanRaiseAsyncError<String>,
    Counterparty: HasUpdateClientPayloadType<Chain, UpdateClientPayload = StarknetUpdateClientPayload>
        + HasDefaultEncoding<AsBytes, Encoding = Encoding>,
    Encoding: Async + CanConvert<StarknetHeader, Any> + CanConvert<SignedStarknetHeader, Any>,
{
    async fn build_update_client_message(
        chain: &Chain,
        client_id: &Chain::ClientId,
        payload: StarknetUpdateClientPayload,
    ) -> Result<Vec<CosmosMessage>, Chain::Error> {
        let encoding = Counterparty::default_encoding();

        let header_bytes: Any = encoding
            .convert(&payload.header)
            .map_err(Chain::raise_error)?;

        let signer = chain.get_default_signer();

        let signature = signer
            .sign(&header_bytes.value)
            .map_err(Chain::raise_error)?;

        let signed_header = SignedStarknetHeader {
            header: payload.header.clone(),
            signature,
        };

        let header_any: Any = encoding
            .convert(&signed_header)
            .map_err(Chain::raise_error)?;

        let update_client_message = CosmosUpdateClientMessage {
            client_id: client_id.clone(),
            header: header_any,
        }
        .to_cosmos_message();

        Ok(vec![update_client_message])
    }
}
