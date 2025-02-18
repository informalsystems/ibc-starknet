use cgp::prelude::*;
use hermes_chain_type_components::traits::types::time::ProvideTimeType;
use hermes_cosmos_chain_components::components::client::TimeTypeComponent;

pub struct ProvideStarknetTimeType;

#[cgp_provider(TimeTypeComponent)]
impl<Chain: Async> ProvideTimeType<Chain> for ProvideStarknetTimeType {
    // Dummy implementation for now
    type Time = ();
}
