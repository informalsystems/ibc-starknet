use cgp::core::Async;
use hermes_test_components::chain::traits::types::amount::ProvideAmountType;
use hermes_test_components::chain::traits::types::denom::HasDenomType;
use starknet::core::types::Felt;

use crate::types::amount::StarknetAmount;

pub struct ProvideU256Amount;

impl<Chain: Async> ProvideAmountType<Chain> for ProvideU256Amount
where
    Chain: HasDenomType<Denom = Felt>,
{
    type Amount = StarknetAmount;

    fn amount_denom(amount: &StarknetAmount) -> &Felt {
        &amount.token_address
    }
}
