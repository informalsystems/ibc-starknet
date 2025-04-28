use cgp::prelude::*;
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelTrace;
use serde::{Deserialize, Serialize};

use crate::traits::json_rpc::{JsonRpcRequestSender, JsonRpcRequestSenderComponent};
use crate::traits::rpc_client::{HasJsonRpcUrl, HasReqwestClient};

#[cgp_new_provider(JsonRpcRequestSenderComponent)]
impl<Context, Request, Response> JsonRpcRequestSender<Context, Request, Response>
    for SendJsonRpcRequestWithReqwest
where
    Context: HasReqwestClient
        + HasJsonRpcUrl
        + CanLog<LevelTrace>
        + CanRaiseAsyncError<reqwest::Error>
        + CanRaiseAsyncError<serde_json::Error>,
    Request: Async + Serialize,
    Response: Async + for<'a> Deserialize<'a>,
{
    async fn send_json_rpc_request(
        context: &Context,
        method: &str,
        params: &Request,
    ) -> Result<Response, Context::Error> {
        let request_body = JsonRpcRequest {
            id: 1,
            jsonrpc: "2.0",
            method,
            params,
        };

        let request_string = serde_json::to_string(&request_body).map_err(Context::raise_error)?;

        context
            .log(
                &format!("sending json rpc request: {request_string}"),
                &LevelTrace,
            )
            .await;

        let request = context
            .reqwest_client()
            .post(context.json_rpc_url().clone())
            .body(request_string)
            .header("Content-Type", "application/json");

        let response = request.send().await.map_err(Context::raise_error)?;

        let response_string = response.text().await.map_err(Context::raise_error)?;

        context
            .log(
                &format!("received json rpc response: {response_string}"),
                &LevelTrace,
            )
            .await;

        let rpc_response: JsonRpcResponse<Response> =
            serde_json::from_str(&response_string).map_err(Context::raise_error)?;

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
