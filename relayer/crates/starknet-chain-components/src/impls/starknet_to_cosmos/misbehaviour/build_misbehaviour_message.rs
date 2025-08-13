use hermes_core::chain_components::traits::{
    HasChainId, HasClientIdType, HasEvidenceType, HasMessageType, MisbehaviourMessageBuilder,
    MisbehaviourMessageBuilderComponent,
};
use hermes_core::encoding_components::traits::{CanConvert, HasDefaultEncoding};
use hermes_core::encoding_components::types::AsBytes;
use hermes_core::relayer_components::transaction::traits::HasDefaultSigner;
use hermes_cosmos_core::chain_components::impls::SubmitMisbehaviour;
use hermes_cosmos_core::chain_components::traits::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_core::chain_components::types::Secp256k1KeyPair;
use hermes_prelude::*;
use ibc::clients::wasm_types::client_message::{ClientMessage, WASM_CLIENT_MESSAGE_TYPE_URL};
use ibc::core::host::types::identifiers::ClientId;
use ibc_client_starknet_types::misbehaviour::StarknetMisbehaviour;
use ibc_proto::ibc::lightclients::wasm::v1::ClientMessage as RawClientMessage;
use ibc_proto::Protobuf;
use prost::Message;
use prost_types::Any;

#[cgp_new_provider(MisbehaviourMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> MisbehaviourMessageBuilder<Chain, Counterparty>
    for CosmosFromStarknetMisbehaviourMessageBuilder
where
    Chain: HasEvidenceType<Evidence = Any>
        + HasChainId
        + HasClientIdType<Counterparty, ClientId = ClientId>
        + HasDefaultSigner<Signer = Secp256k1KeyPair>
        + HasMessageType<Message = CosmosMessage>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasDefaultEncoding<AsBytes, Encoding = Encoding>,
    Encoding: CanConvert<Any, StarknetMisbehaviour> + HasAsyncErrorType,
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

        let signer = chain.get_default_signer();

        /*let misbehaviour_message = MsgSubmitMisbehaviour {
            client_id: msg.client_id.to_string(),
            misbehaviour: Some(evidence.clone()),
            signer: signer.account().into(),
        };

        let any_misbehaviour = Any::from_msg(&misbehaviour_message)
            .expect("failed to convert `MsgSubmitMisbehaviour` to `Any`");*/

        let wasm_message = ClientMessage {
            data: evidence.encode_to_vec(),
        };

        // Convert Wasm ClientMessage to Any
        let any_wasm_message = Any {
            type_url: WASM_CLIENT_MESSAGE_TYPE_URL.to_owned(),
            value: Protobuf::<RawClientMessage>::encode_vec(wasm_message),
        };

        /*let message = CosmosUpdateClientMessage {
            client_id: client_id.clone(),
            header: any_wasm_message,
        };*/

        let message = SubmitMisbehaviour {
            client_id: client_id.clone(),
            evidence: any_wasm_message,
        };

        Ok(message.to_cosmos_message())

        //Ok(msg.to_cosmos_message())
    }
}
