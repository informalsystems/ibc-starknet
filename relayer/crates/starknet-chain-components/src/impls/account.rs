use std::marker::PhantomData;

use cgp_core::prelude::*;
use starknet::accounts::Account;

use crate::traits::account::{
    HasStarknetAccountType, ProvideStarknetAccountType, StarknetAccountGetter,
};

pub struct GetStarknetAccountField<Tag>(pub PhantomData<Tag>);

impl<Chain, Tag> ProvideStarknetAccountType<Chain> for GetStarknetAccountField<Tag>
where
    Chain: Async + HasField<Tag>,
    Tag: Async,
    Chain::Field: Async + Account,
{
    type Account = Chain::Field;
}

impl<Chain, Tag> StarknetAccountGetter<Chain> for GetStarknetAccountField<Tag>
where
    Chain: Async + HasStarknetAccountType + HasField<Tag, Field = Chain::Account>,
    Tag: Async,
{
    fn account(chain: &Chain) -> &Chain::Account {
        chain.get_field(PhantomData)
    }
}
