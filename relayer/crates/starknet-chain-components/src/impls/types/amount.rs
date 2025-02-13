use cgp::core::Async;
use hermes_test_components::chain::traits::types::amount::ProvideAmountType;
use hermes_test_components::chain::traits::types::denom::HasDenomType;

use super::address::StarknetAddress;
use crate::types::amount::StarknetAmount;

pub struct ProvideU256Amount;

impl<Chain: Async> ProvideAmountType<Chain> for ProvideU256Amount
where
    Chain: HasDenomType<Denom = StarknetAddress>,
{
    type Amount = StarknetAmount;

    fn amount_denom(amount: &StarknetAmount) -> &StarknetAddress {
        &amount.token_address
    }
}
