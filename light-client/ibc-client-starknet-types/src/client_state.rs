use cgp::prelude::*;
use ibc_core::client::types::Height;

pub const CLIENT_STATE_TYPE_URL: &str = "/StarknetClientState";

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, derive_more::From, HasField)]
pub struct StarknetClientState {
    pub latest_height: Height,
}
