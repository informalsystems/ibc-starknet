use snforge_std::start_cheat_block_timestamp_global;
use starknet_ibc_clients::cometbft::CometClientComponent::{
    CometClientHandler, CometClientQuery, ClientReaderImpl
};
use starknet_ibc_clients::cometbft::CometClientComponent;
use starknet_ibc_core::client::StatusTrait;
use starknet_ibc_testkit::configs::CometClientConfigTrait;
use starknet_ibc_testkit::dummies::HEIGHT;
use starknet_ibc_testkit::mocks::MockCometClient;

type ComponentState = CometClientComponent::ComponentState<MockCometClient::ContractState>;

fn COMPONENT_STATE() -> ComponentState {
    CometClientComponent::component_state_for_testing()
}

fn setup() -> ComponentState {
    let mut state = COMPONENT_STATE();
    state
}

#[test]
fn test_create_client_ok() {
    let mut state = setup();
    let mut cfg = CometClientConfigTrait::default();
    start_cheat_block_timestamp_global(cfg.latest_timestamp + 1);
    let msg = cfg.dummy_msg_create_client();
    state.create_client(msg);
    assert_eq!(state.client_type(), cfg.client_type);
    assert_eq!(state.latest_height(0), cfg.latest_height);
    assert!(state.status(0).is_active());
}

/// Test that the client sequence is updated correctly by creating two clients.
#[test]
fn test_client_sequence_ok() {
    let mut state = setup();
    let mut cfg = CometClientConfigTrait::default();
    start_cheat_block_timestamp_global(cfg.latest_timestamp + 1);
    let msg = cfg.dummy_msg_create_client();
    state.create_client(msg.clone());
    state.create_client(msg);
    assert_eq!(state.read_client_sequence(), 2);
}

#[test]
fn test_update_client_ok() {
    let mut state = setup();
    let mut cfg = CometClientConfigTrait::default();
    start_cheat_block_timestamp_global(cfg.latest_timestamp.clone() + 1);
    let msg = cfg.dummy_msg_create_client();
    let create_resp = state.create_client(msg);
    let updating_height = cfg.latest_height.clone() + HEIGHT(1);
    let updating_timestmap = cfg.latest_timestamp + 1;
    let msg = cfg
        .dummy_msg_update_client(
            create_resp.client_id, create_resp.height, updating_height.clone(), updating_timestmap
        );
    state.update_client(msg);
    assert_eq!(state.client_type(), cfg.client_type);
    assert_eq!(state.latest_height(0), updating_height);
    assert!(state.status(0).is_active());
}

#[test]
#[should_panic(expected: 'ICS07: missing client state')]
fn test_missing_client_state() {
    let mut state = setup();
    state.read_client_state(0);
}

#[test]
#[should_panic(expected: 'ICS07: missing consensus state')]
fn test_missing_consensus_state() {
    let mut state = setup();
    state.read_consensus_state(0, HEIGHT(5));
}

#[test]
#[should_panic(expected: 'ICS07: missing processed time')]
fn test_missing_client_processed_time() {
    let mut state = setup();
    state.read_client_processed_time(0, HEIGHT(5));
}

#[test]
#[should_panic(expected: 'ICS07: missing processed height')]
fn test_missing_client_processed_height() {
    let mut state = setup();
    state.read_client_processed_height(0, HEIGHT(5));
}
