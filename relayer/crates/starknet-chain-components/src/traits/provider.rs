use cgp_core::prelude::*;
use starknet::providers::Provider;

#[derive_component(StarknetProviderTypeComponent, ProvideStarknetProviderType<Chain>)]
pub trait HasStarknetProviderType: Async {
    type Provider: Async + Provider;
}

#[derive_component(StarknetProviderGetterComponent, StarknetProviderGetter<Chain>)]
pub trait HasStarknetProvider: HasStarknetProviderType {
    fn provider(&self) -> &Self::Provider;
}
