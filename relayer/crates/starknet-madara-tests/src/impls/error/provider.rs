use cgp::core::error::{ErrorRaiser, ErrorRaiserComponent};
use cgp::prelude::*;
use starknet_v13::core::types::StarknetError;
use starknet_v13::providers::ProviderError;

pub struct RaiseProviderError;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context> ErrorRaiser<Context, ProviderError> for RaiseProviderError
where
    Context: CanRaiseAsyncError<StarknetError> + CanRaiseAsyncError<String>,
{
    fn raise_error(e: ProviderError) -> Context::Error {
        match e {
            ProviderError::StarknetError(e) => Context::raise_error(e),
            _ => Context::raise_error(format!("ProviderError: {e:?}")),
        }
    }
}
