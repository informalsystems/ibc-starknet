use cgp::prelude::*;
use starknet::accounts::{Account, AccountError, ConnectedAccount};

#[derive_component(StarknetAccountTypeComponent, ProvideStarknetAccountType<Chain>)]
pub trait HasStarknetAccountType: Async {
    type Account: Async + ConnectedAccount;
}

#[derive_component(StarknetAccountGetterComponent, StarknetAccountGetter<Chain>)]
pub trait HasStarknetAccount: HasStarknetAccountType {
    fn account(&self) -> &Self::Account;
}

pub trait CanRaiseAccountErrors:
    HasStarknetAccountType
    + CanRaiseError<<Self::Account as Account>::SignError>
    + CanRaiseError<AccountError<<Self::Account as Account>::SignError>>
{
}

impl<Chain> CanRaiseAccountErrors for Chain where
    Chain: HasStarknetAccountType
        + CanRaiseError<<Chain::Account as Account>::SignError>
        + CanRaiseError<AccountError<<Self::Account as Account>::SignError>>
{
}
