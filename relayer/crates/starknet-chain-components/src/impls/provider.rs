use std::marker::PhantomData;

use cgp::prelude::*;
use starknet::providers::Provider;

use crate::traits::provider::{
    HasStarknetProviderType, ProvideStarknetProviderType, StarknetProviderGetter,
};

pub struct GetStarknetProviderField<Tag>(pub PhantomData<Tag>);

impl<Chain, Tag> ProvideStarknetProviderType<Chain> for GetStarknetProviderField<Tag>
where
    Chain: Async + HasField<Tag>,
    Tag: Async,
    Chain::Field: Async + Provider,
{
    type Provider = Chain::Field;
}

impl<Chain, Tag> StarknetProviderGetter<Chain> for GetStarknetProviderField<Tag>
where
    Chain: Async + HasStarknetProviderType + HasField<Tag, Field = Chain::Provider>,
    Tag: Async,
{
    fn provider(chain: &Chain) -> &Chain::Provider {
        chain.get_field(PhantomData)
    }
}
