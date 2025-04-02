use core::marker::PhantomData;

use cgp::prelude::*;
use starknet::providers::Provider;

use crate::traits::client::{
    HasStarknetClientType, StarknetClientGetter, StarknetClientGetterComponent,
    StarknetClientTypeProvider, StarknetClientTypeProviderComponent,
};

pub struct GetStarknetProviderField<Tag>(pub PhantomData<Tag>);

#[cgp_provider(StarknetClientTypeProviderComponent)]
impl<Chain, Tag> StarknetClientTypeProvider<Chain> for GetStarknetProviderField<Tag>
where
    Chain: Async + HasField<Tag>,
    Tag: Async,
    Chain::Value: Async + Provider,
{
    type Client = Chain::Value;
}

#[cgp_provider(StarknetClientGetterComponent)]
impl<Chain, Tag> StarknetClientGetter<Chain> for GetStarknetProviderField<Tag>
where
    Chain: Async + HasStarknetClientType + HasField<Tag, Value = Chain::Client>,
    Tag: Async,
{
    fn provider(chain: &Chain) -> &Chain::Client {
        chain.get_field(PhantomData)
    }
}
