use alexandria_data_structures::span_ext::SpanTraitExt;
use core::num::traits::Zero;
use ics23::ByteArrayIntoArrayU8;
use protobuf::primitives::array::{ByteArrayAsProtoMessage, BytesAsProtoMessage};
use protobuf::primitives::numeric::U128AsProtoMessage;
use protobuf::types::message::{
    DecodeContext, DecodeContextImpl, EncodeContext, EncodeContextImpl, ProtoCodecImpl,
    ProtoMessage, ProtoName,
};
use protobuf::types::tag::WireType;
use starknet_ibc_core::client::{Duration, DurationTrait};
use starknet_ibc_core::commitment::{StateValue, StateValueZero};
use starknet_ibc_core::connection::ConnectionErrors;
use starknet_ibc_core::host::{
    BasePrefix, BasePrefixTrait, BasePrefixZero, ClientId, ClientIdImpl, ClientIdTrait,
    ClientIdZero, ConnectionId, ConnectionIdZero,
};
use starknet_ibc_utils::ValidateBasic;

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct ConnectionEnd {
    pub state: ConnectionState,
    pub client_id: ClientId,
    pub counterparty: Counterparty,
    pub version: Version,
    pub delay_period: Duration,
}

impl ConnectionEndAsProtoMessage of ProtoMessage<ConnectionEnd> {
    fn encode_raw(self: @ConnectionEnd, ref context: EncodeContext) {
        let client_id_ba = self.client_id.to_byte_array();
        context.encode_field(1, @client_id_ba);
        context.encode_repeated_field(2, @array![self.version.clone()]);
        context.encode_enum(3, self.state);
        context.encode_field(4, self.counterparty);
        if self.delay_period.is_non_zero() {
            context.encode_field(5, @self.delay_period.as_nanos());
        }
    }

