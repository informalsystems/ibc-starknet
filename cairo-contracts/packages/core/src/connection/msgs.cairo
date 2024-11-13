use core::num::traits::Zero;
use starknet_ibc_core::client::Height;
use starknet_ibc_core::commitment::StateProof;
use starknet_ibc_core::connection::ConnectionErrors;
use starknet_ibc_core::connection::{Counterparty, Version};
use starknet_ibc_core::host::{ClientId, ClientIdImpl, ConnectionId, PathPrefix};
use starknet_ibc_utils::ValidateBasic;
use super::VersionTrait;

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgConnOpenInit {
    pub client_id_on_a: ClientId,
    pub counterparty: Counterparty,
    pub version: Version,
    pub delay_period: u64,
}

#[generate_trait]
pub impl MsgConnOpenInitImpl of MsgConnOpenInitTrait {
    fn client_type_on_a(self: @MsgConnOpenInit) -> @felt252 {
        self.client_id_on_a.client_type
    }

    fn counterparty_client_id(self: @MsgConnOpenInit) -> @ClientId {
        self.counterparty.client_id
    }

    fn counterparty_connection_id(self: @MsgConnOpenInit) -> @ConnectionId {
        self.counterparty.connection_id
    }

    fn counterparty_prefix(self: @MsgConnOpenInit) -> @PathPrefix {
        self.counterparty.prefix
    }
}

impl MsgConnOpenInitValidateBasic of ValidateBasic<MsgConnOpenInit> {
    fn validate_basic(self: @MsgConnOpenInit) {
        self.counterparty.validate_basic();
        if self.version.is_non_zero() {
            assert(self.version.is_supported(), ConnectionErrors::UNSUPPORTED_VERSION);
        }
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgConnOpenTry {
    pub client_id_on_b: ClientId,
    pub counterparty: Counterparty,
    pub version_on_a: Version,
    pub proof_conn_end_on_a: StateProof,
    pub proof_height_on_a: Height,
    pub delay_period: u64,
}

#[generate_trait]
pub impl MsgConnOpenTryImpl of MsgConnOpenTryTrait {
    fn client_type_on_b(self: @MsgConnOpenTry) -> @felt252 {
        self.client_id_on_b.client_type
    }

    fn counterparty_client_id(self: @MsgConnOpenTry) -> @ClientId {
        self.counterparty.client_id
    }

    fn counterparty_connection_id(self: @MsgConnOpenTry) -> @ConnectionId {
        self.counterparty.connection_id
    }

    fn counterparty_prefix(self: @MsgConnOpenTry) -> @PathPrefix {
        self.counterparty.prefix
    }
}

impl MsgConnOpenTryValidateBasic of ValidateBasic<MsgConnOpenTry> {
    fn validate_basic(self: @MsgConnOpenTry) {
        assert(!self.client_id_on_b.is_zero(), ConnectionErrors::MISSING_CLIENT_ID);
        self.counterparty.validate_basic();
        if self.version_on_a.is_non_zero() {
            assert(self.version_on_a.is_supported(), ConnectionErrors::UNSUPPORTED_VERSION);
        }
        assert(self.proof_conn_end_on_a.is_non_zero(), ConnectionErrors::EMPTY_CONN_END_PROOF);
        assert(self.proof_height_on_a.is_non_zero(), ConnectionErrors::ZERO_PROOF_HEIGHT);
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgConnOpenAck {}

impl MsgConnOpenAckValidateBasic of ValidateBasic<MsgConnOpenAck> {
    fn validate_basic(self: @MsgConnOpenAck) {}
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgConnOpenConfirm {}

impl MsgConnOpenConfirmValidateBasic of ValidateBasic<MsgConnOpenConfirm> {
    fn validate_basic(self: @MsgConnOpenConfirm) {}
}
