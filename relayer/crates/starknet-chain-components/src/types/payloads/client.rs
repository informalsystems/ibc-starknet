use hermes_prelude::*;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ChainId;
use ibc_client_starknet_types::header::StarknetHeader;
use starknet_v14::core::types::StorageProof;

use crate::types::{
    CairoStarknetClientState, CairoStarknetConsensusState, WasmStarknetConsensusState,
};

#[derive(Debug, HasField)]
pub struct StarknetCreateClientPayload {
    pub latest_height: Height,
    pub final_height: u64,
    pub chain_id: ChainId,
    pub client_state_wasm_code_hash: Vec<u8>,
    pub consensus_state: WasmStarknetConsensusState,
    pub sequencer_public_key: Vec<u8>,
    pub ibc_contract_address: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct StarknetCreateClientPayloadOptions {
    pub wasm_code_hash: [u8; 32],
}

#[derive(Debug)]
pub struct StarknetUpdateClientPayload {
    pub header: StarknetHeader,
}

#[derive(Clone, Debug)]
pub struct StarknetUpgradeClientPayload {
    pub upgrade_height: Height,
    pub client_state: CairoStarknetClientState,
    pub consensus_state: CairoStarknetConsensusState,
    pub client_state_proof: StorageProof,
    pub consensus_state_proof: StorageProof,
}
