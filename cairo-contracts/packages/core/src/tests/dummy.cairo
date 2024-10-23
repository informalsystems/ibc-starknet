use starknet::{contract_address_const, ContractAddress};
use starknet_ibc_core::channel::{ChannelEnd, ChannelState, ChannelOrdering, Counterparty};
use starknet_ibc_core::client::Height;
use starknet_ibc_core::host::{ClientId, PortId, ChannelId, Sequence};

pub fn HEIGHT(revision_height: u64) -> Height {
    Height { revision_number: 0, revision_height }
}

pub fn CLIENT() -> ContractAddress {
    contract_address_const::<'COMETBFT'>()
}

pub fn CLIENT_TYPE() -> felt252 {
    '07-cometbft'
}

pub fn CLIENT_ID() -> ClientId {
    ClientId { client_type: CLIENT_TYPE(), sequence: 0 }
}

pub fn PORT_ID() -> PortId {
    PortId { port_id: "transfer" }
}

pub fn CHANNEL_ID(sequence: u64) -> ChannelId {
    ChannelId { channel_id: format!("channel-{sequence}") }
}

pub fn SEQUENCE(sequence: u64) -> Sequence {
    Sequence { sequence }
}

pub fn CHANNEL_END(counterparty_channel_sequence: u64) -> ChannelEnd {
    ChannelEnd {
        state: ChannelState::Open,
        ordering: ChannelOrdering::Unordered,
        remote: Counterparty {
            port_id: PORT_ID(), channel_id: CHANNEL_ID(counterparty_channel_sequence),
        },
        client_id: CLIENT_ID(),
    }
}
