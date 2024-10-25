use core::num::traits::Zero;
use starknet_ibc_core::channel::ChannelErrors;
use starknet_ibc_core::client::{Height, Timestamp, HeightPartialOrd, TimestampPartialOrd};
use starknet_ibc_core::host::{ClientId, ChannelId, PortId, Sequence};
use starknet_ibc_utils::ValidateBasic;

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
    /// Checks if the packet timeout is set.
    fn is_timeout_set(self: @Packet) -> bool {
        !(self.timeout_height_on_b.is_zero() && self.timeout_timestamp_on_b.is_zero())
    }

    /// Checks if the packet is not timed out, and throws an error if it is.
    fn verify_not_timed_out(self: @Packet, current_height: @Height, current_timestamp: @Timestamp) {
        assert(self.timeout_height_on_b > current_height, ChannelErrors::TIMED_OUT_PACKET);
        assert(self.timeout_timestamp_on_b > current_timestamp, ChannelErrors::TIMED_OUT_PACKET);
    }

    fn compute_commitment(self: @Packet) -> Array<u8> {
        array![]
    }
}

impl PacketValidateBasicImpl of ValidateBasic<Packet> {
    fn validate_basic(self: @Packet) {}
}

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
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
    /// Returns port ID on the counterparty chain
    fn counterparty_port_id(self: @ChannelEnd) -> @PortId {
        self.remote.port_id
    }

    /// Returns channel ID on the counterparty chain
    fn counterparty_channel_id(self: @ChannelEnd) -> @ChannelId {
        self.remote.channel_id
    }

    /// Returns true if the channel is in the open state.
    fn is_open(self: @ChannelEnd) -> bool {
        self.state == @ChannelState::Open
    }

    /// Returns true if the channel is of ordered kind.
    fn is_ordered(self: @ChannelEnd) -> bool {
        self.ordering == @ChannelOrdering::Ordered
    }

    /// Returns true if the counterparty matches the given counterparty.
    fn counterparty_matches(
        self: @ChannelEnd, counterparty_port_id: @PortId, counterparty_channel_id: @ChannelId
    ) -> bool {
        self.remote.port_id == counterparty_port_id
            && self.remote.channel_id == counterparty_channel_id
    }

    /// Validates the channel end be in the open state and the counterparty
    /// parameters match with the expected one.
    fn validate(
        self: @ChannelEnd, counterparty_port_id: @PortId, counterparty_chan_id: @ChannelId
    ) {
        assert(self.is_open(), ChannelErrors::INVALID_CHANNEL_STATE);
        assert(
            self.counterparty_matches(counterparty_port_id, counterparty_chan_id),
            ChannelErrors::INVALID_COUNTERPARTY
        );
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

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub enum ChannelOrdering {
    Unordered,
    Ordered,
}

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct Counterparty {
    pub port_id: PortId,
    pub channel_id: ChannelId,
}

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub enum Receipt {
    Ok
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct Acknowledgement {
    pub ack: Array<u8>,
}

pub impl ArrayU8IntoAcknowledgement of Into<Array<u8>, Acknowledgement> {
    fn into(self: Array<u8>) -> Acknowledgement {
        Acknowledgement { ack: self }
    }
}

#[generate_trait]
pub impl AcknowledgementImpl of AcknowledgementTrait {
    fn is_non_empty(self: @Acknowledgement) -> bool {
        self.ack.len() > 0
    }

    fn compute_commitment(self: @Acknowledgement) -> felt252 {
        ''
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub enum AckStatus {
    Success: Acknowledgement,
    Error: Acknowledgement,
}

#[generate_trait]
pub impl AckStatusImpl of AckStatusTrait {
    /// Constructs a new `AckStatus`.
    fn new(ack: Acknowledgement, expected_ack: @Acknowledgement) -> AckStatus {
        if @ack == expected_ack {
            AckStatus::Success(ack)
        } else {
            AckStatus::Error(ack)
        }
    }

    /// Returns true if the status is success.
    fn is_success(self: @AckStatus) -> bool {
        match self {
            AckStatus::Success(_) => true,
            _ => false,
        }
    }

    /// Returns true if the status is error.
    fn is_error(self: @AckStatus) -> bool {
        match self {
            AckStatus::Error(_) => true,
            _ => false,
        }
    }
}
