use ibc_client_starknet_types::{
    ClientState, ConsensusState, ProtoClientState, ProtoConsensusState, CLIENT_STATE_TYPE_URL,
    CONSENSUS_STATE_TYPE_URL,
};

pub type StarknetClientState = ClientState;

pub type StarknetConsensusState = ConsensusState;

pub type ProtoStarknetClientState = ProtoClientState;

pub type ProtoStarknetConsensusState = ProtoConsensusState;

pub const STARKNET_CLIENT_STATE_TYPE_URL: &str = CLIENT_STATE_TYPE_URL;

pub const STARKNET_CONSENSUS_STATE_TYPE_URL: &str = CONSENSUS_STATE_TYPE_URL;
