use core::num::traits::Bounded;
use openzeppelin_testing::spy_events;
use snforge_std::test_address;
use starknet::storage::{StorageAsPointer, StoragePathEntry};
use starknet_ibc_core::client::ClientHandlerComponent::{
    ClientInitializerImpl, ClientInternalImpl, ClientReaderTrait, ClientWriterTrait,
    CoreClientHandlerImpl, CoreRegisterClientImpl, EventEmitterImpl,
};
use starknet_ibc_core::client::{ClientHandlerComponent, CreateResponse, Duration, DurationImpl};
use starknet_ibc_testkit::dummies::{CLIENT, CLIENT_ID, CLIENT_TYPE, HEIGHT, RELAYER};
use starknet_ibc_testkit::event_spy::ClientEventSpyExt;
use starknet_ibc_testkit::mocks::MockClientHandler;

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
fn test_register_client() {
    let mut state = setup();
    state.register_client(CLIENT_TYPE(), CLIENT());
    let supported_client = state.read_supported_client(CLIENT_TYPE());
    assert_eq!(supported_client, CLIENT());
}

#[test]
fn test_allowed_relayers() {
    let mut state = setup();
    assert!(!state.in_allowed_relayers(RELAYER()));
    state.write_allowed_relayer(RELAYER());
    assert!(state.in_allowed_relayers(RELAYER()));
}

#[test]
fn test_get_client_ok() {
    let mut state = setup();
    state.register_client(CLIENT_TYPE(), CLIENT());
    let client = state.get_client(CLIENT_TYPE());
    assert_eq!(client.address, CLIENT());
}

#[should_panic(expected: 'ICS02: client address is 0')]
#[test]
fn test_get_client_fail() {
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

// Must not overflow.
#[test]
fn test_duration_max() {
    Duration { seconds: Bounded::MAX, nanos: Bounded::MAX }.as_nanos();
}

#[test]
fn test_schedule_upgrade_keys() {
    let mut state = COMPONENT_STATE();

    assert_eq!(state.final_height.__base_address__.into(), selector!("final_height"));

    assert_eq!(state.final_height.__storage_pointer_address__.into(), selector!("final_height"));

    assert_eq!(
        state.final_height.__storage_pointer_address__.into(),
        0x30faa2222f968e83202b54d3d8135ee4204634ae65d21f20f86a2732cbabfca,
    );

    assert_eq!(
        state
            .upgraded_client_state_commitments
            .entry(42)
            .as_ptr()
            .__storage_pointer_address__
            .into(),
        0x7f1877168ebc2b7ec579aa0f1514007124ad2c19fe35a56f3b12d2c68718a44,
    );
    assert_eq!(
        state
            .upgraded_consensus_state_commitments
            .entry(42)
            .as_ptr()
            .__storage_pointer_address__
            .into(),
        0xef005e48e802e8403a09622b8ffd8299020c511293a5ed773b0f5d80ab81b9,
    )
}
