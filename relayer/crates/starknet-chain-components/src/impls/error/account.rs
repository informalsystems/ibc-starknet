use core::fmt::Debug;

use cgp::core::error::{CanRaiseAsyncError, ErrorRaiser};
use starknet::accounts::AccountError;
use starknet::providers::ProviderError;

pub struct RaiseAccountError;

impl<Context, S> ErrorRaiser<Context, AccountError<S>> for RaiseAccountError
where
    Context: CanRaiseAsyncError<ProviderError> + CanRaiseAsyncError<String>,
    S: Debug,
{
    fn raise_error(e: AccountError<S>) -> Context::Error {
        match e {
            AccountError::Provider(e) => Context::raise_error(e),
            _ => Context::raise_error(format!("AccountError: {:?}", e)),
        }
    }
}
