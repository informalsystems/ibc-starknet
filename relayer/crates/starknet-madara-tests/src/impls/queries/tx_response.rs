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
use starknet_v13::core::types::requests::TraceTransactionRequestRef;
use starknet_v13::core::types::{Felt, StarknetError, TransactionTrace};
use starknet_v13::providers::jsonrpc::JsonRpcMethod;
use starknet_v13::providers::{Provider, ProviderError};

use crate::traits::{HasJsonRpcUrl, HasRpcClient};
use crate::types::TxResponse;

#[cgp_new_provider(TxResponseQuerierComponent)]
impl<Chain> TxResponseQuerier<Chain> for QueryTransactionReceipt
where
    Chain: HasTxHashType<TxHash = Felt>
        + HasTxResponseType<TxResponse = TxResponse>
        + HasStarknetClient<Client: Provider>
        + HasRpcClient
        + HasJsonRpcUrl
        + HasRuntime<Runtime: CanSleep>
        + CanRaiseAsyncError<ProviderError>
        + CanRaiseAsyncError<serde_json::Error>
        + CanRaiseAsyncError<reqwest::Error>,
{
    async fn query_tx_response(
        chain: &Chain,
        tx_hash: &Felt,
    ) -> Result<Option<TxResponse>, Chain::Error> {
        let provider = chain.provider();

        let result = provider.get_transaction_receipt(tx_hash).await;

        match result {
            Ok(receipt) => {
                let params = TraceTransactionRequestRef {
                    transaction_hash: tx_hash.as_ref(),
                };

                let request_body = JsonRpcRequest {
                    id: 1,
                    jsonrpc: "2.0",
                    method: JsonRpcMethod::TraceTransaction,
                    params,
                };

                let request_body =
                    serde_json::to_string(&request_body).map_err(Chain::raise_error)?;

                let request = chain
                    .rpc_client()
                    .post(chain.json_rpc_url().clone())
                    .body(request_body)
                    .header("Content-Type", "application/json");

                let response = request.send().await.map_err(Chain::raise_error)?;

                let response_body = response.text().await.map_err(Chain::raise_error)?;

                let rpc_response: JsonRpcResponse<TraceTransactionResponse> =
                    serde_json::from_str(&response_body).map_err(Chain::raise_error)?;

                let trace = rpc_response.result.trace_root;

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

#[derive(Debug, Serialize)]
pub struct JsonRpcRequest<T> {
    id: u64,
    jsonrpc: &'static str,
    method: JsonRpcMethod,
    params: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub result: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraceTransactionResponse {
    pub trace_root: TransactionTrace,
}
