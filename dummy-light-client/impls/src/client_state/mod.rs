pub mod common;
pub mod execution;
pub mod validation;

use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Height;
use ibc_core::primitives::proto::{Any, Protobuf};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct ClientState {
    latest_height: Height,
}

impl Protobuf<Any> for ClientState {}

impl TryFrom<Any> for ClientState {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        if raw.type_url != "/DummyClientState" {
            return Err(ClientError::from("invalid type URL or empty value"));
        }

        let revision_number = u64::from_be_bytes(raw.value.try_into().unwrap());

        Ok(Self {
            latest_height: Height::min(revision_number),
        })
    }
}

impl From<ClientState> for Any {
    fn from(client_state: ClientState) -> Self {
        Self {
            type_url: "/DummyClientState".to_string(),
            value: client_state
                .latest_height
                .revision_number()
                .to_be_bytes()
                .to_vec(),
        }
    }
}
