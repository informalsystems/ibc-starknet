use cgp::prelude::*;
use hermes_chain_components::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::messages::client::create::CosmosCreateClientMessage;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_encoding_components::types::AsBytes;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientMessageOptionsType, HasCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use ibc_client_starknet_types::StarknetClientState;
use prost_types::Any;

use crate::types::client_state::WasmStarknetClientState;
use crate::types::consensus_state::WasmStarknetConsensusState;
use crate::types::payloads::client::StarknetCreateClientPayload;

pub struct BuildStarknetCreateClientMessage;

#[cgp_provider(CreateClientMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildStarknetCreateClientMessage
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasCreateClientMessageOptionsType<Counterparty>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasCreateClientPayloadType<Chain, CreateClientPayload = StarknetCreateClientPayload>
        + HasClientStateType<Chain, ClientState = WasmStarknetClientState>
        + HasConsensusStateType<Chain, ConsensusState = WasmStarknetConsensusState>
        + HasDefaultEncoding<AsBytes, Encoding = Encoding>,
    Encoding: Async
        + CanConvert<Counterparty::ClientState, Any>
        + CanConvert<Counterparty::ConsensusState, Any>,
{
    async fn build_create_client_message(
        _chain: &Chain,
        _options: &Chain::CreateClientMessageOptions,
        payload: StarknetCreateClientPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let encoding = Counterparty::default_encoding();

        let starknet_client_state = StarknetClientState {
            latest_height: payload.latest_height,
            chain_id: payload.chain_id,
            pub_key: payload.proof_signer_pub_key,
        };

        let client_state = WasmStarknetClientState {
            client_state: starknet_client_state,
            wasm_code_hash: payload.client_state_wasm_code_hash,
        };

        let client_state = encoding
            .convert(&client_state)
            .map_err(Chain::raise_error)?;

        let consensus_state = encoding
            .convert(&payload.consensus_state)
            .map_err(Chain::raise_error)?;

        let message = CosmosCreateClientMessage {
            client_state,
            consensus_state,
        };

        Ok(message.to_cosmos_message())
    }
}
