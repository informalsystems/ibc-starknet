use cgp::prelude::*;
use ibc_core::client::types::Height;

use crate::StarknetConsensusState;

pub const STARKNET_HEADER_TYPE_URL: &str = "/StarknetHeader";
pub const SIGNED_STARKNET_HEADER_TYPE_URL: &str = "/SignedStarknetHeader";

#[derive(Debug, Clone, HasField)]
pub struct StarknetHeader {
    pub height: Height,
    pub consensus_state: StarknetConsensusState,
}

#[derive(Debug, Clone, HasField)]
pub struct SignedStarknetHeader {
    pub header: Vec<u8>,
    pub signature: Vec<u8>,
}
