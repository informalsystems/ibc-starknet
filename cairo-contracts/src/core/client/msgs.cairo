use starknet_ibc::core::host::ClientId;

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgCreateClient {
    pub client_type: felt252,
    pub client_state: Array<felt252>,
    pub consensus_state: Array<felt252>,
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgUpdateClient {
    pub client_id: ClientId,
    pub client_message: Array<felt252>,
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgRecoverClient {
    pub subject_client_id: ClientId,
    pub substitute_client_id: ClientId,
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgUpgradeClient {
    pub client_id: ClientId,
}
