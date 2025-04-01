use std::sync::Arc;

use cgp::prelude::*;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;

#[cgp_getter {
    provider: JsonRpcClientGetter,
    context: Chain,
}]
pub trait HasJsonRpcClient: Async {
    fn json_rpc_client(&self) -> &Arc<JsonRpcClient<HttpTransport>>;
}
