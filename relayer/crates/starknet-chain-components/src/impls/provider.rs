use core::marker::PhantomData;

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
    Chain::Value: Async + Provider,
{
    type StarknetProvider = Chain::Value;
}

impl<Chain, Tag> StarknetProviderGetter<Chain> for GetStarknetProviderField<Tag>
where
    Chain: Async + HasStarknetProviderType + HasField<Tag, Value = Chain::StarknetProvider>,
    Tag: Async,
{
    fn provider(chain: &Chain) -> &Chain::StarknetProvider {
        chain.get_field(PhantomData)
    }
}
