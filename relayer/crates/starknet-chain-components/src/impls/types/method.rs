use cgp_core::Async;
use starknet::core::types::Felt;

use crate::traits::types::method::ProvideSelectorType;

pub struct ProvideFeltSelector;

impl<Chain: Async> ProvideSelectorType<Chain> for ProvideFeltSelector {
    type Selector = Felt;
}
