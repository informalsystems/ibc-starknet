use starknet_ibc_core::channel::ChannelErrors;
use starknet_ibc_core::client::{Height, Timestamp, HeightPartialOrd, TimestampPartialOrd};
use starknet_ibc_core::host::{ClientId, ChannelId, PortId, Sequence};
use starknet_ibc_utils::ValidateBasicTrait;

#[derive(Clone, Debug, Drop, Serde)]
pub struct Packet {
    pub seq_on_a: Sequence,
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub port_id_on_b: PortId,
    pub chan_id_on_b: ChannelId,
    pub data: Array<felt252>,
    pub timeout_height_on_b: Height,
    pub timeout_timestamp_on_b: Timestamp,
}

#[generate_trait]
pub impl PacketImpl of PacketTrait {
    /// Checks if the packet is not timed out, and throws an error if it is.
    fn check_timed_out(self: @Packet, current_height: @u64, current_timestamp: @u64) {
        assert(
            self.timeout_height_on_b.revision_height > current_height,
            ChannelErrors::TIMED_OUT_PACKET
        );
        assert(
            self.timeout_timestamp_on_b.timestamp > current_timestamp,
            ChannelErrors::TIMED_OUT_PACKET
        );
    }

    fn compute_packet_commitment(self: @Packet) -> Array<u8> {
        array![]
    }
}

impl PacketValidateBasicImpl of ValidateBasicTrait<Packet> {
    fn validate_basic(self: @Packet) {}
}

#[derive(Clone, Debug, Drop, Serde, starknet::Store)]
pub struct ChannelEnd {
    pub state: ChannelState,
    pub ordering: ChannelOrdering,
    pub remote: Counterparty,
    // TODO: we currently peer each channel end with a client ID, but later once
    // we decided which IBC protocol to go with, either the current specs,
    // Eureka or something else, this part should be updated.
    pub client_id: ClientId,
}

#[generate_trait]
pub impl ChannelEndImpl of ChannelEndTrait {
    /// Returns true if the channel is in the open state.
    fn is_open(self: @ChannelEnd) -> bool {
        self.state == @ChannelState::Open
    }

    /// Returns true if the counterparty matches the given counterparty.
    fn counterparty_matches(
        self: @ChannelEnd, counterparty_port_id: @PortId, counterparty_chan_id: @ChannelId
    ) -> bool {
        self.remote.port_id == counterparty_port_id && self.remote.chan_id == counterparty_chan_id
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub enum ChannelState {
    Uninitialized,
    Init,
    TryOpen,
    Open,
    Closed,
}

#[derive(Clone, Debug, Drop, Serde, starknet::Store)]
pub enum ChannelOrdering {
    Unordered,
    Ordered,
}

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct Counterparty {
    pub port_id: PortId,
    pub chan_id: ChannelId,
}

#[derive(Clone, Debug, Drop, Serde)]
pub struct Acknowledgement {
    pub ack: felt252,
}


#[derive(Clone, Debug, Drop, Serde, starknet::Store)]
pub enum Receipt {
    Ok
}
