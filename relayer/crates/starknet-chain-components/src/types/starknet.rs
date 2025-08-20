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
    pub is_frozen: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, HasField, HasFields)]
pub struct CairoStarknetConsensusState {
    pub root: Felt,
    pub time: Timestamp,
}

impl TryFrom<CairoStarknetClientState> for StarknetClientState {
    type Error = String;

    fn try_from(state: CairoStarknetClientState) -> Result<Self, Self::Error> {
        let CairoStarknetClientState {
            latest_height,
            final_height,
            chain_id,
            sequencer_public_key,
            ibc_contract_address,
            is_frozen,
        } = state;

        let latest_height =
            IbcHeight::new(latest_height.revision_number, latest_height.revision_height)
                .map_err(|e| format!("Invalid height: {e:?}"))?;

        let chain_id_str = String::try_from(chain_id)
            .map_err(|e| format!("Chain ID conversion failed: {e:?}"))?;
        let chain_id = chain_id_str
            .parse()
            .map_err(|e| format!("Chain ID parse failed: {e:?}"))?;

        Ok(Self {
            latest_height,
            final_height,
            chain_id,
            sequencer_public_key: sequencer_public_key.to_bytes_be().to_vec(),
            ibc_contract_address: ibc_contract_address.to_bytes_be().to_vec(),
            is_frozen,
        })
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
            is_frozen,
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
            is_frozen,
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
