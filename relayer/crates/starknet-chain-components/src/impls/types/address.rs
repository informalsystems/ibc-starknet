use cgp_core::Async;
use starknet::core::types::Felt;

use crate::traits::types::address::ProvideAddressType;

pub struct ProvideFeltAddressType;

impl<Chain: Async> ProvideAddressType<Chain> for ProvideFeltAddressType {
    type Address = Felt;
}
