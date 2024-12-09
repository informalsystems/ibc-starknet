use cgp::prelude::*;

#[cgp_component {
  name: SelectorTypeComponent,
  provider: ProvideSelectorType,
  context: Chain,
}]
pub trait HasSelectorType: Async {
    type Selector: Async;
}
