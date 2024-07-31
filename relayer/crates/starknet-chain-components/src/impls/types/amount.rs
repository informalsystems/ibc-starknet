use cgp_core::Async;
use starknet::core::types::U256;

use crate::traits::types::amount::ProvideAmountType;

pub struct ProvideU256Amount;

impl<Chain: Async> ProvideAmountType<Chain> for ProvideU256Amount {
    type Amount = U256;
}
