pub mod common;
pub mod cw;
pub mod execution;
pub mod validation;

use hermes_encoding_components::traits::convert::CanConvert;
use ibc_client_starknet_types::StarknetClientState as ClientStateType;
use ibc_core::client::types::error::ClientError;
use ibc_core::primitives::proto::{Any, Protobuf};
use prost_types::Any as ProstAny;

use crate::encoding::context::StarknetLightClientEncoding;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct ClientState(pub ClientStateType);

impl Protobuf<Any> for ClientState {}

impl TryFrom<Any> for ClientState {
    type Error = ClientError;

    fn try_from(any: Any) -> Result<Self, Self::Error> {
        let client_state: ClientStateType = StarknetLightClientEncoding.convert(&ProstAny {
            type_url: any.type_url,
            value: any.value,
        })?;

        Ok(client_state.into())
    }
}

impl From<ClientState> for Any {
    fn from(client_state: ClientState) -> Self {
        let any = StarknetLightClientEncoding
            .convert(&client_state.0)
            .unwrap();

        Self {
            type_url: any.type_url,
            value: any.value,
        }
    }
}
