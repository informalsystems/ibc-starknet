use cgp::prelude::*;
use starknet::providers::Provider;

#[cgp_component {
  name: StarknetProviderTypeComponent,
  provider: ProvideStarknetProviderType,
  context: Chain,
}]
pub trait HasStarknetProviderType: Async {
    type Provider: Async + Provider;
}

#[cgp_component {
  name: StarknetProviderGetterComponent,
  provider: StarknetProviderGetter,
  context: Chain,
}]
pub trait HasStarknetProvider: HasStarknetProviderType {
    fn provider(&self) -> &Self::Provider;
}
