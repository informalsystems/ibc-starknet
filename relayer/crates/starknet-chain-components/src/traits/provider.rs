use cgp::prelude::*;

#[cgp_component {
    name: StarknetProviderTypeComponent,
    provider: ProvideStarknetProviderType,
    context: Chain,
}]
pub trait HasStarknetProviderType: Async {
    type StarknetProvider: Async;
}

#[cgp_component {
    name: StarknetProviderGetterComponent,
    provider: StarknetProviderGetter,
    context: Chain,
}]
pub trait HasStarknetProvider: HasStarknetProviderType {
    fn provider(&self) -> &Self::StarknetProvider;
}
