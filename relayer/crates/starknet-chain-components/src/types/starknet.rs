use hermes_prelude::*;
use ibc::primitives::Timestamp;
use starknet::core::types::{ByteArray, Felt};

use crate::impls::StarknetAddress;
use crate::types::Height;

#[derive(Debug, Clone, PartialEq, Eq, HasField, HasFields)]
pub struct CairoStarknetClientState {
    pub latest_height: Height,
    pub final_height: u64,
    pub chain_id: ByteArray,
    pub sequencer_public_key: Felt,
    pub ibc_contract_address: StarknetAddress,
}

#[derive(Debug, Clone, PartialEq, Eq, HasField, HasFields)]
pub struct CairoStarknetConsensusState {
    pub root: Felt,
    pub time: Timestamp,
}
