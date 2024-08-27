use ibc_client_starknet_types::{ConsensusState, ProtoConsensusState, CONSENSUS_STATE_TYPE_URL};

pub type StarknetConsensusState = ConsensusState;

pub type ProtoStarknetConsensusState = ProtoConsensusState;

pub const STARKNET_CONSENSUS_STATE_TYPE_URL: &str = CONSENSUS_STATE_TYPE_URL;
