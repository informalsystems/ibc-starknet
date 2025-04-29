use crate::types::{ChannelId, ClientId, ConnectionId};

pub struct StarknetCreateClientEvent {
    pub client_id: ClientId,
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
