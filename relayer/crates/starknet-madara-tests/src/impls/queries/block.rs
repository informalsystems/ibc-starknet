use hermes_core::chain_components::traits::{
    BlockQuerier, BlockQuerierComponent, HasBlockType, HasHeightType,
};
use hermes_cosmos_core::chain_components::types::Time;
use hermes_prelude::*;
use hermes_starknet_chain_components::traits::{CanSendJsonRpcRequest, HasStarknetClient};
use hermes_starknet_chain_components::types::StarknetChainStatus;
use starknet::core::types::Felt;
use starknet::providers::{Provider, ProviderError};

use crate::impls::queries::utils::{QueryBlockWithTxHashesRequest, QueryBlockWithTxHashesResponse};

#[cgp_new_provider(BlockQuerierComponent)]
impl<Chain> BlockQuerier<Chain> for QueryStarknetBlock
where
    Chain: HasBlockType<Block = StarknetChainStatus>
        + HasHeightType<Height = u64>
        + HasStarknetClient<Client: Provider>
        + CanSendJsonRpcRequest<QueryBlockWithTxHashesRequest, QueryBlockWithTxHashesResponse>
        + CanRaiseAsyncError<ProviderError>
        + CanRaiseAsyncError<&'static str>,
{
    async fn query_block(chain: &Chain, height: &u64) -> Result<StarknetChainStatus, Chain::Error> {
        let request = QueryBlockWithTxHashesRequest { block_id: "latest" };
        let block = chain
            .send_json_rpc_request("starknet_getBlockWithTxHashes", &request)
            .await?;

        Ok(StarknetChainStatus {
            height: block.block_number,
            block_hash: Felt::from_hex_unchecked(block.block_hash.as_str()),
            time: i64::try_from(block.timestamp)
                .ok()
                .and_then(|ts| Time::from_unix_timestamp(ts, 0).ok())
                .ok_or_else(|| Chain::raise_error("invalid timestamp"))?,
        })
    }
}
