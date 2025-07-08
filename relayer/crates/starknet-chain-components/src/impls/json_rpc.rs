use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelTrace;
use hermes_prelude::*;
use serde::{Deserialize, Serialize};

use crate::traits::{
    HasJsonRpcUrl, HasReqwestClient, JsonRpcRequestSender, JsonRpcRequestSenderComponent,
};

#[cgp_new_provider(JsonRpcRequestSenderComponent)]
impl<Context, Request, Response> JsonRpcRequestSender<Context, Request, Response>
    for SendJsonRpcRequestWithReqwest
where
    Context: HasReqwestClient
        + HasJsonRpcUrl
        + CanLog<LevelTrace>
        + CanRaiseAsyncError<ureq::Error>
        + CanRaiseAsyncError<serde_json::Error>
        + CanRaiseAsyncError<String>,
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
            .post(context.json_rpc_url().as_str())
            .header("Content-Type", "application/json");

        let mut response = request.send(request_string).map_err(Context::raise_error)?;

        let response_string = response
            .body_mut()
            .read_to_string()
            .map_err(Context::raise_error)?;

        context
            .log(
                &format!("received json rpc response: {response_string}"),
                &LevelTrace,
            )
            .await;

        let rpc_response: JsonRpcResponse<Response> =
            serde_json::from_str(&response_string).map_err(Context::raise_error)?;

        match rpc_response.data {
            ResponseData::Error(err) => {
                context
                    .log(&format!("json rpc error: {err:?}"), &LevelTrace)
                    .await;
                Err(Context::raise_error(format!(
                    "json rpc error: code: {}, message: {}, data: {:?}",
                    err.code, err.message, err.data
                )))
            }
            ResponseData::Result(result) => Ok(result),
        }
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
    #[serde(flatten)]
    pub data: ResponseData<T>,
}

#[derive(Debug, Deserialize)]
pub enum ResponseData<T> {
    #[serde(rename = "result")]
    Result(T),
    #[serde(rename = "error")]
    Error(JsonRpcError),
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
