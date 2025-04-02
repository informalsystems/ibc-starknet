use cgp::prelude::*;

#[cgp_type {
    provider: StarknetClientTypeProvider,
    context: Chain,
}]
pub trait HasStarknetClientType: Async {
    type Client: Async;
}

#[cgp_getter {
    provider: StarknetClientGetter,
    context: Chain,
}]
pub trait HasStarknetClient: HasStarknetClientType {
    fn provider(&self) -> &Self::Client;
}
