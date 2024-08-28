use core::num::traits::zero::Zero;
use starknet_ibc_core_client::ClientErrors;
use starknet_ibc_core_host::ClientId;
use starknet_ibc_utils::ValidateBasicTrait;

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgCreateClient {
    pub client_type: felt252,
    pub client_state: Array<felt252>,
    pub consensus_state: Array<felt252>,
}

impl MsgCreateClientValidateBasicImpl of ValidateBasicTrait<MsgCreateClient> {
    fn validate_basic(self: @MsgCreateClient) {
        assert(self.client_type.is_non_zero(), ClientErrors::ZERO_CLIENT_TYPE);

        assert(!self.client_state.is_empty(), ClientErrors::EMPTY_CLIENT_STATE);

        assert(!self.consensus_state.is_empty(), ClientErrors::EMPTY_CONSENSUS_STATE);
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgUpdateClient {
    pub client_id: ClientId,
    pub client_message: Array<felt252>,
}

impl MsgUpdateClientValidateBasicImpl of ValidateBasicTrait<MsgUpdateClient> {
    fn validate_basic(self: @MsgUpdateClient) {
        assert(!self.client_message.is_empty(), ClientErrors::EMPTY_CLIENT_MESSAGE);
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgRecoverClient {
    pub subject_client_id: ClientId,
    pub substitute_client_id: ClientId,
}

impl MsgRecoverClientValidateBasicImpl of ValidateBasicTrait<MsgRecoverClient> {
    fn validate_basic(self: @MsgRecoverClient) {
        assert(
            self.subject_client_id.client_type == self.substitute_client_id.client_type,
            ClientErrors::INVALID_SUBSTITUTE_CLIENT_ID
        );
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgUpgradeClient {
    pub client_id: ClientId,
}

impl MsgUpgradeClientValidateBasicImpl of ValidateBasicTrait<MsgUpgradeClient> {
    fn validate_basic(self: @MsgUpgradeClient) {}
}
