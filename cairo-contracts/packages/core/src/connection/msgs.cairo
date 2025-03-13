use core::num::traits::Zero;
use starknet_ibc_core::client::{Duration, Height};
use starknet_ibc_core::commitment::StateProof;
use starknet_ibc_core::connection::{ConnectionErrors, Version};
use starknet_ibc_core::host::{BasePrefix, ClientId, ClientIdImpl, ConnectionId};
use starknet_ibc_utils::ValidateBasic;
use super::VersionTrait;

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgConnOpenInit {
    pub client_id_on_a: ClientId,
    pub client_id_on_b: ClientId,
    pub prefix_on_b: BasePrefix,
    pub version: Version,
    pub delay_period: Duration,
}

#[generate_trait]
pub impl MsgConnOpenInitImpl of MsgConnOpenInitTrait {
    fn client_type_on_a(self: @MsgConnOpenInit) -> @felt252 {
        self.client_id_on_a.client_type
    }
}

impl MsgConnOpenInitValidateBasic of ValidateBasic<MsgConnOpenInit> {
    fn validate_basic(self: @MsgConnOpenInit) {
        assert(!self.client_id_on_a.is_zero(), ConnectionErrors::MISSING_CLIENT_ID);
        assert(!self.client_id_on_b.is_zero(), ConnectionErrors::MISSING_CLIENT_ID);
        if self.version.is_non_zero() {
            assert(self.version.is_supported(), ConnectionErrors::UNSUPPORTED_VERSION);
        }
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgConnOpenTry {
    pub client_id_on_b: ClientId,
    pub client_id_on_a: ClientId,
    pub conn_id_on_a: ConnectionId,
    pub prefix_on_a: BasePrefix,
    pub version_on_a: Version,
    pub proof_conn_end_on_a: StateProof,
    pub proof_height_on_a: Height,
    pub delay_period: Duration,
}

#[generate_trait]
pub impl MsgConnOpenTryImpl of MsgConnOpenTryTrait {
    fn client_type_on_b(self: @MsgConnOpenTry) -> @felt252 {
        self.client_id_on_b.client_type
    }
}

impl MsgConnOpenTryValidateBasic of ValidateBasic<MsgConnOpenTry> {
    fn validate_basic(self: @MsgConnOpenTry) {
        assert(!self.client_id_on_b.is_zero(), ConnectionErrors::MISSING_CLIENT_ID);
        assert(!self.client_id_on_a.is_zero(), ConnectionErrors::MISSING_CLIENT_ID);
        assert(!self.conn_id_on_a.is_zero(), ConnectionErrors::MISSING_CONNECTION_ID);
        if self.version_on_a.is_non_zero() {
            assert(self.version_on_a.is_supported(), ConnectionErrors::UNSUPPORTED_VERSION);
        }
        assert(self.proof_conn_end_on_a.is_non_zero(), ConnectionErrors::EMPTY_CONN_END_PROOF);
        assert(self.proof_height_on_a.is_non_zero(), ConnectionErrors::ZERO_PROOF_HEIGHT);
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgConnOpenAck {
    pub conn_id_on_a: ConnectionId,
    pub conn_id_on_b: ConnectionId,
    pub proof_conn_end_on_b: StateProof,
    pub proof_height_on_b: Height,
    pub version: Version,
}

pub impl MsgConnOpenAckValidateBasic of ValidateBasic<MsgConnOpenAck> {
    fn validate_basic(self: @MsgConnOpenAck) {
        assert(!self.conn_id_on_a.is_zero(), ConnectionErrors::MISSING_CONNECTION_ID);
        assert(!self.conn_id_on_b.is_zero(), ConnectionErrors::MISSING_CONNECTION_ID);
        assert(self.proof_conn_end_on_b.is_non_zero(), ConnectionErrors::EMPTY_CONN_END_PROOF);
        assert(self.proof_height_on_b.is_non_zero(), ConnectionErrors::ZERO_PROOF_HEIGHT);
        assert(self.version.is_supported(), ConnectionErrors::UNSUPPORTED_VERSION);
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgConnOpenConfirm {
    pub conn_id_on_b: ConnectionId,
    pub proof_conn_end_on_a: StateProof,
    pub proof_height_on_a: Height,
}

impl MsgConnOpenConfirmValidateBasic of ValidateBasic<MsgConnOpenConfirm> {
    fn validate_basic(self: @MsgConnOpenConfirm) {
        assert(!self.conn_id_on_b.is_zero(), ConnectionErrors::MISSING_CONNECTION_ID);
        assert(self.proof_conn_end_on_a.is_non_zero(), ConnectionErrors::EMPTY_CONN_END_PROOF);
        assert(self.proof_height_on_a.is_non_zero(), ConnectionErrors::ZERO_PROOF_HEIGHT);
    }
}
