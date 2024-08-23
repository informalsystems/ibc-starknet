use ibc_core::client::types::error::ClientError;
use ibc_core::commitment_types::commitment::CommitmentRoot;
use ibc_core::primitives::proto::{Any, Protobuf};
use prost::Message;

pub const CONSENSUS_STATE_TYPE_URL: &str = "/StarknetConsensusState";

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct ConsensusState {
    root: CommitmentRoot,
}

#[derive(Clone, Message)]
pub struct ProtoConsensusState {
    #[prost(message, tag = "1")]
    pub root: Option<Vec<u8>>,
}

impl ConsensusState {
    pub fn root(&self) -> &CommitmentRoot {
        &self.root
    }
}

impl Default for ConsensusState {
    fn default() -> Self {
        Self {
            root: CommitmentRoot::from(vec![]),
        }
    }
}

impl Protobuf<Any> for ConsensusState {}

impl TryFrom<Any> for ConsensusState {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        if raw.type_url != CONSENSUS_STATE_TYPE_URL || !raw.value.is_empty() {
            return Err(ClientError::UnknownConsensusStateType {
                consensus_state_type: raw.type_url,
            });
        }

        let proto_consensus_state =
            ProtoConsensusState::decode(raw.value.as_ref()).map_err(|e| ClientError::Other {
                description: e.to_string(),
            })?;

        proto_consensus_state.try_into()
    }
}

impl From<ConsensusState> for Any {
    fn from(consensus_state: ConsensusState) -> Self {
        Self {
            type_url: CONSENSUS_STATE_TYPE_URL.to_string(),
            value: ProtoConsensusState::from(consensus_state).encode_to_vec(),
        }
    }
}

impl From<ConsensusState> for ProtoConsensusState {
    fn from(consensus_state: ConsensusState) -> Self {
        ProtoConsensusState {
            root: Some(consensus_state.root.into_vec()),
        }
    }
}

impl TryFrom<ProtoConsensusState> for ConsensusState {
    type Error = ClientError;

    fn try_from(proto_consensus_state: ProtoConsensusState) -> Result<Self, Self::Error> {
        let root = proto_consensus_state
            .root
            .ok_or_else(|| ClientError::Other {
                description: "empty commitment root".into(),
            })?
            .into();

        Ok(ConsensusState { root })
    }
}
