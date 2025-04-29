use cgp::extra::runtime::HasRuntime;
use hermes_core::chain_type_components::traits::{
    AmountDenomGetter, AmountDenomGetterComponent, AmountTypeProviderComponent, HasAmountType,
    HasDenomType,
};
use hermes_core::runtime_components::traits::CanGenerateRandom;
use hermes_core::test_components::chain::traits::{AmountMethodsComponent, ProvideAmountMethods};
use hermes_core::test_components::chain_driver::traits::{
    HasChainType, RandomAmountGenerator, RandomAmountGeneratorComponent,
};
use hermes_prelude::*;

use super::address::StarknetAddress;
use crate::types::StarknetAmount;

pub struct UseU256Amount;

delegate_components! {
    UseU256Amount {
        AmountTypeProviderComponent: UseType<StarknetAmount>
    }
}

#[cgp_provider(AmountDenomGetterComponent)]
impl<Chain> AmountDenomGetter<Chain> for UseU256Amount
where
    Chain: HasAmountType<Amount = StarknetAmount> + HasDenomType<Denom = StarknetAddress>,
{
    fn amount_denom(amount: &StarknetAmount) -> &StarknetAddress {
        &amount.token_address
    }
}

#[cgp_provider(AmountMethodsComponent)]
impl<Chain> ProvideAmountMethods<Chain> for UseU256Amount
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

#[cgp_provider(RandomAmountGeneratorComponent)]
impl<ChainDriver> RandomAmountGenerator<ChainDriver> for UseU256Amount
where
    ChainDriver: HasChainType + HasRuntime,
    ChainDriver::Chain: HasAmountType<Amount = StarknetAmount>,
    ChainDriver::Runtime: CanGenerateRandom<u128>,
{
    async fn random_amount(
        chain_driver: &ChainDriver,
        min: usize,
        max: &StarknetAmount,
    ) -> StarknetAmount {
        // FIXME: figure how to generate random U256 amount

        let max_quantity = max.quantity.low();

        let quantity = chain_driver
            .runtime()
            .random_range(min as u128, max_quantity)
            .await;

        StarknetAmount {
            quantity: quantity.into(),
            token_address: max.token_address,
        }
    }
}
