use core::num::traits::zero::Zero;
use starknet::{BlockInfo, get_block_info};
use starknet_ibc_core::client::{ClientErrors, StarknetClientState, StarknetConsensusState};
use starknet_ibc_core::commitment::StateProof;
use starknet_ibc_core::host::ClientId;
use starknet_ibc_utils::ValidateBasic;

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgCreateClient {
    pub client_type: felt252,
    pub client_state: Array<felt252>,
    pub consensus_state: Array<felt252>,
}

impl MsgCreateClientValidateBasic of ValidateBasic<MsgCreateClient> {
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

impl MsgUpdateClientValidateBasic of ValidateBasic<MsgUpdateClient> {
    fn validate_basic(self: @MsgUpdateClient) {
        assert(!self.client_message.is_empty(), ClientErrors::EMPTY_CLIENT_MESSAGE);
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgRecoverClient {
    pub subject_client_id: ClientId,
    pub substitute_client_id: ClientId,
}

impl MsgRecoverClientValidateBasic of ValidateBasic<MsgRecoverClient> {
    fn validate_basic(self: @MsgRecoverClient) {
        assert(
            self.subject_client_id.client_type == self.substitute_client_id.client_type,
            ClientErrors::INVALID_SUBSTITUTE_CLIENT_ID,
        );
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgUpgradeClient {
    pub client_id: ClientId,
    pub upgraded_client_state: Array<felt252>,
    pub upgraded_consensus_state: Array<felt252>,
    pub proof_upgrade_client: StateProof,
    pub proof_upgrade_consensus: StateProof,
}

impl MsgUpgradeClientValidateBasic of ValidateBasic<MsgUpgradeClient> {
    fn validate_basic(self: @MsgUpgradeClient) {}
}

#[derive(Clone, Debug, Drop, Serde)]
pub struct MsgScheduleUpgrade {
    pub upgraded_client_state: StarknetClientState,
    pub upgraded_consensus_state: StarknetConsensusState,
}

impl MsgScheduleUpgradeValidateBasic of ValidateBasic<MsgScheduleUpgrade> {
    fn validate_basic(self: @MsgScheduleUpgrade) {
        let BlockInfo { block_number, block_timestamp, .. } = get_block_info().unbox();

        assert(
            @block_number < self.upgraded_client_state.latest_height,
            ClientErrors::UPGRADE_HEIGHT_IN_PAST,
        );

        assert(
            @block_timestamp < self.upgraded_consensus_state.time,
            ClientErrors::UPGRADE_TIMESTAMP_IN_PAST,
        );

        assert(
            self.upgraded_consensus_state.root.is_zero(), ClientErrors::UPGRADE_ROOT_IS_NON_ZERO,
        );
    }
}
