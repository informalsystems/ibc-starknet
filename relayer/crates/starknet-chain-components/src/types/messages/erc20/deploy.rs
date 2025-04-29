use hermes_prelude::*;

use crate::impls::types::address::StarknetAddress;
#[derive(HasField, HasFields)]
pub struct DeployErc20TokenMessage {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub owner: StarknetAddress,
}
