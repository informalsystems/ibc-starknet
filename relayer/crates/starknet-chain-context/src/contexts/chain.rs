use cgp_core::prelude::HasComponents;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;
use url::Url;

pub struct StarknetChain {
    pub rpc_client: JsonRpcClient<HttpTransport>,
}

pub struct StarknetChainContextComponents;

impl StarknetChain {
    pub fn new(json_rpc_url: Url) -> Self {
        let rpc_client = JsonRpcClient::new(HttpTransport::new(json_rpc_url));

        Self { rpc_client }
    }
}

impl HasComponents for StarknetChain {
    type Components = StarknetChainContextComponents;
}
