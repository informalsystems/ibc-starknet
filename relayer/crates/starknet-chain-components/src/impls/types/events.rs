use starknet_crypto::Felt;

use crate::types::{ChannelId, ClientId, ConnectionId, Height};

pub struct StarknetCreateClientEvent {
    pub client_id: ClientId,
}

#[derive(Debug)]
pub struct StarknetUpdateClientEvent {
    pub client_id: ClientId,
    pub consensus_heights: Vec<Height>,
    pub header: Vec<Felt>,
}

pub struct StarknetConnectionOpenInitEvent {
    pub connection_id: ConnectionId,
}

pub struct StarknetConnectionOpenTryEvent {
    pub connection_id: ConnectionId,
}

pub struct StarknetChannelOpenInitEvent {
    pub channel_id: ChannelId,
}

pub struct StarknetChannelOpenTryEvent {
    pub channel_id: ChannelId,
}
