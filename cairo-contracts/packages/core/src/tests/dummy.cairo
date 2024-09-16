use starknet_ibc_core::channel::{ChannelEnd, ChannelState, ChannelOrdering, Counterparty};
use starknet_ibc_core::host::{ClientId, PortId, ChannelId, Sequence};

pub fn CLIENT_ID() -> ClientId {
    ClientId { client_type: '07-cometbft', sequence: 0 }
}

pub fn PORT_ID() -> PortId {
    PortId { port_id: "transfer" }
}

pub fn CHANNEL_ID(index: u64) -> ChannelId {
    ChannelId { channel_id: format!("channel-{index}") }
}

pub fn SEQUENCE(sequence: u64) -> Sequence {
    Sequence { sequence }
}

pub fn CHANNEL_END() -> ChannelEnd {
    ChannelEnd {
        state: ChannelState::Open,
        ordering: ChannelOrdering::Unordered,
        remote: Counterparty { port_id: PORT_ID(), channel_id: CHANNEL_ID(0), },
        client_id: CLIENT_ID(),
    }
}
