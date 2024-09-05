use cgp::core::Async;
use hermes_test_components::chain::traits::types::address::ProvideAddressType;
use starknet::core::types::Felt;

pub struct ProvideFeltAddressType;

impl<Chain: Async> ProvideAddressType<Chain> for ProvideFeltAddressType {
    type Address = Felt;
}
