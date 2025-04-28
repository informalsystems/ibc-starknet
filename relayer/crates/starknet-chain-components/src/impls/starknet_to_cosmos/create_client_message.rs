use cgp::prelude::*;
use hermes_chain_components::traits::CreateClientMessageBuilderComponent;
use hermes_core::chain_components::traits::{
    CreateClientMessageBuilder, HasClientStateType, HasConsensusStateType,
    HasCreateClientMessageOptionsType, HasCreateClientPayloadType, HasMessageType,
};
use hermes_cosmos_chain_components::traits::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::CosmosCreateClientMessage;
use hermes_encoding_components::traits::{CanConvert, HasDefaultEncoding};
use hermes_encoding_components::types::AsBytes;
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
