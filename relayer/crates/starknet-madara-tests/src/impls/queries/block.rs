use cgp::prelude::*;
use hermes_core::chain_components::traits::{
    BlockQuerier, BlockQuerierComponent, HasBlockType, HasHeightType,
};
use hermes_cosmos_chain_components::types::Time;
use hermes_starknet_chain_components::traits::client::HasStarknetClient;
use hermes_starknet_chain_components::types::status::StarknetChainStatus;
use starknet_v13::core::types::{BlockId, MaybePendingBlockWithTxHashes};
use starknet_v13::providers::{Provider, ProviderError};

#[cgp_new_provider(BlockQuerierComponent)]
impl<Chain> BlockQuerier<Chain> for QueryStarknetBlock
where
    Chain: HasBlockType<Block = StarknetChainStatus>
        + HasHeightType<Height = u64>
        + HasStarknetClient<Client: Provider>
        + CanRaiseAsyncError<ProviderError>
        + CanRaiseAsyncError<&'static str>,
{
    async fn query_block(chain: &Chain, height: &u64) -> Result<StarknetChainStatus, Chain::Error> {
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
