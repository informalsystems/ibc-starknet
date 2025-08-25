use hermes_prelude::*;
use ibc_core::client::types::Height;
use ibc_core::commitment_types::commitment::CommitmentRoot;
use ibc_core::primitives::Timestamp;
use starknet_block_verifier::{Block, Signature};
use starknet_core::types::StorageProof;

use crate::StarknetConsensusState;

pub const STARKNET_HEADER_TYPE_URL: &str = "/StarknetHeader";

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, HasField)]
pub struct StarknetHeader {
    pub block_header: Block,
    pub block_signature: Signature,
    pub final_height: u64,
    pub storage_proof: StorageProof,
}

impl StarknetHeader {
    pub fn height(&self) -> Height {
        Height::new(0, self.block_header.block_number).expect("Block number exceeds u64 range")
    }

    pub fn timestamp(&self) -> Timestamp {
        Timestamp::from_unix_timestamp(self.block_header.timestamp, 0)
            .expect("Timestamp exceeds u64 range")
    }

    pub fn commitment_root(&self) -> CommitmentRoot {
        self.storage_proof
            .contracts_proof
            .contract_leaves_data
            .first()
            .and_then(|leaf| leaf.storage_root)
            .expect("contract root not found in storage proof")
            .to_bytes_be()
            .to_vec()
            .into()
    }
}

impl From<StarknetHeader> for StarknetConsensusState {
    fn from(header: StarknetHeader) -> Self {
        Self {
            root: header.commitment_root(),
            time: header.timestamp(),
        }
    }
}
