use core::marker::PhantomData;

use cgp::prelude::*;
use starknet::accounts::ConnectedAccount;

use crate::traits::account::{
    HasStarknetAccountType, ProvideStarknetAccountType, StarknetAccountGetter,
    StarknetAccountGetterComponent, StarknetAccountTypeComponent,
};

pub struct GetStarknetAccountField<Tag>(pub PhantomData<Tag>);

#[cgp_provider(StarknetAccountTypeComponent)]
impl<Chain, Tag> ProvideStarknetAccountType<Chain> for GetStarknetAccountField<Tag>
where
    Chain: Async + HasField<Tag>,
    Tag: Async,
    Chain::Value: Async + ConnectedAccount,
{
    type Account = Chain::Value;
}

#[cgp_provider(StarknetAccountGetterComponent)]
impl<Chain, Tag> StarknetAccountGetter<Chain> for GetStarknetAccountField<Tag>
where
    Chain: Async + HasStarknetAccountType + HasField<Tag, Value = Chain::Account>,
    Tag: Async,
{
    fn account(chain: &Chain) -> &Chain::Account {
        chain.get_field(PhantomData)
    }
}
