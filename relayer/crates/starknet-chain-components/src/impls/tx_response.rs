use core::time::Duration;

use cgp_core::error::CanRaiseError;
use hermes_relayer_components::transaction::impls::poll_tx_response::PollTimeoutGetter;
use hermes_relayer_components::transaction::traits::query_tx_response::TxResponseQuerier;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use hermes_relayer_components::transaction::traits::types::tx_response::HasTxResponseType;
use starknet::core::types::{Felt, StarknetError, TransactionReceiptWithBlockInfo};
use starknet::providers::{Provider, ProviderError};

use crate::traits::provider::HasStarknetProvider;

pub struct QueryTransactionReceipt;

impl<Chain> TxResponseQuerier<Chain> for QueryTransactionReceipt
where
    Chain: HasTransactionHashType<TxHash = Felt>
        + HasTxResponseType<TxResponse = TransactionReceiptWithBlockInfo>
        + HasStarknetProvider
        + CanRaiseError<ProviderError>,
{
    async fn query_tx_response(
        chain: &Chain,
        tx_hash: &Felt,
    ) -> Result<Option<Chain::TxResponse>, Chain::Error> {
        let result = chain.provider().get_transaction_receipt(tx_hash).await;

        match result {
            Ok(receipt) => Ok(Some(receipt)),
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
