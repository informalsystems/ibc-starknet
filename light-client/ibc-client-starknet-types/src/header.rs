use cgp::prelude::*;
use ibc_core::client::types::Height;

use crate::StarknetConsensusState;

pub const STARKNET_HEADER_TYPE_URL: &str = "/StarknetHeader";

#[derive(Debug, Clone, HasField)]
pub struct StarknetHeader {
    pub height: Height,
    pub consensus_state: StarknetConsensusState,
}
