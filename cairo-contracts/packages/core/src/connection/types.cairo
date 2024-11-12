use alexandria_data_structures::span_ext::SpanTraitExt;
use core::num::traits::Zero;
use starknet_ibc_core::host::{ClientId, ConnectionId, ConnectionIdZero, PathPrefix};
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

    fn state(self: @ConnectionEnd) -> @ConnectionState {
        self.state
    }

    fn is_zero(self: @ConnectionEnd) -> bool {
        self.state == @ConnectionState::Uninitialized
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
    fn validate_basic(self: @Counterparty) {}
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
