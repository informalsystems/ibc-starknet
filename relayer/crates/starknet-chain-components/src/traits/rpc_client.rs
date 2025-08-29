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

#[cgp_getter {
    name: FeederGatewayUrlGetterComponent,
    provider: FeederGatewayUrlGetter,
}]
pub trait HasFeederGatewayUrl {
    fn feeder_gateway_url(&self) -> &Url;
}

#[cgp_getter {
    name: Ed25519AttestatorAddressesGetterComponent,
    provider: Ed25519AttestatorAddressesGetter,
}]
pub trait HasEd25519AttestatorAddresses {
    fn ed25519_attestator_addresses(&self) -> &Vec<String>;
}
