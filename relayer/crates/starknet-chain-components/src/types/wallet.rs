use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;

use crate::impls::types::address::StarknetAddress;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarknetWallet {
    pub account_address: StarknetAddress,
    pub signing_key: Felt,
    pub public_key: Felt,
}
