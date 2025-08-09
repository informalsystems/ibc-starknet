use crate::types::{ChannelId, ClientId, ConnectionId, Height};

pub struct StarknetCreateClientEvent {
    pub client_id: ClientId,
}

pub struct StarknetScheduleUpgradeEvent {
    pub upgrade_height: Height,
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
