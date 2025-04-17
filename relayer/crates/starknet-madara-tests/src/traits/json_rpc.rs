use cgp::prelude::*;
use serde::{Deserialize, Serialize};

use crate::traits::{HasJsonRpcUrl, HasRpcClient};

#[async_trait]
pub trait CanSendJsonRpcRequest<Request, Response>: HasAsyncErrorType {
    async fn send_json_rpc_request(
        &self,
        method: &str,
        request: &Request,
    ) -> Result<Response, Self::Error>;
}

impl<Context, Request, Response> CanSendJsonRpcRequest<Request, Response> for Context
where
    Context: HasRpcClient
        + HasJsonRpcUrl
        + CanRaiseAsyncError<reqwest::Error>
        + CanRaiseAsyncError<serde_json::Error>,
    Request: Async + Serialize,
    Response: Async + for<'a> Deserialize<'a>,
{
    async fn send_json_rpc_request(
        &self,
        method: &str,
        params: &Request,
    ) -> Result<Response, Self::Error> {
        let request_body = JsonRpcRequest {
            id: 1,
            jsonrpc: "2.0",
            method,
            params,
        };

        let request_string = serde_json::to_string(&request_body).map_err(Self::raise_error)?;

        let request = self
            .rpc_client()
            .post(self.json_rpc_url().clone())
            .body(request_string)
            .header("Content-Type", "application/json");

        let response = request.send().await.map_err(Self::raise_error)?;

        let response_string = response.text().await.map_err(Self::raise_error)?;

        let rpc_response: JsonRpcResponse<Response> =
            serde_json::from_str(&response_string).map_err(Self::raise_error)?;

        Ok(rpc_response.result)
    }
}

#[derive(Debug, Serialize)]
pub struct JsonRpcRequest<'a, T> {
    pub id: u64,
    pub jsonrpc: &'a str,
    pub method: &'a str,
    pub params: &'a T,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub result: T,
}
