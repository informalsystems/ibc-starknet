use cgp::core::Async;
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_test_components::bootstrap::traits::fields::dynamic_gas_fee::DynamicGasGetter;

pub struct NoDynamicGas;

impl<Bootstrap: Async> DynamicGasGetter<Bootstrap> for NoDynamicGas {
    fn dynamic_gas(_bootstrap: &Bootstrap) -> &Option<DynamicGasConfig> {
        &None
    }
}
