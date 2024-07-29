use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Height;
use ibc_core::primitives::proto::{Any, Protobuf};

pub const CLIENT_STATE_TYPE_URL: &str = "/DummyClientState";

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct ClientState {
    pub latest_height: Height,
}

impl Protobuf<Any> for ClientState {}

impl TryFrom<Any> for ClientState {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        if raw.type_url != CLIENT_STATE_TYPE_URL {
            return Err(ClientError::UnknownClientStateType {
                client_state_type: raw.type_url,
            });
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
