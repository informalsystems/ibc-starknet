use core::num::traits::Zero;
use starknet_ibc_core::connection::{Counterparty, Version};
use starknet_ibc_core::host::{ClientId, ConnectionId, PathPrefix};
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
            self.version.is_supported();
        }
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgConnOpenTry {}

impl MsgConnOpenTryValidateBasic of ValidateBasic<MsgConnOpenTry> {
    fn validate_basic(self: @MsgConnOpenTry) {}
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
