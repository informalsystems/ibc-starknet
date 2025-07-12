use alloc::vec::Vec;

use hermes_prelude::*;
use ibc_core::client::types::Height;
use ibc_core::commitment_types::commitment::CommitmentRoot;
use ibc_core::primitives::Timestamp;

use crate::StarknetConsensusState;

// use starknet_block_verifier::{Block, Signature};
// use starknet_core::types::StorageProof;

pub const STARKNET_HEADER_TYPE_URL: &str = "/StarknetHeader";
pub const SIGNED_STARKNET_HEADER_TYPE_URL: &str = "/SignedStarknetHeader";

#[derive(Debug, Clone, HasField)]
pub struct StarknetHeader {
    // pub block_header: Block,
    // pub block_signature: Signature,
    // pub storage_proof: StorageProof,
    pub block_header: Vec<u8>,
    pub block_signature: Vec<u8>,
    pub storage_proof: Vec<u8>,
}

#[derive(Debug, Clone, HasField)]
pub struct SignedStarknetHeader {
    pub header: Vec<u8>,
    pub signature: Vec<u8>,
}

impl StarknetHeader {
    pub fn height(&self) -> Height {
        todo!()
    }
}

impl From<StarknetHeader> for StarknetConsensusState {
    fn from(header: StarknetHeader) -> Self {
        todo!()
    }
}
