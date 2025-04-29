use hermes_prelude::*;
use starknet::core::types::Felt;

use crate::traits::types::method::{ProvideSelectorType, SelectorTypeComponent};

pub struct ProvideFeltSelector;

#[cgp_provider(SelectorTypeComponent)]
impl<Chain: Async> ProvideSelectorType<Chain> for ProvideFeltSelector {
    type Selector = Felt;
}
