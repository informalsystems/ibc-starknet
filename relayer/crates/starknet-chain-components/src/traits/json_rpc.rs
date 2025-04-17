use cgp::prelude::*;

#[cgp_component {
    provider: JsonRpcRequestSender,
}]
#[async_trait]
pub trait CanSendJsonRpcRequest<Request, Response>: HasAsyncErrorType
where
    Request: Async,
    Response: Async,
{
    async fn send_json_rpc_request(
        &self,
        method: &str,
        request: &Request,
    ) -> Result<Response, Self::Error>;
}
