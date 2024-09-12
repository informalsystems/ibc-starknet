use core::time::Duration;

use cgp::core::error::CanRaiseError;
use hermes_relayer_components::transaction::impls::poll_tx_response::PollTimeoutGetter;
use hermes_relayer_components::transaction::traits::query_tx_response::TxResponseQuerier;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use hermes_relayer_components::transaction::traits::types::tx_response::HasTxResponseType;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;
use starknet::core::types::{Felt, StarknetError};
use starknet::providers::{Provider, ProviderError};

use crate::traits::provider::HasStarknetProvider;
use crate::types::tx_response::TxResponse;

pub struct QueryTransactionReceipt;

impl<Chain> TxResponseQuerier<Chain> for QueryTransactionReceipt
where
    Chain: HasTransactionHashType<TxHash = Felt>
        + HasTxResponseType<TxResponse = TxResponse>
        + HasStarknetProvider
        + HasRuntime<Runtime: CanSleep>
        + CanRaiseError<ProviderError>,
{
    async fn query_tx_response(
        chain: &Chain,
        tx_hash: &Felt,
    ) -> Result<Option<Chain::TxResponse>, Chain::Error> {
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

pub struct DefaultPollTimeout;

impl<Chain> PollTimeoutGetter<Chain> for DefaultPollTimeout {
    fn poll_timeout(_chain: &Chain) -> Duration {
        Duration::from_secs(300)
    }

    fn poll_backoff(_chain: &Chain) -> Duration {
        Duration::from_millis(100)
    }
}
