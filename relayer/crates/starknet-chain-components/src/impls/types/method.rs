use cgp_core::Async;
use starknet::core::types::Felt;

use crate::traits::types::method::ProvideMethodSelectorType;

pub struct ProvideFeltMethodSelector;

impl<Chain: Async> ProvideMethodSelectorType<Chain> for ProvideFeltMethodSelector {
    type MethodSelector = Felt;
}
