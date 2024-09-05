use starknet::ContractAddress;
use starknet_ibc_core::client::{
    MsgCreateClient, MsgUpdateClient, MsgRecoverClient, MsgUpgradeClient, Height, Status,
    CreateResponse, UpdateResponse
};

#[starknet::interface]
pub trait IClientHandler<TContractState> {
    fn create_client(ref self: TContractState, msg: MsgCreateClient) -> CreateResponse;

    fn update_client(ref self: TContractState, msg: MsgUpdateClient) -> UpdateResponse;

    fn recover_client(ref self: TContractState, msg: MsgRecoverClient);

    fn upgrade_client(ref self: TContractState, msg: MsgUpgradeClient);
}

#[starknet::interface]
pub trait IRegisterClient<TContractState> {
    fn register_client(
        ref self: TContractState, client_type: felt252, client_address: ContractAddress
    );
}

#[starknet::interface]
pub trait IClientState<TContractState> {
    fn client_type(self: @TContractState) -> felt252;

    fn latest_height(self: @TContractState, client_sequence: u64) -> Height;

    fn status(self: @TContractState, client_sequence: u64) -> Status;
}

#[starknet::interface]
pub trait IClientStateValidation<TContractState> {
    fn verify_client_message(
        self: @TContractState, client_sequence: u64, client_message: Array<felt252>
    );

    fn verify_misbehaviour(
        self: @TContractState, client_sequence: u64, client_message: Array<felt252>
    ) -> bool;

    fn verify_substitute(self: @TContractState, substitute_client_state: Array<felt252>);

    fn verify_upgrade(
        self: @TContractState,
        upgrade_client_state: Array<felt252>,
        upgrade_consensus_state: Array<felt252>,
        proof_upgrade_client: Array<felt252>,
        proof_upgrade_consensus: Array<felt252>,
        root: felt252,
    );
}

#[starknet::interface]
pub trait IClientStateExecution<TContractState> {
    fn initialize(
        ref self: TContractState,
        client_sequence: u64,
        client_state: Array<felt252>,
        consensus_state: Array<felt252>
    ) -> CreateResponse;

    fn update_state(
        ref self: TContractState, client_sequence: u64, client_message: Array<felt252>,
    ) -> UpdateResponse;

    fn update_on_misbehaviour(
        ref self: TContractState, client_sequence: u64, client_message: Array<felt252>,
    ) -> UpdateResponse;

    fn update_on_recover(
        ref self: TContractState,
        subject_client_sequence: u64,
        substitute_client_state: Array<felt252>,
        substitute_consensus_state: Array<felt252>,
    );

    fn update_on_upgrade(
        ref self: TContractState,
        client_sequence: u64,
        new_client_state: Array<felt252>,
        new_consensus_state: Array<felt252>,
    );
}
