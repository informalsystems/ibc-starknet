use hermes_core::chain_components::traits::{
    CanQueryBlock, CanQueryChainHeight, CreateClientPayloadBuilder,
    CreateClientPayloadBuilderComponent, HasChainId, HasCreateClientPayloadOptionsType,
    HasCreateClientPayloadType,
};
use hermes_cosmos_core::chain_components::types::Secp256k1KeyPair;
use hermes_prelude::*;
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
