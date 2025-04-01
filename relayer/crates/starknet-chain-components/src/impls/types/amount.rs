use cgp::prelude::*;
use hermes_test_components::chain::traits::types::amount::{
    AmountMethodsComponent, AmountTypeComponent, HasAmountType, ProvideAmountMethods,
    ProvideAmountType,
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

#[cgp_provider(AmountMethodsComponent)]
impl<Chain> ProvideAmountMethods<Chain> for ProvideU256Amount
where
    Chain: HasAmountType<Amount = StarknetAmount> + CanRaiseAsyncError<&'static str>,
{
    fn add_amount(
        current: &StarknetAmount,
        amount: &StarknetAmount,
    ) -> Result<StarknetAmount, Chain::Error> {
        if current.token_address != amount.token_address {
            return Err(Chain::raise_error("mismatch denom"));
        }

        let quantity = current.quantity + amount.quantity;

        Ok(StarknetAmount {
            quantity,
            token_address: current.token_address,
        })
    }

    fn subtract_amount(
        current: &StarknetAmount,
        amount: &StarknetAmount,
    ) -> Result<StarknetAmount, Chain::Error> {
        if current.token_address != amount.token_address {
            return Err(Chain::raise_error("mismatch denom"));
        }

        let quantity = current.quantity - amount.quantity;

        Ok(StarknetAmount {
            quantity,
            token_address: current.token_address,
        })
    }
}
