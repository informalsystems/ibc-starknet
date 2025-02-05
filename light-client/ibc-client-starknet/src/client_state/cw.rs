use ibc_client_cw::api::CwClientStateExecution;
use ibc_client_cw::context::client_ctx::CwClientExecution;

use super::ClientState;
use crate::ConsensusState;

impl<'a, E> CwClientStateExecution<'a, E> for ClientState
where
    E: CwClientExecution<'a, ClientStateMut = ClientState, ConsensusStateRef = ConsensusState>,
{
    fn public_key(&self) -> Option<Vec<u8>> {
        Some(self.0.pub_key.clone())
    }
}
