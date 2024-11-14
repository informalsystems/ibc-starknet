use alexandria_data_structures::span_ext::SpanTraitExt;
use core::num::traits::Zero;
use starknet_ibc_core::commitment::{StateValue, StateValueZero};
use starknet_ibc_core::connection::ConnectionErrors;
use starknet_ibc_core::host::{
    ClientId, ClientIdImpl, ClientIdZero, ConnectionId, ConnectionIdZero, PathPrefix, PathPrefixZero
};
use starknet_ibc_utils::ValidateBasic;

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct ConnectionEnd {
    pub state: ConnectionState,
    pub client_id: ClientId,
    pub counterparty: Counterparty,
    pub version: Version,
    pub delay_period: u64,
}

#[generate_trait]
pub impl ConnectionEndImpl of ConnectionEndTrait {
    fn new(
        state: ConnectionState,
        client_id: ClientId,
        counterparty_client_id: ClientId,
        counterparty_connection_id: ConnectionId,
        counterparty_prefix: PathPrefix,
        version: Version,
        delay_period: u64,
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
        counterparty_prefix: PathPrefix,
        delay_period: u64,
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
        counterparty_prefix: PathPrefix,
        delay_period: u64,
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
        counterparty_prefix: PathPrefix,
        version: Version,
        delay_period: u64,
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

    /// Returns true if all the fields are in the zero state.
    fn is_zero(self: @ConnectionEnd) -> bool {
        self.state == @ConnectionState::Uninitialized
            && self.client_id.is_zero()
            && self.counterparty.is_zero()
            && self.version.is_zero()
            && self.delay_period == @0
    }
}

pub impl ConnectionEndIntoStateValue of Into<ConnectionEnd, StateValue> {
    fn into(self: ConnectionEnd) -> StateValue {
        // TODO: Implement once membership proof verification is implemented.
        StateValueZero::zero()
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

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct Counterparty {
    pub client_id: ClientId,
    pub connection_id: ConnectionId,
    pub prefix: PathPrefix,
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
            prefix: PathPrefixZero::zero(),
        }
    }

    fn is_zero(self: @Counterparty) -> bool {
        self.client_id.is_zero() && self.connection_id.is_zero() && self.prefix.is_zero()
    }

    fn is_non_zero(self: @Counterparty) -> bool {
        !self.is_zero()
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct Version {
    pub identifier: ByteArray,
    pub features: [ByteArray; 2],
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
