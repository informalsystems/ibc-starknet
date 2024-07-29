use ibc_core::client::types::error::ClientError;
use ibc_core::commitment_types::commitment::CommitmentRoot;
use ibc_core::primitives::proto::{Any, Protobuf};

const TYPE_URL: &str = "/DummyConsensusState";

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct ConsensusState {
    root: CommitmentRoot,
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
        if raw.type_url != TYPE_URL || !raw.value.is_empty() {
            return Err(ClientError::UnknownConsensusStateType {
                consensus_state_type: raw.type_url,
            });
        }

        Ok(Self::default())
    }
}

impl From<ConsensusState> for Any {
    fn from(consensus_state: ConsensusState) -> Self {
        Self {
            type_url: "/DummyConsensusState".to_string(),
            value: vec![],
        }
    }
}
