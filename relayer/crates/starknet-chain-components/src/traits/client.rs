use cgp::prelude::*;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;

#[cgp_component {
  name: JsonRpcClientGetterComponent,
  provider: JsonRpcClientGetter,
  context: Chain,
}]
pub trait HasJsonRpcClient: Async {
    fn json_rpc_client(&self) -> &JsonRpcClient<HttpTransport>;
}
