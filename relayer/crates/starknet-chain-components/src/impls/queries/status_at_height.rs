use cgp::prelude::*;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_cosmos_chain_components::types::status::Time;
use hermes_relayer_components::chain::traits::types::status::HasChainStatusType;
use starknet::core::types::{BlockId, MaybePendingBlockWithTxHashes};
use starknet::providers::{Provider, ProviderError};

use crate::traits::provider::HasStarknetProvider;
use crate::traits::queries::status_at_height::{
    ChainStatusAtHeightQuerier, ChainStatusAtHeightQuerierComponent,
};
use crate::types::status::StarknetChainStatus;

pub struct QueryStarknetChainStatusAtHeight;

#[cgp_provider(ChainStatusAtHeightQuerierComponent)]
impl<Chain> ChainStatusAtHeightQuerier<Chain> for QueryStarknetChainStatusAtHeight
where
    Chain: HasChainStatusType<ChainStatus = StarknetChainStatus>
        + HasHeightType<Height = u64>
        + HasStarknetProvider
        + CanRaiseAsyncError<ProviderError>
        + CanRaiseAsyncError<&'static str>,
{
    async fn query_chain_status_at_height(
        chain: &Chain,
        height: &u64,
    ) -> Result<StarknetChainStatus, Chain::Error> {
        let block = chain
            .provider()
            .get_block_with_tx_hashes(BlockId::Number(*height))
            .await
            .map_err(Chain::raise_error)?;

        match block {
            MaybePendingBlockWithTxHashes::Block(block) => Ok(StarknetChainStatus {
                height: block.block_number,
                block_hash: block.block_hash,
                time: i64::try_from(block.timestamp)
                    .ok()
                    .and_then(|ts| Time::from_unix_timestamp(ts, 0).ok())
                    .ok_or_else(|| Chain::raise_error("invalid timestamp"))?,
            }),
            MaybePendingBlockWithTxHashes::PendingBlock(_) => Err(Chain::raise_error(
                "expected finalized block, but given pending block",
            )),
        }
    }
}
