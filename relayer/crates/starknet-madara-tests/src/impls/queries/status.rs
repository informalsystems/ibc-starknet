use cgp::prelude::*;
use hermes_chain_components::traits::queries::chain_status::ChainStatusQuerierComponent;
use hermes_cosmos_chain_components::types::status::Time;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerier;
use hermes_relayer_components::chain::traits::types::status::HasChainStatusType;
use hermes_starknet_chain_components::traits::client::HasStarknetClient;
use hermes_starknet_chain_components::types::status::StarknetChainStatus;
use starknet_v13::core::types::{BlockId, BlockTag, MaybePendingBlockWithTxHashes};
use starknet_v13::providers::{Provider, ProviderError};

#[cgp_new_provider(ChainStatusQuerierComponent)]
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
