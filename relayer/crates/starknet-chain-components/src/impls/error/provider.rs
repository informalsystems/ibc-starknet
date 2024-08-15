use cgp_core::error::{CanRaiseError, ErrorRaiser};
use starknet::core::types::StarknetError;
use starknet::providers::ProviderError;

pub struct RaiseProviderError;

impl<Context> ErrorRaiser<Context, ProviderError> for RaiseProviderError
where
    Context: CanRaiseError<StarknetError> + CanRaiseError<String>,
{
    fn raise_error(e: ProviderError) -> Context::Error {
        match e {
            ProviderError::StarknetError(e) => Context::raise_error(e),
            _ => Context::raise_error(format!("ProviderError: {:?}", e)),
        }
    }
}
