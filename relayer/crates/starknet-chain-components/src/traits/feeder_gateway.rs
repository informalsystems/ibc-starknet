use hermes_prelude::*;
use starknet_block_verifier::Endpoint;

#[cgp_getter {
    name: FeederGatewayEndpointGetterComponent,
    provider: FeederGatewayEndpointGetter,
}]
pub trait HasFeederGatewayEndpoint {
    fn feeder_gateway_endpoint(&self) -> &Endpoint;
}
