use hermes_core::chain_type_components::traits::{ProvideTimeType, TimeTypeComponent};
use hermes_prelude::*;

pub struct ProvideStarknetTimeType;

#[cgp_provider(TimeTypeComponent)]
impl<Chain: Async> ProvideTimeType<Chain> for ProvideStarknetTimeType {
    // Dummy implementation for now
    type Time = ();
}
