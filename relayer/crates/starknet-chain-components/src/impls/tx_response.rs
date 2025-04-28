use core::time::Duration;

use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::{
    HasTxHashType, HasTxResponseType, TxResponseQuerier, TxResponseQuerierComponent,
};
use hermes_runtime_components::traits::{CanSleep, HasRuntime};
use starknet::core::types::{Felt, StarknetError};
use starknet::providers::{Provider, ProviderError};

use crate::traits::client::HasStarknetClient;
use crate::types::tx_response::TxResponse;

pub struct QueryTransactionReceipt;

#[cgp_provider(TxResponseQuerierComponent)]
impl<Chain> TxResponseQuerier<Chain> for QueryTransactionReceipt
where
    Chain: HasTxHashType<TxHash = Felt>
        + HasTxResponseType<TxResponse = TxResponse>
        + HasStarknetClient<Client: Provider>
        + HasRuntime<Runtime: CanSleep>
        + CanRaiseAsyncError<ProviderError>,
{
    async fn query_tx_response(
        chain: &Chain,
        tx_hash: &Felt,
    ) -> Result<Option<TxResponse>, Chain::Error> {
        let provider = chain.provider();

        let result = provider.get_transaction_receipt(tx_hash).await;

        match result {
            Ok(receipt) => {
                let trace = provider
                    .trace_transaction(tx_hash)
                    .await
                    .map_err(Chain::raise_error)?;

                // Wait for a second for the starknet-devnet chain to progress.
                // We may not need this when we transition to a production chain.
                chain.runtime().sleep(Duration::from_secs(1)).await;

                Ok(Some(TxResponse { receipt, trace }))
            }
            Err(ProviderError::StarknetError(StarknetError::TransactionHashNotFound)) => Ok(None),
            Err(e) => Err(Chain::raise_error(e)),
        }
    }
}
