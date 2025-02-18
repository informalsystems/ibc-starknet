use cgp::prelude::*;
use starknet::core::types::Felt;

use crate::components::chain::SelectorTypeComponent;
use crate::traits::types::method::ProvideSelectorType;

pub struct ProvideFeltSelector;

#[cgp_provider(SelectorTypeComponent)]
impl<Chain: Async> ProvideSelectorType<Chain> for ProvideFeltSelector {
    type Selector = Felt;
}
