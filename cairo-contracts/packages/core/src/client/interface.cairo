use starknet::ContractAddress;
use starknet_ibc_core::client::{
    CreateResponse, Height, MsgCreateClient, MsgRecoverClient, MsgUpdateClient, MsgUpgradeClient,
    Status, Timestamp, UpdateResponse,
};
use starknet_ibc_core::commitment::{StateProof, StateRoot, StateValue};

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
        ref self: TContractState, client_type: felt252, client_address: ContractAddress,
    );
}

#[starknet::interface]
pub trait IRegisterRelayer<TContractState> {
    fn register_relayer(ref self: TContractState, relayer_address: ContractAddress);
}

#[starknet::interface]
pub trait IClientStateValidation<TContractState> {
    fn verify_membership(
        self: @TContractState,
        client_sequence: u64,
        paths: Array<ByteArray>,
        value: StateValue,
        proof: StateProof,
        root: StateRoot,
    );

    fn verify_non_membership(
        self: @TContractState,
        client_sequence: u64,
        paths: Array<ByteArray>,
        proof: StateProof,
        root: StateRoot,
    );

    fn verify_client_message(
        self: @TContractState, client_sequence: u64, client_message: Array<felt252>,
    );

    fn verify_misbehaviour(
        self: @TContractState, client_sequence: u64, client_message: Array<felt252>,
    ) -> bool;

    fn verify_substitute(self: @TContractState, substitute_client_state: Array<felt252>);

    fn verify_upgrade(
        self: @TContractState,
        upgrade_client_state: Array<felt252>,
        upgrade_consensus_state: Array<felt252>,
        proof_upgrade_client: Array<felt252>,
        proof_upgrade_consensus: Array<felt252>,
        root: ByteArray,
    );
}

#[starknet::interface]
pub trait IClientStateExecution<TContractState> {
    fn initialize(
        ref self: TContractState,
        client_sequence: u64,
        client_state: Array<felt252>,
        consensus_state: Array<felt252>,
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
        substitute_client_sequence: u64,
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

#[starknet::interface]
pub trait IClientQuery<TContractState> {
    fn client_type(self: @TContractState) -> felt252;

    fn latest_height(self: @TContractState, client_sequence: u64) -> Height;

    /// Returns the latest update height that is less than or equal to the
    /// target height.
    fn update_height_before(
        self: @TContractState, client_sequence: u64, target_height: Height,
    ) -> Option<Height>;

    fn update_height_after(
        self: @TContractState, client_sequence: u64, target_height: Height,
    ) -> Option<Height>;

    fn latest_timestamp(self: @TContractState, client_sequence: u64) -> Timestamp;

    fn status(self: @TContractState, client_sequence: u64) -> Status;

    fn client_state(self: @TContractState, client_sequence: u64) -> Array<felt252>;

    fn consensus_state(
        self: @TContractState, client_sequence: u64, height: Height,
    ) -> Array<felt252>;

    fn consensus_state_root(
        self: @TContractState, client_sequence: u64, height: Height,
    ) -> StateRoot;
}
