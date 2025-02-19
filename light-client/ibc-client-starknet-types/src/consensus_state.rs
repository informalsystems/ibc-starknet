use cgp::prelude::*;
use ibc_core::commitment_types::commitment::CommitmentRoot;
use ibc_core::primitives::Timestamp;

pub const STARKNET_CONSENSUS_STATE_TYPE_URL: &str = "/StarknetConsensusState";

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, derive_more::From, HasField)]
pub struct StarknetConsensusState {
    pub root: CommitmentRoot,
    pub time: Timestamp,
}
