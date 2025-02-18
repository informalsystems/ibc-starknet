use cgp::prelude::*;
use hermes_test_components::chain::traits::types::amount::{
    AmountTypeComponent, ProvideAmountType,
};
use hermes_test_components::chain::traits::types::denom::HasDenomType;

use super::address::StarknetAddress;
use crate::types::amount::StarknetAmount;

pub struct ProvideU256Amount;

#[cgp_provider(AmountTypeComponent)]
impl<Chain> ProvideAmountType<Chain> for ProvideU256Amount
where
    Chain: Async + HasDenomType<Denom = StarknetAddress>,
{
    type Amount = StarknetAmount;

    fn amount_denom(amount: &StarknetAmount) -> &StarknetAddress {
        &amount.token_address
    }
}
