use MockClientComponent::ClientWriterTrait;
use snforge_std::start_cheat_block_timestamp_global;
use starknet_ibc_clients::mock::MockClientComponent;
use starknet_ibc_clients::mock::MockClientComponent::{
    ClientReaderImpl, MockClientHandler, MockClientQuery,
};
use starknet_ibc_core::client::{StatusTrait, TimestampTrait};
use starknet_ibc_testkit::configs::MockClientConfigTrait;
use starknet_ibc_testkit::dummies::{HEIGHT, TIMESTAMP};
use starknet_ibc_testkit::mocks::MockCometClient;

type ComponentState = MockClientComponent::ComponentState<MockCometClient::ContractState>;

fn COMPONENT_STATE() -> ComponentState {
    MockClientComponent::component_state_for_testing()
}

fn setup() -> ComponentState {
    let mut state = COMPONENT_STATE();
    state
}

/// Performs an update client by taking an elapsed timestamp from the client's latest timestamp.
fn simulate_update_client(elapsed_timestamp: u64) {
    let mut state = setup();
    let mut cfg = MockClientConfigTrait::default();
    start_cheat_block_timestamp_global(cfg.latest_timestamp.clone().as_secs() + 1);
    let msg = cfg.dummy_msg_create_client();
    let create_resp = state.create_client(msg);
    let updating_height = cfg.latest_height.clone() + HEIGHT(1);
    let updating_timestamp = cfg.latest_timestamp.clone() + TIMESTAMP(elapsed_timestamp);
    let msg = cfg
        .dummy_msg_update_client(
            create_resp.client_id,
            create_resp.height,
            updating_height.clone(),
            updating_timestamp.clone(),
        );
    state.update_client(msg);
    assert_eq!(state.client_type(), cfg.client_type);
    assert!(state.status(0).is_active());
    assert_eq!(state.latest_height(0), updating_height);
    let consensus_state = state.read_consensus_state(0, updating_height);
    assert_eq!(consensus_state.timestamp, updating_timestamp);
    state.read_client_processed_time(0, updating_height);
    state.read_client_processed_height(0, updating_height);
}

#[test]
fn test_create_client_ok() {
    let mut state = setup();
    let mut cfg = MockClientConfigTrait::default();
    start_cheat_block_timestamp_global(cfg.latest_timestamp.clone().as_secs() + 1);
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
    let mut cfg = MockClientConfigTrait::default();
    start_cheat_block_timestamp_global(cfg.latest_timestamp.clone().as_secs() + 1);
    let msg = cfg.dummy_msg_create_client();
    state.create_client(msg.clone());
    state.create_client(msg);
    assert_eq!(state.read_next_client_sequence(), 2);
}

#[test]
fn test_update_client_ok() {
    simulate_update_client(1)
}

// Tests with a future header with the elapsed timestamp within the `max_clock_drift`.
#[test]
fn test_update_client_with_valid_future_header() {
    simulate_update_client(2)
}

// Tests with a future header with the elapsed timestamp exceeds the `max_clock_drift`.
#[test]
#[should_panic(expected: 'ICS07: inv header from future')]
fn test_update_client_with_invalid_future_header() {
    simulate_update_client(3)
}

#[test]
fn test_update_client_with_older_header() {
    let mut state = setup();
    let mut cfg = MockClientConfigTrait::default();
    start_cheat_block_timestamp_global(cfg.latest_timestamp.clone().as_secs() + 2);
    let msg = cfg.dummy_msg_create_client();
    let create_resp = state.create_client(msg);

    // First update client to height = 12.
    let updating_height_1 = cfg.latest_height.clone() + HEIGHT(2);
    let updating_timestamp_1 = cfg.latest_timestamp.clone() + TIMESTAMP(2);
    let msg = cfg
        .dummy_msg_update_client(
            create_resp.client_id.clone(),
            create_resp.height,
            updating_height_1.clone(),
            updating_timestamp_1,
        );

    // Second update client with an older height = 11.
    let updating_height_2 = cfg.latest_height.clone() + HEIGHT(1);
    let updating_timestamp_2 = cfg.latest_timestamp.clone() + TIMESTAMP(1);
    state.update_client(msg);
    let msg = cfg
        .dummy_msg_update_client(
            create_resp.client_id,
            updating_height_1,
            updating_height_2.clone(),
            updating_timestamp_2.clone(),
        );

    state.update_client(msg);
    assert_eq!(state.client_type(), cfg.client_type);
    assert!(state.status(0).is_active());
    assert_eq!(state.latest_height(0), updating_height_1);
    let heights = state.read_update_heights(0);
    assert_eq!(heights, array![cfg.latest_height, updating_height_2, updating_height_1]);
    let consensus_state = state.read_consensus_state(0, updating_height_2);
    assert_eq!(consensus_state.timestamp, updating_timestamp_2);
    state.read_client_processed_time(0, updating_height_2); // It panics if not exist. 
    state.read_client_processed_height(0, updating_height_2); // It panics if not exist. 
}

