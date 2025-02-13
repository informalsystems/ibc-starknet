use cgp::core::error::{CanRaiseAsyncError, ErrorRaiser};
use starknet::core::types::StarknetError;

pub struct RaiseStarknetError;

impl<Context> ErrorRaiser<Context, StarknetError> for RaiseStarknetError
where
    Context: CanRaiseAsyncError<String>,
{
    fn raise_error(e: StarknetError) -> Context::Error {
        match e {
            StarknetError::ContractError(e) => {
                Context::raise_error(format!("ContractError: {}", e.revert_error))
            }
            _ => Context::raise_error(format!("StarknetError: {:?}", e)),
        }
    }
}
