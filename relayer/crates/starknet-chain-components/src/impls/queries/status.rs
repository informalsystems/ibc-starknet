use hermes_core::chain_components::traits::{
    ChainStatusQuerier, ChainStatusQuerierComponent, HasChainStatusType,
};
use hermes_cosmos_core::chain_components::types::Time;
use hermes_prelude::*;
use starknet::core::types::{BlockId, BlockTag, MaybePendingBlockWithTxHashes};
use starknet::providers::{Provider, ProviderError};

use crate::traits::HasStarknetClient;
use crate::types::StarknetChainStatus;

pub struct QueryStarknetChainStatus;

#[cgp_provider(ChainStatusQuerierComponent)]
impl<Chain> ChainStatusQuerier<Chain> for QueryStarknetChainStatus
where
    Chain: HasChainStatusType<ChainStatus = StarknetChainStatus>
        + HasStarknetClient<Client: Provider>
        + CanRaiseAsyncError<ProviderError>
        + CanRaiseAsyncError<&'static str>,
{
    async fn query_chain_status(chain: &Chain) -> Result<StarknetChainStatus, Chain::Error> {
        let block = chain
            .provider()
            .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
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
