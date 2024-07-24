use core::starknet::SyscallResultTrait;
use core::traits::TryInto;
use openzeppelin::tests::utils::deploy;
use starknet::ContractAddress;
use starknet::contract_address_const;
use starknet::syscalls::deploy_syscall;
use starknet::testing;
use starknet_ibc::apps::transfer::component::ICS20TransferComponent::TransferInternalTrait;
use starknet_ibc::apps::transfer::component::ICS20TransferComponent;
use starknet_ibc::apps::transfer::interface::ITransfer;
use starknet_ibc::apps::transfer::interface::{ITransferDispatcher, ITransferDispatcherTrait};
use starknet_ibc::contract::Transfer;
use starknet_ibc::tests::utils::{PUBKEY, TOKEN_NAME, SALT, OWNER, pubkey, owner};

type ComponentState = ICS20TransferComponent::ComponentState<Transfer::ContractState>;

fn component_state() -> ComponentState {
    ICS20TransferComponent::component_state_for_testing()
}

fn basic_setup() -> ComponentState {
    let mut state = component_state();
    testing::set_caller_address(owner());
    state.initializer();
    state
}

fn setup() -> ComponentState {
    let mut state = basic_setup();
    state.register_token(TOKEN_NAME, pubkey());
    state
}

#[test]
fn test_deploy() {
    let contract_address = deploy(Transfer::TEST_CLASS_HASH, array![]);
    ITransferDispatcher { contract_address }.register_token(TOKEN_NAME, owner());
}

#[test]
fn test_register_token() {
    setup();
}

#[test]
#[should_panic(expected: ('ICS20: token is already listed',))]
fn test_register_token_twice() {
    let mut state = setup();
    state.register_token(TOKEN_NAME, pubkey());
}

