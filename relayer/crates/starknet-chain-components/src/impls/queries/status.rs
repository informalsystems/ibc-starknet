use cgp::prelude::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerier;
use hermes_relayer_components::chain::traits::types::status::HasChainStatusType;
use starknet::providers::{Provider, ProviderError};

use crate::traits::provider::HasStarknetProvider;
use crate::types::status::StarknetChainStatus;

pub struct QueryStarknetChainStatus;

impl<Chain> ChainStatusQuerier<Chain> for QueryStarknetChainStatus
where
    Chain: HasChainStatusType<ChainStatus = StarknetChainStatus>
        + HasStarknetProvider
        + CanRaiseError<ProviderError>,
{
    async fn query_chain_status(chain: &Chain) -> Result<StarknetChainStatus, Chain::Error> {
        let status = chain
            .provider()
            .block_hash_and_number()
            .await
            .map_err(Chain::raise_error)?;

        Ok(StarknetChainStatus {
            height: status.block_number,
            block_hash: status.block_hash,
        })
    }
}
