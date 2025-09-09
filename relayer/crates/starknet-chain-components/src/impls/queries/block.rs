use hermes_core::chain_components::traits::{
    BlockQuerier, BlockQuerierComponent, HasBlockType, HasHeightType,
};
use hermes_cosmos_core::chain_components::types::Time;
use hermes_prelude::*;
use starknet::core::types::{BlockId, MaybePreConfirmedBlockWithTxHashes};
use starknet::providers::{Provider, ProviderError};

use crate::traits::HasStarknetClient;
use crate::types::StarknetChainStatus;

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
            MaybePreConfirmedBlockWithTxHashes::Block(block) => Ok(StarknetChainStatus {
                height: block.block_number,
                block_hash: block.block_hash,
                time: i64::try_from(block.timestamp)
                    .ok()
                    .and_then(|ts| Time::from_unix_timestamp(ts, 0).ok())
                    .ok_or_else(|| Chain::raise_error("invalid timestamp"))?,
            }),
            MaybePreConfirmedBlockWithTxHashes::PreConfirmedBlock(_) => Err(Chain::raise_error(
                "expected finalized block, but given pre-confirmed block",
            )),
        }
    }
}
