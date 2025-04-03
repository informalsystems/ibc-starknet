use cgp::core::error::{ErrorRaiser, ErrorRaiserComponent};
use cgp::prelude::*;
use starknet_v13::core::types::StarknetError;
pub struct RaiseStarknetError;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context> ErrorRaiser<Context, StarknetError> for RaiseStarknetError
where
    Context: CanRaiseAsyncError<String>,
{
    fn raise_error(e: StarknetError) -> Context::Error {
        match e {
            StarknetError::ContractError(e) => {
                Context::raise_error(format!("ContractError: {:?}", e.revert_error))
            }
            _ => Context::raise_error(format!("StarknetError: {e:?}")),
        }
    }
}
