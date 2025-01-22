use crate::types::channel_id::ChannelId;
use crate::types::client_id::ClientId;
use crate::types::connection_id::ConnectionId;

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
