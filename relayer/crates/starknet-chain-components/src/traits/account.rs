use cgp::core::macros::blanket_trait;
use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use starknet::accounts::{Account, AccountError, ConnectedAccount};

#[cgp_type {
    provider: StarknetAccountTypeProvider,
    context: Chain,
}]
pub trait HasStarknetAccountType: Async {
    type Account: Async;
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
