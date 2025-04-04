use cgp::prelude::*;
use reqwest::Client;
use url::Url;

#[cgp_getter {
    provider: RpcClientGetter,
}]
pub trait HasRpcClient {
    fn rpc_client(&self) -> &Client;
}

#[cgp_getter {
    provider: JsonRpcUrlGetter,
}]
pub trait HasJsonRpcUrl {
    fn json_rpc_url(&self) -> &Url;
}
