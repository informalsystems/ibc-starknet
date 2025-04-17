use core::time::Duration;

use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::query_tx_response::{
    TxResponseQuerier, TxResponseQuerierComponent,
};
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTxHashType;
use hermes_relayer_components::transaction::traits::types::tx_response::HasTxResponseType;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_starknet_chain_components::traits::client::HasStarknetClient;
use serde::{Deserialize, Serialize};
use starknet_v13::core::types::requests::TraceTransactionRequest;
use starknet_v13::core::types::{Felt, StarknetError, TransactionTrace};
use starknet_v13::providers::{Provider, ProviderError};

use crate::traits::CanSendJsonRpcRequest;
use crate::types::TxResponse;

#[cgp_new_provider(TxResponseQuerierComponent)]
impl<Chain> TxResponseQuerier<Chain> for QueryTransactionReceipt
where
    Chain: HasTxHashType<TxHash = Felt>
        + HasTxResponseType<TxResponse = TxResponse>
        + HasStarknetClient<Client: Provider>
        + CanTraceTransaction
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
                let trace = chain.trace_transaction(*tx_hash).await?;

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

// Madara returns a non-compliant JSON RPC response for the starknet_traceTransaction call.
// Hence we need a workaround to parse the response manually here.
// We may need further abstraction over the underlying JSON RPC client if there are other
// non-compliant APIs out there.
#[async_trait]
pub trait CanTraceTransaction: HasAsyncErrorType {
    async fn trace_transaction(&self, tx_hash: Felt) -> Result<TransactionTrace, Self::Error>;
}

impl<Chain> CanTraceTransaction for Chain
where
    Chain: for<'a> CanSendJsonRpcRequest<TraceTransactionRequest, TraceTransactionResponse>,
{
    async fn trace_transaction(
        &self,
        transaction_hash: Felt,
    ) -> Result<TransactionTrace, Self::Error> {
        let params = TraceTransactionRequest { transaction_hash };

        let rpc_response = self
            .send_json_rpc_request("starknet_traceTransaction", &params)
            .await?;

        let trace = rpc_response.trace_root;

        Ok(trace)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraceTransactionResponse {
    pub trace_root: TransactionTrace,
}
