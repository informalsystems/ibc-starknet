use snforge_std::cheat_block_timestamp_global;
use starknet_ibc_clients::cometbft::CometClientComponent::{
    CometClientHandler, CometCommonClientState, ClientReaderImpl
};
use starknet_ibc_clients::cometbft::CometClientComponent;
use starknet_ibc_clients::tests::{MockCometClient, CometClientConfigTrait};
use starknet_ibc_core::client::StatusTrait;
use starknet_ibc_core::tests::HEIGHT;

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
    cheat_block_timestamp_global(cfg.latest_timestamp + 1);
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
    cheat_block_timestamp_global(cfg.latest_timestamp + 1);
    let msg = cfg.dummy_msg_create_client();
    state.create_client(msg.clone());
    state.create_client(msg);
    assert_eq!(state.read_client_sequence(), 2);
}

#[test]
fn test_update_client_ok() {
    let mut state = setup();
    let mut cfg = CometClientConfigTrait::default();
    cheat_block_timestamp_global(cfg.latest_timestamp + 1);
    let msg = cfg.dummy_msg_create_client();
    let create_resp = state.create_client(msg);
    let updating_height = cfg.latest_height.clone() + HEIGHT(5);
    let msg = cfg
        .dummy_msg_update_client(
            create_resp.client_id, create_resp.height, updating_height.clone()
        );
    state.update_client(msg);
    assert_eq!(state.client_type(), cfg.client_type);
    assert_eq!(state.latest_height(0), updating_height);
    assert!(state.status(0).is_active());
}
