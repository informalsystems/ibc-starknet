use ibc_core::client::context::client_state::ClientStateValidation;
use ibc_core::client::context::ClientValidationContext;
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Status;
use ibc_core::host::types::identifiers::ClientId;
use ibc_core::primitives::proto::Any;

use super::ClientState;
use crate::ConsensusState;

impl<V> ClientStateValidation<V> for ClientState
where
    V: ClientValidationContext<ClientStateRef = ClientState, ConsensusStateRef = ConsensusState>,
{
    fn verify_client_message(
        &self,
        ctx: &V,
        client_id: &ClientId,
        client_message: Any,
    ) -> Result<(), ClientError> {
        Ok(())
    }

    fn check_for_misbehaviour(
        &self,
        ctx: &V,
        client_id: &ClientId,
        client_message: Any,
    ) -> Result<bool, ClientError> {
        Ok(false)
    }

    fn status(&self, ctx: &V, client_id: &ClientId) -> Result<Status, ClientError> {
        Ok(Status::Active)
    }

    fn check_substitute(&self, ctx: &V, substitute_client_state: Any) -> Result<(), ClientError> {
        Ok(())
    }
}
