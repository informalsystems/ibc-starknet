use cgp::core::macros::blanket_trait;
use cgp::prelude::CanRaiseAsyncError;
use hermes_starknet_chain_components::traits::account::HasStarknetAccountType;
use starknet_v13::accounts::{Account, AccountError, ConnectedAccount};

#[blanket_trait]
pub trait CanUseStarknetAccount:
    HasStarknetAccountType<Account: ConnectedAccount>
    + CanRaiseAsyncError<<Self::Account as Account>::SignError>
    + CanRaiseAsyncError<AccountError<<Self::Account as Account>::SignError>>
{
}
