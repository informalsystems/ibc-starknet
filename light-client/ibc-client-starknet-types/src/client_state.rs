use alloc::vec::Vec;

use hermes_prelude::*;
use ibc_core::client::types::Height;
use ibc_core::host::types::identifiers::ChainId;

pub const STARKNET_CLIENT_STATE_TYPE_URL: &str = "/StarknetClientState";

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, derive_more::From, HasField)]
pub struct StarknetClientState {
    pub latest_height: Height,
    pub chain_id: ChainId,
    pub sequencer_public_key: Vec<u8>,
    pub ibc_contract_address: Vec<u8>,
    pub is_frozen: u8,
}
