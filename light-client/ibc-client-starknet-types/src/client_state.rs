use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Height;
use ibc_core::primitives::proto::Any;
use ibc_proto::ibc::core::client::v1::Height as ProtoHeight;
use prost::Message;

pub const CLIENT_STATE_TYPE_URL: &str = "/StarknetClientState";

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct StarknetClientState {
    pub latest_height: Height,
}

pub struct EncodeClientState;

#[derive(Clone, Message)]
pub struct ProtoClientState {
    #[prost(message, tag = "1")]
    pub latest_height: Option<ProtoHeight>,
}

impl TryFrom<ProtoClientState> for StarknetClientState {
    type Error = ClientError;

    fn try_from(proto_client_state: ProtoClientState) -> Result<Self, Self::Error> {
        let latest_height = proto_client_state
            .latest_height
            .ok_or(ClientError::InvalidHeight)?
            .try_into()
            .map_err(|_| ClientError::InvalidHeight)?;

        let client_state = StarknetClientState { latest_height };

        Ok(client_state)
    }
}

impl From<StarknetClientState> for ProtoClientState {
    fn from(client_state: StarknetClientState) -> Self {
        ProtoClientState {
            latest_height: Some(client_state.latest_height.into()),
        }
    }
}

impl TryFrom<Any> for StarknetClientState {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, ClientError> {
        if raw.type_url != CLIENT_STATE_TYPE_URL {
            return Err(ClientError::UnknownClientStateType {
                client_state_type: raw.type_url,
            });
        }

        let proto_client_state =
            ProtoClientState::decode(raw.value.as_ref()).map_err(|e| ClientError::Other {
                description: e.to_string(),
            })?;

        proto_client_state.try_into()
    }
}

impl From<StarknetClientState> for Any {
    fn from(client_state: StarknetClientState) -> Self {
        Self {
            type_url: CLIENT_STATE_TYPE_URL.to_string(),
            value: ProtoClientState::from(client_state).encode_to_vec(),
        }
    }
}
