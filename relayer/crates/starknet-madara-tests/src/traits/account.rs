use cgp::core::macros::blanket_trait;
use hermes_prelude::CanRaiseAsyncError;
use hermes_starknet_chain_components::traits::HasStarknetAccountType;
use starknet::accounts::{Account, AccountError, ConnectedAccount};

#[blanket_trait]
pub trait CanUseStarknetAccount:
    HasStarknetAccountType<Account: ConnectedAccount>
    + CanRaiseAsyncError<<Self::Account as Account>::SignError>
    + CanRaiseAsyncError<AccountError<<Self::Account as Account>::SignError>>
{
}
