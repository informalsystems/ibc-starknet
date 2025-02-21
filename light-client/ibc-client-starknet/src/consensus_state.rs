use hermes_encoding_components::traits::convert::CanConvert;
use ibc_client_starknet_types::StarknetConsensusState as ConsensusStateType;
use ibc_core::client::context::consensus_state::ConsensusState as ConsensusStateTrait;
use ibc_core::client::types::error::ClientError;
use ibc_core::commitment_types::commitment::CommitmentRoot;
use ibc_core::primitives::proto::{Any, Protobuf};
use ibc_core::primitives::Timestamp;
use prost_types::Any as ProstAny;

use crate::encoding::context::StarknetLightClientEncoding;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, derive_more::From)]
pub struct ConsensusState(pub ConsensusStateType);

impl Protobuf<Any> for ConsensusState {}

impl TryFrom<Any> for ConsensusState {
    type Error = ClientError;

    fn try_from(any: Any) -> Result<Self, Self::Error> {
        let consensus_state: ConsensusStateType =
            StarknetLightClientEncoding.convert(&ProstAny {
                type_url: any.type_url,
                value: any.value,
            })?;

        Ok(consensus_state.into())
    }
}

impl From<ConsensusState> for Any {
    fn from(consensus_state: ConsensusState) -> Self {
        let any: ProstAny = StarknetLightClientEncoding
            .convert(&consensus_state.0)
            .unwrap();

        Self {
            type_url: any.type_url,
            value: any.value,
        }
    }
}

impl ConsensusStateTrait for ConsensusState {
    fn root(&self) -> &CommitmentRoot {
        &self.0.root
    }

    fn timestamp(&self) -> Result<Timestamp, ClientError> {
        Ok(self.0.time)
    }
}
