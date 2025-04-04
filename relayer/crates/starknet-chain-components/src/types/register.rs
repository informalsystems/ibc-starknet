use cgp::prelude::*;
use starknet::core::types::Felt;

use crate::impls::types::address::StarknetAddress;
use crate::types::messages::ibc::channel::PortId;

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
