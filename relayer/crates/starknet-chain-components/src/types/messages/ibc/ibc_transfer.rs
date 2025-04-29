use std::fmt::Display;

use hermes_prelude::*;
use starknet::core::types::U256;

use crate::impls::StarknetAddress;
use crate::types::{ChannelId, Height, PortId, PrefixedDenom, Timestamp};

#[derive(HasField, HasFields)]
pub struct TransferPacketData {
    pub denom: PrefixedDenom,
    pub amount: U256,
    pub sender: Participant,
    pub receiver: Participant,
    pub memo: String,
}

#[derive(HasField, HasFields)]
pub struct MsgTransfer {
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub denom: PrefixedDenom,
    pub amount: U256,
    pub receiver: String,
    pub memo: String,
    pub timeout_height_on_b: Height,
    pub timeout_timestamp_on_b: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, HasFields)]
pub enum Participant {
    Native(StarknetAddress),
    External(String),
}

impl Display for Participant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native(address) => write!(f, "{address}"),
            Self::External(address) => write!(f, "{address}"),
        }
    }
}
