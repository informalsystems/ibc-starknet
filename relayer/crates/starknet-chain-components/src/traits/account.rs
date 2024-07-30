use cgp_core::prelude::*;
use starknet::accounts::Account;

#[derive_component(StarknetAccountTypeComponent, ProvideStarknetAccountType<Chain>)]
pub trait HasStarknetAccountType: Async {
    type Account: Async + Account;
}

#[derive_component(StarknetAccountGetterComponent, StarknetAccountGetter<Chain>)]
pub trait HasStarknetAccount: HasStarknetAccountType {
    fn account(&self) -> &Self::Account;
}
