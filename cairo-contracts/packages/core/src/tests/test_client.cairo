use ClientHandlerComponent::ClientReaderTrait;
use core::num::traits::Zero;
use snforge_std::{spy_events, test_address};
use starknet_ibc_core::client::ClientHandlerComponent::{
    ClientInitializerImpl, CoreRegisterClientImpl, CoreClientHandlerImpl, EventEmitterImpl,
    ClientInternalImpl
};
use starknet_ibc_core::client::{ClientHandlerComponent, CreateResponse};
use starknet_ibc_core::tests::{
    CLIENT, CLIENT_TYPE, CLIENT_ID, HEIGHT, ClientEventSpyExt, MockClientHandler
};

type ComponentState = ClientHandlerComponent::ComponentState<MockClientHandler::ContractState>;

fn COMPONENT_STATE() -> ComponentState {
    ClientHandlerComponent::component_state_for_testing()
}

fn setup() -> ComponentState {
    let mut state = COMPONENT_STATE();
    state.initializer();
    state
}

#[test]
fn test_intial_state() {
    let state = setup();
    let supported_client = state.read_supported_client(0);
    assert!(supported_client.is_zero());
}

#[test]
fn test_register_client() {
    let mut state = setup();
    state.register_client(CLIENT_TYPE(), CLIENT());
    let supported_client = state.read_supported_client(CLIENT_TYPE());
    assert_eq!(supported_client, CLIENT());
}

#[test]
fn test_get_client_ok() {
    let mut state = setup();
    state.register_client(CLIENT_TYPE(), CLIENT());
    let client = state.get_client(CLIENT_TYPE());
    assert_eq!(client.address, CLIENT());
}

#[test]
#[should_panic(expected: 'ICS02: client address is 0')]
fn test_get_cleint_fail() {
    let state = setup();
    state.get_client(CLIENT_TYPE());
}

#[test]
fn test_emit_create_client() {
    let mut state = setup();
    let mut spy = spy_events();
    let create_resp = CreateResponse { client_id: CLIENT_ID(), height: HEIGHT(10) };
    state.emit_create_client_event(create_resp);
    spy.assert_create_client_event(test_address(), CLIENT_ID(), HEIGHT(10));
}

#[test]
fn test_emit_update_client() {
    let mut state = setup();
    let mut spy = spy_events();
    let heights = array![HEIGHT(5), HEIGHT(10)];
    let header = array![0, 1, 2, 3];
    state.emit_update_client_event(CLIENT_ID(), heights.clone(), header.clone());
    spy.assert_update_client_event(test_address(), CLIENT_ID(), heights, header);
}

