use cgp::prelude::CanRaiseError;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientPayloadOptionsType, HasCreateClientPayloadType,
};
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;
use ibc::primitives::Timestamp;

use crate::types::client_state::{StarknetClientState, WasmStarknetClientState};
use crate::types::consensus_state::{StarknetConsensusState, WasmStarknetConsensusState};
use crate::types::payloads::client::{
    StarknetCreateClientPayload, StarknetCreateClientPayloadOptions,
};
use crate::types::status::StarknetChainStatus;

pub struct BuildStarknetCreateClientPayload;

impl<Chain, Counterparty> CreateClientPayloadBuilder<Chain, Counterparty>
    for BuildStarknetCreateClientPayload
where
    Chain: HasCreateClientPayloadOptionsType<
            Counterparty,
            CreateClientPayloadOptions = StarknetCreateClientPayloadOptions,
        > + HasCreateClientPayloadType<Counterparty, CreateClientPayload = StarknetCreateClientPayload>
        + CanQueryChainStatus<ChainStatus = StarknetChainStatus>
        + CanRaiseError<ClientError>,
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &StarknetCreateClientPayloadOptions,
    ) -> Result<StarknetCreateClientPayload, Chain::Error> {
        let chain_status = chain.query_chain_status().await?;

        let root = Vec::from(chain_status.block_hash.to_bytes_be());

        let client_state = WasmStarknetClientState {
            wasm_code_hash: create_client_options.wasm_code_hash.into(),
            client_state: StarknetClientState {
                latest_height: Height::new(0, 1).map_err(Chain::raise_error)?,
            },
        };

        let consensus_state = WasmStarknetConsensusState {
            consensus_state: StarknetConsensusState {
                root: root.into(),
                time: Timestamp::now(),
            },
        };

        Ok(StarknetCreateClientPayload {
            client_state,
            consensus_state,
        })
    }
}
