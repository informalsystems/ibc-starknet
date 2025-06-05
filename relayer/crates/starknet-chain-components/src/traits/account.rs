use core::fmt::Debug;

use cgp::core::macros::blanket_trait;
use hermes_core::relayer_components::transaction::traits::HasSignerType;
use hermes_prelude::*;
use starknet::accounts::{Account, AccountError, ConnectedAccount};

#[cgp_type {
    provider: StarknetAccountTypeProvider,
    context: Chain,
}]
pub trait HasStarknetAccountType: Async {
    type Account: Async + Debug;
}

#[cgp_component {
    provider: AccountFromSignerBuilder,
    context: Chain
}]
pub trait CanBuildAccountFromSigner: HasStarknetAccountType + HasSignerType {
    fn build_account_from_signer(&self, signer: &Self::Signer) -> Self::Account;
}

#[blanket_trait]
pub trait CanUseStarknetAccount:
    HasStarknetAccountType<Account: ConnectedAccount>
    + CanRaiseAsyncError<<Self::Account as Account>::SignError>
    + CanRaiseAsyncError<AccountError<<Self::Account as Account>::SignError>>
{
}
