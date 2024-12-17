use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarknetWallet {
    pub account_address: Felt,
    pub signing_key: Felt,
    pub public_key: Felt,
}
