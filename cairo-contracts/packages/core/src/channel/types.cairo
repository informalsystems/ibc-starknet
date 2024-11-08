use core::num::traits::Zero;
use starknet_ibc_core::channel::ChannelErrors;
use starknet_ibc_core::client::{Height, Timestamp, HeightPartialOrd, TimestampPartialOrd};
use starknet_ibc_core::commitment::{array_u8_into_array_u32, IntoArrayU32};
use starknet_ibc_core::host::{ClientId, ClientIdTrait, ChannelId, PortId, PortIdTrait, Sequence};
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

    /// Checks if the packet is timed out.
    fn is_timed_out(self: @Packet, latest_height: @Height, latest_timestamp: @Timestamp) -> bool {
        !(self.timeout_height_on_b > latest_height
            && self.timeout_timestamp_on_b > latest_timestamp)
    }
}

impl PacketValidateBasic of ValidateBasic<Packet> {
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
    fn new(
        state: ChannelState,
        ordering: ChannelOrdering,
        counterparty_port_id: PortId,
        counterparty_channel_id: ChannelId,
        client_id: ClientId,
    ) -> ChannelEnd {
        ChannelEnd {
            state,
            ordering,
            remote: CounterpartyImpl::new(counterparty_port_id, counterparty_channel_id),
            client_id,
        }
    }

    /// Returns port ID on the counterparty chain
    fn counterparty_port_id(self: @ChannelEnd) -> @PortId {
        self.remote.port_id
    }

    /// Returns channel ID on the counterparty chain
    fn counterparty_channel_id(self: @ChannelEnd) -> @ChannelId {
        self.remote.channel_id
    }

    /// Returns the state of the channel end.
    fn state(self: @ChannelEnd) -> @ChannelState {
        self.state
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

    /// Returns true if all the fields are in the zero state.
    fn is_zero(self: @ChannelEnd) -> bool {
        self.state == @ChannelState::Uninitialized
            && self.ordering == @ChannelOrdering::Unordered
            && self.remote.is_zero()
            && self.client_id.is_zero()
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

    /// Consumes the channel end and returns a new channel end with the state
    /// set to closed.
    fn close(self: ChannelEnd) -> ChannelEnd {
        ChannelEnd {
            state: ChannelState::Closed,
            ordering: self.ordering,
            remote: self.remote,
            client_id: self.client_id,
        }
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub enum ChannelState {
    #[default]
    Uninitialized,
    Init,
    TryOpen,
    Open,
    Closed,
}

#[derive(Copy, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub enum ChannelOrdering {
    #[default]
    Unordered,
    Ordered,
}

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct AppVersion {
    pub version: ByteArray,
}

pub impl AppVersionZero of Zero<AppVersion> {
    fn zero() -> AppVersion {
        AppVersion { version: "" }
    }

    fn is_zero(self: @AppVersion) -> bool {
        self.version.len() == 0
    }

    fn is_non_zero(self: @AppVersion) -> bool {
        !self.is_zero()
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct Counterparty {
    pub port_id: PortId,
    pub channel_id: ChannelId,
}

#[generate_trait]
pub impl CounterpartyImpl of CounterpartyTrait {
    fn new(port_id: PortId, channel_id: ChannelId) -> Counterparty {
        Counterparty { port_id, channel_id, }
    }

    fn is_zero(self: @Counterparty) -> bool {
        self.port_id.is_zero() && self.channel_id.is_zero()
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub enum Receipt {
    #[default]
    None,
    Ok
}

#[generate_trait]
pub impl ReceiptImpl of ReceiptTrait {
    fn is_ok(self: @Receipt) -> bool {
        self == @Receipt::Ok
    }

    fn is_none(self: @Receipt) -> bool {
        self == @Receipt::None
    }
}

pub impl ReceiptZero of Zero<Receipt> {
    fn is_zero(self: @Receipt) -> bool {
        self == @Receipt::None
    }

    fn is_non_zero(self: @Receipt) -> bool {
        !self.is_zero()
    }

    fn zero() -> Receipt {
        Receipt::None
    }
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

pub impl AcknowledgementIntoArrayU32 of IntoArrayU32<Acknowledgement> {
    fn into_array_u32(self: Acknowledgement) -> (Array<u32>, u32, u32) {
        array_u8_into_array_u32(self.ack)
    }
}

pub impl AcknowledegementZero of Zero<Acknowledgement> {
    fn zero() -> Acknowledgement {
        Acknowledgement { ack: ArrayTrait::new() }
    }

    fn is_zero(self: @Acknowledgement) -> bool {
        self.ack.len() == 0
    }

    fn is_non_zero(self: @Acknowledgement) -> bool {
        self.ack.len() > 0
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

    /// Returns the acknowledgement.
    fn ack(self: @AckStatus) -> @Acknowledgement {
        match self {
            AckStatus::Success(ack) => ack,
            AckStatus::Error(ack) => ack,
        }
    }

    /// Returns true if the acknowledgement is non-empty.
    fn is_non_empty(self: @AckStatus) -> bool {
        self.ack().is_non_zero()
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

