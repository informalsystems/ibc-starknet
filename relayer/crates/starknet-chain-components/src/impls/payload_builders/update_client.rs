use cgp::prelude::CanRaiseAsyncError;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc::core::client::types::Height;
use ibc::primitives::Timestamp;
use ibc_client_starknet_types::header::StarknetHeader;
use starknet::core::types::{BlockId, MaybePendingBlockWithTxHashes};
use starknet::providers::{Provider, ProviderError};

use crate::traits::provider::HasStarknetProvider;
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
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<ProviderError>,
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

        let block = match block_info {
            MaybePendingBlockWithTxHashes::Block(block) => block,
            MaybePendingBlockWithTxHashes::PendingBlock(_block) => {
                return Err(Chain::raise_error("pending block is not supported"))
            }
        };

        let block_hash = block.block_hash;

        let root = Vec::from(block_hash.to_bytes_be());

        let consensus_state = StarknetConsensusState {
            root: root.into(),
            time: Timestamp::from_nanoseconds(block.timestamp * 1_000_000_000),
        };

        let height = Height::new(0, *target_height).unwrap();

        let header = StarknetHeader {
            height,
            consensus_state,
        };

        Ok(StarknetUpdateClientPayload { header })
    }
}
