use core::time::Duration;

use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelDebug;
use hermes_core::relayer_components::transaction::traits::{
    HasTxHashType, HasTxResponseType, TxResponseQuerier, TxResponseQuerierComponent,
};
use hermes_core::runtime_components::traits::{CanSleep, HasRuntime};
use hermes_prelude::*;
use starknet::core::types::{Felt, StarknetError};
use starknet::providers::{Provider, ProviderError};

use crate::traits::HasStarknetClient;
use crate::types::TxResponse;

pub struct QueryTransactionReceipt;

#[cgp_provider(TxResponseQuerierComponent)]
impl<Chain> TxResponseQuerier<Chain> for QueryTransactionReceipt
where
    Chain: HasTxHashType<TxHash = Felt>
        + HasTxResponseType<TxResponse = TxResponse>
        + HasStarknetClient<Client: Provider>
        + HasRuntime<Runtime: CanSleep>
        + CanLog<LevelDebug>
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

                let tx_response = TxResponse { receipt, trace };

                chain
                    .log(
                        &format!(
                            "[{}] {:?}",
                            tx_hash.to_fixed_hex_string(),
                            tx_response.execution_resources(),
                        ),
                        &LevelDebug,
                    )
                    .await;

                Ok(Some(tx_response))
            }
            Err(ProviderError::StarknetError(StarknetError::TransactionHashNotFound)) => Ok(None),
            Err(e) => Err(Chain::raise_error(e)),
        }
    }
}
