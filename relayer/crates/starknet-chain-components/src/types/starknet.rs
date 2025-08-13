use hermes_prelude::*;
use ibc::core::client::types::Height as IbcHeight;
use ibc::primitives::Timestamp;
use ibc_client_starknet_types::{StarknetClientState, StarknetConsensusState};
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

impl From<CairoStarknetClientState> for StarknetClientState {
    fn from(state: CairoStarknetClientState) -> Self {
        let CairoStarknetClientState {
            latest_height,
            final_height,
            chain_id,
            sequencer_public_key,
            ibc_contract_address,
        } = state;

        Self {
            latest_height: IbcHeight::new(
                latest_height.revision_number,
                latest_height.revision_height,
            )
            .unwrap(),
            final_height,
            chain_id: String::try_from(chain_id).unwrap().parse().unwrap(),
            sequencer_public_key: state.sequencer_public_key.to_bytes_be().to_vec(),
            ibc_contract_address: state.ibc_contract_address.to_bytes_be().to_vec(),
        }
    }
}

impl From<CairoStarknetConsensusState> for StarknetConsensusState {
    fn from(state: CairoStarknetConsensusState) -> Self {
        let CairoStarknetConsensusState { root, time } = state;

        Self {
            root: root.to_bytes_be().to_vec().into(),
            time,
        }
    }
}

impl From<StarknetClientState> for CairoStarknetClientState {
    fn from(value: StarknetClientState) -> Self {
        let StarknetClientState {
            latest_height,
            final_height,
            chain_id,
            sequencer_public_key,
            ibc_contract_address,
        } = value;

        Self {
            latest_height: Height {
                revision_number: latest_height.revision_number(),
                revision_height: latest_height.revision_height(),
            },
            final_height,
            chain_id: ByteArray::from(chain_id.as_str()),
            sequencer_public_key: Felt::from_bytes_be_slice(&sequencer_public_key),
            ibc_contract_address: Felt::from_bytes_be_slice(&ibc_contract_address).into(),
        }
    }
}

impl From<StarknetConsensusState> for CairoStarknetConsensusState {
    fn from(value: StarknetConsensusState) -> Self {
        let StarknetConsensusState { root, time } = value;

        Self {
            root: Felt::from_bytes_be_slice(root.as_bytes()),
            time,
        }
    }
}
