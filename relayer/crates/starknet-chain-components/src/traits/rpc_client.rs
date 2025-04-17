use cgp::prelude::*;
use reqwest::Client;
use url::Url;

#[cgp_getter {
    name: ReqwestClientGetterComponent,
    provider: ReqwestClientGetter,
}]
pub trait HasReqwestClient {
    fn reqwest_client(&self) -> &Client;
}

#[cgp_getter {
    provider: JsonRpcUrlGetter,
}]
pub trait HasJsonRpcUrl {
    fn json_rpc_url(&self) -> &Url;
}
