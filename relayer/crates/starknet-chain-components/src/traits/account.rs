use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use starknet::accounts::{Account, AccountError, ConnectedAccount};

#[cgp_type {
    provider: StarknetAccountTypeProvider,
    context: Chain,
}]
pub trait HasStarknetAccountType: Async {
    type Account: Async + ConnectedAccount;
}

#[cgp_component {
    provider: AccountFromSignerBuilder,
    context: Chain
}]
pub trait CanBuildAccountFromSigner: HasStarknetAccountType + HasSignerType {
    fn build_account_from_signer(&self, signer: &Self::Signer) -> Self::Account;
}

pub trait CanRaiseAccountErrors:
    HasStarknetAccountType
    + CanRaiseAsyncError<<Self::Account as Account>::SignError>
    + CanRaiseAsyncError<AccountError<<Self::Account as Account>::SignError>>
{
}

impl<Chain> CanRaiseAccountErrors for Chain where
    Chain: HasStarknetAccountType
        + CanRaiseAsyncError<<Chain::Account as Account>::SignError>
        + CanRaiseAsyncError<AccountError<<Self::Account as Account>::SignError>>
{
}
