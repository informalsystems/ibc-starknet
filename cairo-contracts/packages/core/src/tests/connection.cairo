use core::num::traits::Zero;
use starknet_ibc_core::connection::ConnectionHandlerComponent;
use starknet_ibc_core::connection::ConnectionHandlerComponent::{
    ConnectionReaderTrait, ConnectionWriterTrait,
};
use starknet_ibc_testkit::dummies::{CLIENT_ID, CONNECTION_END, CONNECTION_ID};
use starknet_ibc_testkit::mocks::MockConnectionHandler;

type ComponentState =
    ConnectionHandlerComponent::ComponentState<MockConnectionHandler::ContractState>;

fn COMPONENT_STATE() -> ComponentState {
    ConnectionHandlerComponent::component_state_for_testing()
}

fn setup() -> ComponentState {
    let mut state = COMPONENT_STATE();
    state
}

#[test]
fn test_initial_state() {
    let state = setup();
    let next_connection_sequence = state.read_next_connection_sequence();
    assert!(next_connection_sequence.is_zero());
}

#[test]
fn test_write_read_connection_end_ok() {
    let mut state = setup();
    state.write_connection_end(@CONNECTION_ID(10), CONNECTION_END(1));
    let channel_end = state.read_connection_end(@CONNECTION_ID(10));
    assert_eq!(channel_end, CONNECTION_END(1));
}

#[test]
#[should_panic(expected: 'ICS03: missing connection end')]
fn test_missing_connection_end() {
    let state = setup();
    state.read_connection_end(@CONNECTION_ID(10));
}

#[test]
fn test_write_read_client_to_connections_ok() {
    let mut state = setup();
    state.write_client_to_connections(@CLIENT_ID(), CONNECTION_ID(1));
    state.write_client_to_connections(@CLIENT_ID(), CONNECTION_ID(2));
    let conn_ids = state.read_client_to_connections(@CLIENT_ID());
    assert_eq!(conn_ids, array![CONNECTION_ID(1), CONNECTION_ID(2)]);
}

#[test]
#[should_panic(expected: 'ICS03: zero connections')]
fn test_zero_connections() {
    let state = setup();
    state.read_client_to_connections(@CLIENT_ID());
}
