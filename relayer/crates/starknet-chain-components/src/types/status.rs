use core::fmt::{Display, Formatter, Result};

use hermes_cosmos_chain_components::types::status::Time;
use serde::Serialize;
use starknet::core::types::Felt;

#[derive(Debug, Serialize)]
pub struct StarknetChainStatus {
    pub height: u64,
    pub block_hash: Felt,
    pub time: Time,
}

impl Display for StarknetChainStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "height: {}, block_hash: {}, time: {}",
            self.height, self.block_hash, self.time
        )
    }
}
