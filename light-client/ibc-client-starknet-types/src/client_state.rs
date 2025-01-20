use cgp::prelude::*;
use ibc_core::client::types::Height;
use ibc_core::host::types::identifiers::ChainId;
use secp256k1::PublicKey;

pub const STARKNET_CLIENT_STATE_TYPE_URL: &str = "/StarknetClientState";

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, derive_more::From, HasField)]
pub struct StarknetClientState {
    pub latest_height: Height,
    pub chain_id: ChainId,
    pub pub_key: PublicKey,
}
