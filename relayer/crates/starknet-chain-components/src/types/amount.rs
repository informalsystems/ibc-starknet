use core::fmt::Display;

use starknet::core::types::U256;

use crate::impls::types::address::StarknetAddress;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StarknetAmount {
    pub quantity: U256,
    pub token_address: StarknetAddress,
}

impl StarknetAmount {
    pub fn new(quantity: U256, token_address: StarknetAddress) -> Self {
        Self {
            quantity,
            token_address,
        }
    }
}

impl Display for StarknetAmount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}-{}", self.quantity, self.token_address)
    }
}
