use cgp::prelude::*;

#[derive_component(SelectorTypeComponent, ProvideSelectorType<Chain>)]
pub trait HasSelectorType: Async {
    type Selector: Async;
}
