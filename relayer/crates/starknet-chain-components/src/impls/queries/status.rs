use cgp::prelude::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerier;
use hermes_relayer_components::chain::traits::types::status::HasChainStatusType;
use starknet::core::types::{BlockId, BlockTag, MaybePendingBlockWithTxHashes};
use starknet::providers::{Provider, ProviderError};

use crate::traits::provider::HasStarknetProvider;
use crate::types::status::StarknetChainStatus;

pub struct QueryStarknetChainStatus;

impl<Chain> ChainStatusQuerier<Chain> for QueryStarknetChainStatus
where
    Chain: HasChainStatusType<ChainStatus = StarknetChainStatus>
        + HasStarknetProvider
        + CanRaiseError<ProviderError>
        + CanRaiseError<&'static str>,
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
            }),
            MaybePendingBlockWithTxHashes::PendingBlock(_) => Err(Chain::raise_error(
                "expected finalized block, but given pending block",
            )),
        }
    }
}
