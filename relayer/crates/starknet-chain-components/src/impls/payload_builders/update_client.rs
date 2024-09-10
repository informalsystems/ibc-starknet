use cgp::prelude::CanRaiseError;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc::primitives::Timestamp;
use starknet::core::types::{BlockId, MaybePendingBlockWithTxHashes};
use starknet::providers::{Provider, ProviderError};

use crate::traits::provider::HasStarknetProvider;
use crate::types::client_header::StarknetClientHeader;
use crate::types::consensus_state::StarknetConsensusState;
use crate::types::payloads::client::StarknetUpdateClientPayload;
use crate::types::status::StarknetChainStatus;

pub struct BuildStarknetUpdateClientPayload;

impl<Chain, Counterparty> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildStarknetUpdateClientPayload
where
    Chain: HasHeightType<Height = u64>
        + HasClientStateType<Counterparty>
        + HasUpdateClientPayloadType<Counterparty, UpdateClientPayload = StarknetUpdateClientPayload>
        + CanQueryChainStatus<ChainStatus = StarknetChainStatus>
        + HasStarknetProvider
        + CanRaiseError<&'static str>
        + CanRaiseError<ProviderError>,
{
    async fn build_update_client_payload(
        chain: &Chain,
        _trusted_height: &u64,
        target_height: &u64,
        _client_state: Chain::ClientState,
    ) -> Result<Chain::UpdateClientPayload, Chain::Error> {
        let block_info = chain
            .provider()
            .get_block_with_tx_hashes(BlockId::Number(*target_height))
            .await
            .map_err(Chain::raise_error)?;

        let block_hash = match block_info {
            MaybePendingBlockWithTxHashes::Block(block) => block.block_hash,
            MaybePendingBlockWithTxHashes::PendingBlock(_block) => {
                return Err(Chain::raise_error("pending block is not supported"))
            }
        };

        let root = Vec::from(block_hash.to_bytes_be());

        let consensus_state = StarknetConsensusState {
            root: root.into(),
            time: Timestamp::now(),
        };

        let header = StarknetClientHeader { consensus_state };

        Ok(StarknetUpdateClientPayload { header })
    }
}
