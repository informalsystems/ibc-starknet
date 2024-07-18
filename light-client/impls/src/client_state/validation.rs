use ibc_core::client::context::client_state::ClientStateValidation;
use ibc_core::client::context::ClientValidationContext;
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Status;
use ibc_core::host::types::identifiers::ClientId;
use ibc_core::primitives::proto::Any;

use super::ClientState;

impl<V> ClientStateValidation<V> for ClientState
where
    V: ClientValidationContext,
{
    fn verify_client_message(
        &self,
        ctx: &V,
        client_id: &ClientId,
        client_message: Any,
    ) -> Result<(), ClientError> {
        todo!()
    }

    fn check_for_misbehaviour(
        &self,
        ctx: &V,
        client_id: &ClientId,
        client_message: Any,
    ) -> Result<bool, ClientError> {
        todo!()
    }

    fn status(&self, ctx: &V, client_id: &ClientId) -> Result<Status, ClientError> {
        todo!()
    }

    fn check_substitute(&self, ctx: &V, substitute_client_state: Any) -> Result<(), ClientError> {
        todo!()
    }
}
