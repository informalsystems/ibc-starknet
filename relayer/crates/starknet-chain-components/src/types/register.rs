use hermes_prelude::*;
use starknet::core::types::Felt;

use crate::impls::StarknetAddress;
use crate::types::PortId;

#[derive(HasField, HasFields)]
pub struct MsgRegisterClient {
    pub client_type: Felt,
    pub contract_address: StarknetAddress,
}

#[derive(HasField, HasFields)]
pub struct MsgRegisterApp {
    pub port_id: PortId,
    pub contract_address: StarknetAddress,
}