    fn decode_raw(ref context: DecodeContext) -> Option<ConnectionEnd> {
        // FIXME: Implement decode when required
        None
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl ConnectionEndAsProtoName of ProtoName<ConnectionEnd> {
    fn type_url() -> ByteArray {
        "ConnectionEnd"
    }
}

#[generate_trait]
pub impl ConnectionEndImpl of ConnectionEndTrait {
    fn new(
        state: ConnectionState,
        client_id: ClientId,
        counterparty_client_id: ClientId,
        counterparty_connection_id: ConnectionId,
        counterparty_prefix: BasePrefix,
        version: Version,
        delay_period: Duration,
    ) -> ConnectionEnd {
        ConnectionEnd {
            state,
            client_id,
            counterparty: Counterparty {
                client_id: counterparty_client_id,
                connection_id: counterparty_connection_id,
                prefix: counterparty_prefix,
            },
            version,
            delay_period,
        }
    }

    /// Initializes a new connection end.
    fn init(
        client_id: ClientId,
        counterparty_client_id: ClientId,
        counterparty_prefix: BasePrefix,
        delay_period: Duration,
    ) -> ConnectionEnd {
        Self::new(
            ConnectionState::Init,
            client_id,
            counterparty_client_id,
            ConnectionIdZero::zero(),
            counterparty_prefix,
            VersionImpl::supported(),
            delay_period,
        )
    }

    /// Creates a new connection end in the try open state.
    fn try_open(
        client_id: ClientId,
        counterparty_client_id: ClientId,
        counterparty_connection_id: ConnectionId,
        counterparty_prefix: BasePrefix,
        delay_period: Duration,
    ) -> ConnectionEnd {
        Self::new(
            ConnectionState::TryOpen,
            client_id,
            counterparty_client_id,
            counterparty_connection_id,
            counterparty_prefix,
            VersionImpl::supported(),
            delay_period,
        )
    }

    /// Creates a new connection end in the open state.
    fn open(
        client_id: ClientId,
        counterparty_client_id: ClientId,
        counterparty_connection_id: ConnectionId,
        counterparty_prefix: BasePrefix,
        version: Version,
        delay_period: Duration,
    ) -> ConnectionEnd {
        Self::new(
            ConnectionState::Open,
            client_id,
            counterparty_client_id,
            counterparty_connection_id,
            counterparty_prefix,
            version,
            delay_period,
        )
    }

    /// Consumes the connection end and returns a new connection end in the open state.
    fn to_open(self: ConnectionEnd) -> ConnectionEnd {
        ConnectionEnd {
            state: ConnectionState::Open,
            client_id: self.client_id,
            counterparty: self.counterparty,
            version: self.version,
            delay_period: self.delay_period,
        }
    }

    /// Opens the connection with the given counterparty connection ID and version.
    fn to_open_with_params(
        self: ConnectionEnd, counterparty_connection_id: ConnectionId, version: Version,
    ) -> ConnectionEnd {
        ConnectionEnd {
            state: ConnectionState::Open,
            client_id: self.client_id,
            counterparty: Counterparty {
                client_id: self.counterparty.client_id,
                connection_id: counterparty_connection_id,
                prefix: self.counterparty.prefix,
            },
            version,
            delay_period: self.delay_period,
        }
    }

    /// Returns the state of the connection end.
    fn state(self: @ConnectionEnd) -> @ConnectionState {
        self.state
    }

    /// Returns true if the connection is in the init state.
    fn is_init(self: @ConnectionEnd) -> bool {
        self.state == @ConnectionState::Init
    }

    /// Returns true if the connection is in the try open state.
    fn is_try_open(self: @ConnectionEnd) -> bool {
        self.state == @ConnectionState::TryOpen
    }

    /// Returns true if the connection is in the open state.
    fn is_open(self: @ConnectionEnd) -> bool {
        self.state == @ConnectionState::Open
    }

    /// Returns true if all the fields are in the zero state.
    fn is_zero(self: @ConnectionEnd) -> bool {
        self.state == @ConnectionState::Uninitialized
            && self.client_id.is_zero()
            && self.counterparty.is_zero()
            && self.version.is_zero()
            && self.delay_period.is_zero()
    }
}

pub impl ConnectionEndIntoStateValue of Into<ConnectionEnd, StateValue> {
    fn into(self: ConnectionEnd) -> StateValue {
        let encoded_connection_end = ProtoCodecImpl::encode(@self);
        StateValue { value: encoded_connection_end.into() }
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub enum ConnectionState {
    #[default]
    Uninitialized,
    Init,
    TryOpen,
    Open,
}

pub impl ConnectionStateIntoU32 of Into<@ConnectionState, u32> {
    fn into(self: @ConnectionState) -> u32 {
        match self {
            ConnectionState::Uninitialized => 0,
            ConnectionState::Init => 1,
            ConnectionState::TryOpen => 2,
            ConnectionState::Open => 3,
        }
    }
}

#[derive(Default, Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct Counterparty {
    pub client_id: ClientId,
    pub connection_id: ConnectionId,
    pub prefix: BasePrefix,
}

impl CounterpartyAsProtoMessage of ProtoMessage<Counterparty> {
    fn encode_raw(self: @Counterparty, ref context: EncodeContext) {
        let client_id_ba = self.client_id.to_byte_array();
        context.encode_field(1, @client_id_ba);

        context.encode_field(2, self.connection_id.connection_id);

        context.encode_field(3, self.prefix);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Counterparty> {
        // FIXME: Implement decode when required
        None
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl CounterpartyAsProtoName of ProtoName<Counterparty> {
    fn type_url() -> ByteArray {
        "Counterparty"
    }
}

pub impl CounterpartyValidateBasic of ValidateBasic<Counterparty> {
    fn validate_basic(self: @Counterparty) {
        assert(!self.client_id.is_zero(), ConnectionErrors::MISSING_CLIENT_ID);
        assert(self.connection_id.is_non_zero(), ConnectionErrors::MISSING_CONNECTION_ID);
    }
}

pub impl CounterpartyZero of Zero<Counterparty> {
    fn zero() -> Counterparty {
        Counterparty {
            client_id: ClientIdZero::zero(),
            connection_id: ConnectionIdZero::zero(),
            prefix: BasePrefixZero::zero(),
        }
    }

    fn is_zero(self: @Counterparty) -> bool {
        self.client_id.is_zero() && self.connection_id.is_zero() && self.prefix.is_zero()
    }

    fn is_non_zero(self: @Counterparty) -> bool {
        !self.is_zero()
    }
}

#[derive(Default, Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct Version {
    pub identifier: ByteArray,
    pub features: [ByteArray; 2],
}

impl VersionAsProtoMessage of ProtoMessage<Version> {
    fn encode_raw(self: @Version, ref context: EncodeContext) {
        context.encode_field(1, self.identifier);

        let [feature0, feature1] = self.features;
        let mut features_array: Array<ByteArray> = ArrayTrait::new();
        features_array.append(feature0.clone());
        features_array.append(feature1.clone());

        context.encode_repeated_field(2, @features_array);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Version> {
        // FIXME: Implement decode when required
        None
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl VersionAsProtoName of ProtoName<Version> {
    fn type_url() -> ByteArray {
        "Version"
    }
}

#[generate_trait]
pub impl VersionImpl of VersionTrait {
    fn supported() -> Version {
        Version { identifier: "1", features: ["ORDER_ORDERED", "ORDER_UNORDERED"] }
    }

    fn is_supported(self: @Version) -> bool {
        let features_span = self.features.span();
        self.identifier == @"1"
            && features_span.contains(@"ORDER_ORDERED")
            && features_span.contains(@"ORDER_UNORDERED")
    }

    fn is_feature_supported(self: @Version, feature: @ByteArray) -> bool {
        self.features.span().contains(feature)
    }
}

pub impl VersionZero of Zero<Version> {
    fn zero() -> Version {
        Version { identifier: "", features: ["", ""] }
    }

    fn is_zero(self: @Version) -> bool {
        let features_span = self.features.span();
        self.identifier.len() == 0 && features_span.at(0) == @"" && features_span.at(1) == @""
    }

    fn is_non_zero(self: @Version) -> bool {
        !self.is_zero()
    }
}
