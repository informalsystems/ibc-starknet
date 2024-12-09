use cgp::prelude::*;
use starknet::accounts::{Account, AccountError, ConnectedAccount};

#[cgp_component {
  name: StarknetAccountTypeComponent,
  provider: ProvideStarknetAccountType,
  context: Chain,
}]
pub trait HasStarknetAccountType: Async {
    type Account: Async + ConnectedAccount;
}

#[cgp_component {
  name: StarknetAccountGetterComponent,
  provider: StarknetAccountGetter,
  context: Chain,
}]
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
