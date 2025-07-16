use hermes_prelude::*;
use url::Url;

#[cgp_getter {
    name: FeederGatewayUrlGetterComponent,
    provider: FeederGatewayUrlGetter,
}]
pub trait HasFeederGatewayUrl {
    fn feeder_gateway_url(&self) -> &Url;
}
