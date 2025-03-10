use cgp::prelude::*;
use hermes_chain_components::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
use hermes_chain_components::traits::queries::block::CanQueryBlock;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_components::traits::types::chain_id::HasChainId;
use hermes_cosmos_chain_components::types::key_types::secp256k1::Secp256k1KeyPair;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientPayloadOptionsType, HasCreateClientPayloadType,
};
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ChainId;
use ibc::primitives::Timestamp;

use crate::traits::proof_signer::HasStarknetProofSigner;
use crate::types::consensus_state::{StarknetConsensusState, WasmStarknetConsensusState};
use crate::types::payloads::client::{
    StarknetCreateClientPayload, StarknetCreateClientPayloadOptions,
};
use crate::types::status::StarknetChainStatus;

pub struct BuildStarknetCreateClientPayload;

#[cgp_provider(CreateClientPayloadBuilderComponent)]
impl<Chain, Counterparty> CreateClientPayloadBuilder<Chain, Counterparty>
    for BuildStarknetCreateClientPayload
where
    Chain: HasCreateClientPayloadOptionsType<
            Counterparty,
            CreateClientPayloadOptions = StarknetCreateClientPayloadOptions,
        > + HasCreateClientPayloadType<Counterparty, CreateClientPayload = StarknetCreateClientPayload>
        + CanQueryBlock<Block = StarknetChainStatus>
        + CanQueryChainHeight<Height = u64>
        + HasChainId<ChainId = ChainId>
        + HasStarknetProofSigner<ProofSigner = Secp256k1KeyPair>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<ClientError>,
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &StarknetCreateClientPayloadOptions,
    ) -> Result<StarknetCreateClientPayload, Chain::Error> {
        let height = chain.query_chain_height().await?;

        let block = chain.query_block(&height).await?;

        let root = Vec::from(block.block_hash.to_bytes_be());

        let consensus_state = WasmStarknetConsensusState {
            consensus_state: StarknetConsensusState {
                root: root.into(),
                time: u64::try_from(block.time.unix_timestamp_nanos())
                    .ok()
                    .map(Timestamp::from_nanoseconds)
                    .ok_or_else(|| Chain::raise_error("invalid timestamp"))?,
            },
        };

        Ok(StarknetCreateClientPayload {
            latest_height: Height::new(0, block.height).map_err(Chain::raise_error)?,
            chain_id: chain.chain_id().clone(),
            client_state_wasm_code_hash: create_client_options.wasm_code_hash.into(),
            consensus_state,
            proof_signer_pub_key: chain.proof_signer().public_key.serialize().to_vec(),
        })
    }
}
