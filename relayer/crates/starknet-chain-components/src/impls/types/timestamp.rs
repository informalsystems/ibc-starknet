use cgp::core::Async;
use hermes_chain_type_components::traits::types::time::ProvideTimeType;

pub struct ProvideStarknetTimeType;

impl<Chain: Async> ProvideTimeType<Chain> for ProvideStarknetTimeType {
    // Dummy implementation for now
    type Time = ();
}
