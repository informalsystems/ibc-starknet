use hermes_prelude::*;
use ibc_core::host::types::identifiers::ClientId;

use crate::header::StarknetHeader;

pub const STARKNET_MISBEHAVIOUR_TYPE_URL: &str = "/StarknetMisbehaviour";

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, HasField)]
pub struct StarknetMisbehaviour {
    pub client_id: ClientId,
    pub header_1: StarknetHeader,
    pub header_2: StarknetHeader,
}
