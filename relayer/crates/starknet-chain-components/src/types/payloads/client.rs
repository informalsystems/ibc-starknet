use cgp::prelude::*;

use crate::types::client_state::WasmStarknetClientState;
use crate::types::consensus_state::{StarknetConsensusState, WasmStarknetConsensusState};

#[derive(Debug, HasField)]
pub struct StarknetCreateClientPayload {
    pub client_state: WasmStarknetClientState,
    pub consensus_state: WasmStarknetConsensusState,
}

pub struct StarknetCreateClientPayloadOptions {
    pub wasm_code_hash: [u8; 32],
}

pub struct StarknetUpdateClientPayload {
    pub consensus_state: StarknetConsensusState,
}
