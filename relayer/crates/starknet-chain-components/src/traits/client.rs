use cgp_core::prelude::*;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;

#[derive_component(JsonRpcClientGetterComponent, JsonRpcClientGetter<Chain>)]
pub trait HasJsonRpcClient: Async {
    fn json_rpc_client(&self) -> JsonRpcClient<HttpTransport>;
}
