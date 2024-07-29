use cgp_core::prelude::*;

#[derive_component(MethodSelectorTypeComponent, ProvideMethodSelectorType<Chain>)]
pub trait HasMethodSelectorType: Async {
    type MethodSelector: Async;
}
