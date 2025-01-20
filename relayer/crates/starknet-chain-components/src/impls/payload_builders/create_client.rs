use cgp::prelude::CanRaiseAsyncError;
use hermes_chain_components::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientPayloadOptionsType, HasCreateClientPayloadType,
};
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ChainId;
use ibc::primitives::Timestamp;

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
        + HasChainId<ChainId = ChainId>
        + CanRaiseAsyncError<ClientError>,
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &StarknetCreateClientPayloadOptions,
    ) -> Result<StarknetCreateClientPayload, Chain::Error> {
        let chain_status = chain.query_chain_status().await?;

        let root = Vec::from(chain_status.block_hash.to_bytes_be());

        let consensus_state = WasmStarknetConsensusState {
            consensus_state: StarknetConsensusState {
                root: root.into(),
                time: Timestamp::now(),
            },
        };

        Ok(StarknetCreateClientPayload {
            latest_height: Height::new(0, 1).map_err(Chain::raise_error)?,
            chain_id: chain.chain_id().clone(),
            client_state_wasm_code_hash: create_client_options.wasm_code_hash.into(),
            consensus_state,
        })
    }
}
