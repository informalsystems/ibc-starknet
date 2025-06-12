use hermes_prelude::*;
use ureq::Agent;
use url::Url;

#[cgp_getter {
    name: ReqwestClientGetterComponent,
    provider: ReqwestClientGetter,
}]
pub trait HasReqwestClient {
    fn reqwest_client(&self) -> &Agent;
}

#[cgp_getter {
    provider: JsonRpcUrlGetter,
}]
pub trait HasJsonRpcUrl {
    fn json_rpc_url(&self) -> &Url;
}
