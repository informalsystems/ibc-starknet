pub mod common;
pub mod execution;
pub mod validation;

use ibc_client_starknet_types::ClientState as StarknetClientState;
use ibc_core::client::types::error::ClientError;
use ibc_core::primitives::proto::{Any, Protobuf};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct ClientState(StarknetClientState);

impl Protobuf<Any> for ClientState {}

impl TryFrom<Any> for ClientState {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<ClientState> for Any {
    fn from(client_state: ClientState) -> Self {
        todo!()
    }
}
