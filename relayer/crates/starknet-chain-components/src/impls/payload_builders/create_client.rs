use cgp::prelude::CanRaiseAsyncError;
use hermes_chain_components::traits::types::chain_id::HasChainId;
use hermes_cosmos_chain_components::types::key_types::secp256k1::Secp256k1KeyPair;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientPayloadOptionsType, HasCreateClientPayloadType,
};
use hermes_relayer_components::transaction::traits::default_signer::HasDefaultSigner;
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ChainId;
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
        + HasChainId<ChainId = ChainId>
        // TODO: StarknetChain doesn't have a Secp256k1KeyPair Signer
        + HasDefaultSigner<Signer = Secp256k1KeyPair>
        + CanRaiseAsyncError<ClientError>,
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &StarknetCreateClientPayloadOptions,
    ) -> Result<StarknetCreateClientPayload, Chain::Error> {
        let chain_status = chain.query_chain_status().await?;

        let root = Vec::from(chain_status.block_hash.to_bytes_be());

        let signer = chain.get_default_signer();

        let client_state = WasmStarknetClientState {
            wasm_code_hash: create_client_options.wasm_code_hash.into(),
            client_state: StarknetClientState {
                latest_height: Height::new(0, 1).map_err(Chain::raise_error)?,
                chain_id: chain.chain_id().clone(),
                pub_key: signer.public_key.clone(),
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