#[test]
fn test_client_misbehaviour() {
    let mut state = setup();
    let mut cfg = MockClientConfigTrait::default();
    start_cheat_block_timestamp_global(cfg.latest_timestamp.clone().as_secs() + 1);
    let msg = cfg.dummy_msg_create_client();
    let create_resp = state.create_client(msg);
    let updating_height = cfg.latest_height.clone() + HEIGHT(1);
    let updating_timestamp = cfg.latest_timestamp.clone() + TIMESTAMP(1);
    let (msg1, msg2) = cfg
        .dummy_msg_misbehaviour_client(
            create_resp.client_id,
            create_resp.height,
            updating_height.clone(),
            updating_timestamp.clone(),
        );
    state.update_client(msg1);
    assert!(state.status(0).is_active());
    state.update_client(msg2);
    assert!(state.status(0).is_frozen());
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

#[test]
fn test_empty_update_heights() {
    let mut state = setup();
    let heights = state.read_update_heights(0);
    assert!(heights.len() == 0);
}

#[test]
fn test_write_duplicate_update_height() {
    let mut state = setup();
    state.write_update_height(0, HEIGHT(5));
    state.write_update_height(0, HEIGHT(5));
    let heights = state.read_update_heights(0);
    assert_eq!(heights, array![HEIGHT(5)]);
}

#[test]
fn test_update_heights_sort() {
    let mut state = setup();
    state.write_update_height(0, HEIGHT(1));
    state.write_update_height(0, HEIGHT(4));
    state.write_update_height(0, HEIGHT(2));
    state.write_update_height(0, HEIGHT(3));
    let heights = state.read_update_heights(0);
    assert_eq!(heights, array![HEIGHT(1), HEIGHT(2), HEIGHT(3), HEIGHT(4)]);
}

#[test]
fn test_update_height_before() {
    let mut state = setup();
    state.write_update_height(0, HEIGHT(5));
    let height = state.update_height_before(0, HEIGHT(3));
    assert_eq!(height, None);

    state.write_update_height(0, HEIGHT(2));
    let height = state.update_height_before(0, HEIGHT(3));
    assert_eq!(height, Some(HEIGHT(2)));

    state.write_update_height(0, HEIGHT(4));
    let height = state.update_height_before(0, HEIGHT(4));
    assert_eq!(height, Some(HEIGHT(4)));

    state.write_update_height(0, HEIGHT(6));
    let height = state.update_height_before(0, HEIGHT(7));
    assert_eq!(height, Some(HEIGHT(6)));

    let height = state.update_height_before(0, HEIGHT(1));
    assert_eq!(height, None);
}

#[test]
fn test_update_height_after() {
    let mut state = setup();
    state.write_update_height(0, HEIGHT(2));
    let height = state.update_height_after(0, HEIGHT(3));
    assert_eq!(height, None);

    state.write_update_height(0, HEIGHT(4));
    let height = state.update_height_after(0, HEIGHT(3));
    assert_eq!(height, Some(HEIGHT(4)));

    state.write_update_height(0, HEIGHT(7));
    let height = state.update_height_after(0, HEIGHT(6));
    assert_eq!(height, Some(HEIGHT(7)));

    state.write_update_height(0, HEIGHT(6));
    let height = state.update_height_after(0, HEIGHT(6));
    assert_eq!(height, Some(HEIGHT(6)));

    let height = state.update_height_after(0, HEIGHT(8));
    assert_eq!(height, None);
}

#[test]
fn test_update_heights_max_size() {
    let mut state = setup();
    let mut i = 0;
    while i != 101 {
        state.write_update_height(0, HEIGHT(i));
        i += 1;
    }
    let heights = state.read_update_heights(0);
    assert_eq!(heights.len(), 100);
    assert_eq!(heights.at(99), @HEIGHT(100));
}

#[test]
#[should_panic(expected: 'ICS07: zero update heights')]
fn test_update_height_before_empty() {
    let mut state = setup();
    let _ = state.update_height_before(0, HEIGHT(3));
}

#[test]
#[should_panic(expected: 'ICS07: zero update heights')]
fn test_update_height_after_empty() {
    let mut state = setup();
    let _ = state.update_height_before(0, HEIGHT(3));
}
