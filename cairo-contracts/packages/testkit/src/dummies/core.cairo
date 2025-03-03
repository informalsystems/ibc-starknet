use starknet::{ContractAddress, contract_address_const};
use starknet_ibc_core::channel::{
    AppVersion, ChannelEnd, ChannelOrdering, ChannelState, Counterparty as ChanCounterparty,
};
use starknet_ibc_core::client::{Height, Timestamp};
use starknet_ibc_core::commitment::{StateProof, StateRoot};
use starknet_ibc_core::connection::{
    ConnectionEnd, ConnectionState, Counterparty as ConnCounterparty, VersionImpl,
};
use starknet_ibc_core::host::{BasePrefix, ChannelId, ClientId, ConnectionId, PortId, Sequence};

pub fn HEIGHT(revision_height: u64) -> Height {
    Height { revision_number: 0, revision_height }
}

pub fn TIMESTAMP(timestamp: u64) -> Timestamp {
    Timestamp { timestamp }
}

pub fn TIMEOUT_HEIGHT(height: u64) -> Height {
    HEIGHT(height)
}

pub fn TIMEOUT_TIMESTAMP(timestamp: u64) -> Timestamp {
    TIMESTAMP(timestamp)
}

pub fn RELAYER() -> ContractAddress {
    contract_address_const::<'RELAYER'>()
}

pub fn CLIENT() -> ContractAddress {
    contract_address_const::<'COMETBFT'>()
}

pub fn CLIENT_TYPE() -> felt252 {
    '07-tendermint'
}

pub fn CLIENT_ID() -> ClientId {
    ClientId { client_type: CLIENT_TYPE(), sequence: 0 }
}

pub fn CONNECTION_ID(sequence: u64) -> ConnectionId {
    ConnectionId { connection_id: format!("connection-{sequence}") }
}

pub fn CONNECTION_END(counterparty_connection_sequence: u64) -> ConnectionEnd {
    ConnectionEnd {
        state: ConnectionState::Open,
        client_id: CLIENT_ID(),
        counterparty: ConnCounterparty {
            client_id: CLIENT_ID(),
            connection_id: CONNECTION_ID(counterparty_connection_sequence),
            prefix: BasePrefix { prefix: "" },
        },
        version: VersionImpl::supported(),
        delay_period: 0,
    }
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
        remote: ChanCounterparty {
            port_id: PORT_ID(), channel_id: CHANNEL_ID(counterparty_channel_sequence),
        },
        connection_id: CONNECTION_ID(0),
        version: VERSION_PROPOSAL(),
    }
}

pub fn VERSION_PROPOSAL() -> AppVersion {
    AppVersion { version: "" }
}

pub fn STATE_PROOF() -> StateProof {
    StateProof { proof: array![1] }
}

pub fn STATE_ROOT() -> StateRoot {
    StateRoot { root: "1" }
}

pub fn IBC_PREFIX() -> BasePrefix {
    BasePrefix { prefix: "Ibc" }
}
