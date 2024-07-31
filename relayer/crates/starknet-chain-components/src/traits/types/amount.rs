use cgp_core::prelude::*;

#[derive_component(AmountTypeComponent, ProvideAmountType<Chain>)]
pub trait HasAmountType: Async {
    type Amount: Async;
}
