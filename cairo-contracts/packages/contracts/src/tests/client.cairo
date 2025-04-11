use openzeppelin_testing::spy_events;
use snforge_std::{start_cheat_block_number_global, start_cheat_block_timestamp_global};
use starknet_ibc_core::client::{ClientContractTrait, StatusTrait, TimestampTrait, UpdateResponse};
use starknet_ibc_core::commitment::StateRootZero;
use starknet_ibc_testkit::configs::CometClientConfigTrait;
use starknet_ibc_testkit::dummies::{HEIGHT, TIMESTAMP};
use starknet_ibc_testkit::event_spy::ClientEventSpyExt;
use starknet_ibc_testkit::handles::CoreHandle;
use starknet_ibc_testkit::setup::SetupImpl;

#[test]
fn test_create_comet_client_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let mut cfg = CometClientConfigTrait::default();

    let (mut core, mut comet) = SetupImpl::setup_core_with_client("IBCCore", "CometClient");

    let mut spy = spy_events();

    // -----------------------------------------------------------
    // Create Client
    // -----------------------------------------------------------

    let resp = cfg.create_client(@core);

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    assert_eq!(comet.client_type(), cfg.client_type);
    assert_eq!(comet.latest_height(0), cfg.latest_height);
    assert!(comet.status(0).is_active());

    // Assert the `CreateClientEvent` emitted.
    spy.assert_create_client_event(core.address, resp.client_id, resp.height);
}

#[test]
fn test_update_comet_client_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let mut cfg = CometClientConfigTrait::default();

    let (mut core, mut comet) = SetupImpl::setup_core_with_client("IBCCore", "CometClient");

    let mut spy = spy_events();

    // -----------------------------------------------------------
    // Create Client
    // -----------------------------------------------------------

    let create_resp = cfg.create_client(@core);

    // -----------------------------------------------------------
    // Update Client
    // -----------------------------------------------------------

    // Update the client to a new height and time.
    let updating_height = cfg.latest_height.clone() + HEIGHT(1);
    let updating_time = cfg.latest_timestamp.clone() + TIMESTAMP(1);

    // Create a `MsgUpdateClient` message.
    let msg = cfg
        .dummy_msg_update_client(
            create_resp.client_id,
            create_resp.height,
            updating_height.clone(),
            updating_time.clone(),
        );

    // Submit a `MsgUpdateClient` to the IBC core contract.
    let update_resp = core.update_client(msg.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    assert_eq!(comet.client_type(), cfg.client_type);
    assert_eq!(comet.latest_height(0), updating_height);
    assert!(comet.status(0).is_active());

    if let UpdateResponse::Success(heights) = update_resp {
        // Assert the `UpdateClientEvent` emitted.
        spy.assert_update_client_event(core.address, msg.client_id, heights, msg.client_message);
    } else {
        panic!("update client failed");
    }
}

#[test]
#[should_panic(expected: 'ICS07: missing consensus state')]
fn test_prune_consensus_state() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let mut cfg = CometClientConfigTrait::default();

    let (mut core, comet) = SetupImpl::setup_core_with_client("IBCCore", "CometClient");

    // -----------------------------------------------------------
    // Create Client
    // -----------------------------------------------------------

    let create_resp = cfg.create_client(@core);

    // -----------------------------------------------------------
    // Update Client
    // -----------------------------------------------------------

    // Update the client to a new height and time.
    let first_updating_height = cfg.latest_height.clone() + HEIGHT(1);
    let updating_time = cfg.latest_timestamp.clone() + TIMESTAMP(1);

    // Create a `MsgUpdateClient` message.
    let msg = cfg
        .dummy_msg_update_client(
            create_resp.client_id.clone(),
            create_resp.height.clone(),
            first_updating_height.clone(),
            updating_time.clone(),
        );

    // Submit a `MsgUpdateClient` to the IBC core contract.
    core.update_client(msg.clone());

    // -----------------------------------------------------------
    // Wait for 55 seconds
    // -----------------------------------------------------------

    let new_timestamp = cfg.latest_timestamp.clone().as_secs() + 55;
    start_cheat_block_timestamp_global(new_timestamp);
    start_cheat_block_number_global(5);

    cfg.latest_timestamp = (new_timestamp * 1_000_000_000).into();
    cfg.latest_height.revision_height = cfg.latest_height.revision_height + 5;

    // -----------------------------------------------------------
    // Update Client
    // -----------------------------------------------------------

    // Update the client to a new height and time.
    let second_updating_height = cfg.latest_height.clone() + HEIGHT(1);
    let updating_time = cfg.latest_timestamp.clone() + TIMESTAMP(1);

    // Create a `MsgUpdateClient` message.
    let msg = cfg
        .dummy_msg_update_client(
            create_resp.client_id.clone(),
            create_resp.height.clone(),
            second_updating_height.clone(),
            updating_time.clone(),
        );

    // Submit a `MsgUpdateClient` to the IBC core contract.
    core.update_client(msg.clone());

    // -----------------------------------------------------------
    // Wait for 55 seconds
    // -----------------------------------------------------------

    let new_timestamp = cfg.latest_timestamp.clone().as_secs() + 55;
    start_cheat_block_timestamp_global(new_timestamp);
    start_cheat_block_number_global(5);

    cfg.latest_timestamp = (new_timestamp * 1_000_000_000).into();
    cfg.latest_height.revision_height = cfg.latest_height.revision_height + 5;

    // -----------------------------------------------------------
    // Update Client
    // -----------------------------------------------------------

    // Update the client to a new height and time.
    let third_updating_height = cfg.latest_height.clone() + HEIGHT(1);
    let updating_time = cfg.latest_timestamp.clone() + TIMESTAMP(1);

    // Create a `MsgUpdateClient` message.
    let msg = cfg
        .dummy_msg_update_client(
            create_resp.client_id,
            create_resp.height,
            third_updating_height.clone(),
            updating_time.clone(),
        );

    // Submit a `MsgUpdateClient` to the IBC core contract.
    core.update_client(msg.clone());

    // -----------------------------------------------------------
    // Wait for 50 seconds
    // -----------------------------------------------------------

    let new_timestamp = cfg.latest_timestamp.clone().as_secs() + 50;
    start_cheat_block_timestamp_global(new_timestamp);
    start_cheat_block_number_global(5);

    cfg.latest_timestamp = (new_timestamp * 1_000_000_000).into();
    cfg.latest_height.revision_height = cfg.latest_height.revision_height + 5;

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    let third_consensus_state = comet.consensus_state_root(0, third_updating_height.clone());
    assert!(third_consensus_state.is_non_zero());
    let second_consensus_state = comet.consensus_state_root(0, second_updating_height.clone());
    assert!(second_consensus_state.is_non_zero());
    // Should panic as the first consensus state has been pruned
    comet.consensus_state_root(0, first_updating_height.clone());
}
