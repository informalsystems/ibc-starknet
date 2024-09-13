use starknet_ibc_core::channel::{ChannelEnd, ChannelState, ChannelOrdering, Counterparty};
use starknet_ibc_core::host::{ClientId, PortId, ChannelId, Sequence};

pub fn CLIENT_ID() -> ClientId {
    ClientId { client_type: '07-cometbft', sequence: 0 }
}

pub fn PORT_ID() -> PortId {
    PortId { port_id: "transfer" }
}

pub fn CHANNEL_ID() -> ChannelId {
    ChannelId { channel_id: "channel-0" }
}

pub fn SEQUENCE() -> Sequence {
    Sequence { sequence: 1 }
}

pub fn CHANNEL_END() -> ChannelEnd {
    ChannelEnd {
        state: ChannelState::Open,
        ordering: ChannelOrdering::Ordered,
        remote: Counterparty { port_id: PORT_ID(), channel_id: CHANNEL_ID(), },
        client_id: CLIENT_ID(),
    }
}
