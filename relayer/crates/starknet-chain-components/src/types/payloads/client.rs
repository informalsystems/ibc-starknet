use cgp::prelude::*;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ChainId;
use ibc_client_starknet_types::header::StarknetHeader;

use crate::types::consensus_state::WasmStarknetConsensusState;

#[derive(Debug, HasField)]
pub struct StarknetCreateClientPayload {
    pub latest_height: Height,
    pub chain_id: ChainId,
    pub client_state_wasm_code_hash: Vec<u8>,
    pub consensus_state: WasmStarknetConsensusState,
}

pub struct StarknetCreateClientPayloadOptions {
    pub wasm_code_hash: [u8; 32],
}

pub struct StarknetUpdateClientPayload {
    pub header: StarknetHeader,
}
