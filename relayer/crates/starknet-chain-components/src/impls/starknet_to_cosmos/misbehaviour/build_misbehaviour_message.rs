use hermes_core::chain_components::traits::{
    HasClientIdType, HasEvidenceType, HasMessageType, MisbehaviourMessageBuilder,
    MisbehaviourMessageBuilderComponent,
};
use hermes_core::encoding_components::traits::{CanConvert, HasDefaultEncoding};
use hermes_core::encoding_components::types::AsBytes;
use hermes_cosmos_core::chain_components::impls::SubmitMisbehaviour;
use hermes_cosmos_core::chain_components::traits::{CosmosMessage, ToCosmosMessage};
use hermes_prelude::*;
use ibc::clients::wasm_types::client_message::ClientMessage;
use ibc::core::host::types::identifiers::ClientId;
use prost::Message;
use prost_types::Any;

#[cgp_new_provider(MisbehaviourMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> MisbehaviourMessageBuilder<Chain, Counterparty>
    for CosmosFromStarknetMisbehaviourMessageBuilder
where
    Chain: HasEvidenceType<Evidence = Any>
        + HasClientIdType<Counterparty, ClientId = ClientId>
        + HasMessageType<Message = CosmosMessage>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasDefaultEncoding<AsBytes, Encoding = Encoding>,
    Encoding: CanConvert<ClientMessage, Any> + HasAsyncErrorType,
{
    async fn build_misbehaviour_message(
        chain: &Chain,
        client_id: &Chain::ClientId,
        evidence: &Chain::Evidence,
    ) -> Result<Chain::Message, Chain::Error> {
        let encoding = Counterparty::default_encoding();

        let msg = SubmitMisbehaviour {
            client_id: client_id.clone(),
            evidence: evidence.clone(),
        };

        let wasm_message = ClientMessage {
            data: evidence.encode_to_vec(),
        };

        // Convert Wasm ClientMessage to Any
        let any_wasm_message = encoding
            .convert(&wasm_message)
            .map_err(Chain::raise_error)?;

        let message = SubmitMisbehaviour {
            client_id: client_id.clone(),
            evidence: any_wasm_message,
        };

        Ok(message.to_cosmos_message())
    }
}
