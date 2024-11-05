use openzeppelin_testing::events::EventSpyExt;
use snforge_std::spy_events;
use starknet_ibc_core::client::{UpdateResponse, StatusTrait, ClientContractTrait};
use starknet_ibc_testkit::configs::CometClientConfigTrait;
use starknet_ibc_testkit::dummies::HEIGHT;
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

    let msg = cfg.dummy_msg_create_client();

    // Submit a `MsgCreateClient` to the IBC core contract.
    let resp = core.create_client(msg);

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    // Assert the `CreateClientEvent` emitted.
    spy.assert_create_client_event(core.address, resp.client_id, resp.height);

    assert_eq!(comet.client_type(), cfg.client_type);

    assert_eq!(comet.latest_height(0), cfg.latest_height);

    assert!(comet.status(0).is_active());
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

    let msg_create_client = cfg.dummy_msg_create_client();

    // Submit a `MsgCreateClient` to the IBC core contract.
    let create_resp = core.create_client(msg_create_client);

    // -----------------------------------------------------------
    // Update Client
    // -----------------------------------------------------------

    spy.drop_all_events();

    // Update the client to a new height and time.
    let updating_height = cfg.latest_height.clone() + HEIGHT(1);
    let updating_time = cfg.latest_timestamp.clone() + 1;

    // Create a `MsgUpdateClient` message.
    let msg = cfg
        .dummy_msg_update_client(
            create_resp.client_id,
            create_resp.height,
            updating_height.clone(),
            updating_time.clone()
        );

    // Submit a `MsgUpdateClient` to the IBC core contract.
    let update_resp = core.update_client(msg.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    if let UpdateResponse::Success(heights) = update_resp {
        // Assert the `UpdateClientEvent` emitted.
        spy.assert_update_client_event(core.address, msg.client_id, heights, msg.client_message);
    } else {
        panic!("update client failed");
    }

    assert_eq!(comet.client_type(), cfg.client_type);

    assert_eq!(comet.latest_height(0), updating_height);

    assert!(comet.status(0).is_active());
}
