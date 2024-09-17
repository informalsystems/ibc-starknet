use openzeppelin_testing::events::EventSpyExt;
use snforge_std::cheat_block_timestamp_global;
use snforge_std::spy_events;
use starknet_ibc_clients::tests::CometClientConfigTrait;
use starknet_ibc_contracts::tests::handles::{CoreContract, CoreHandle, ClientHandle};
use starknet_ibc_core::client::{
    UpdateResponse, Height, StatusTrait, ClientContract, ClientContractTrait
};
use starknet_ibc_core::tests::ClientEventSpyExt;

// Deploys the IBC core and Comet client contracts, and registers the Comet
// client into the IBC core.
fn setup_contracts(client_type: felt252) -> (CoreContract, ClientContract) {
    // Deploy an IBC core contract.
    let mut core = CoreHandle::setup();

    // Deploy a Comet client contract.
    let comet = ClientHandle::setup_cometbft();

    // Register the Comet client into the IBC core contract.
    core.register_client(client_type, comet.address);

    (core, comet)
}

#[test]
fn test_create_comet_client_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let mut cfg = CometClientConfigTrait::default();

    let (mut core, comet) = setup_contracts(cfg.client_type);

    let mut spy = spy_events();

    // -----------------------------------------------------------
    // Create Client
    // -----------------------------------------------------------

    // Cheat the block timestamp to simulate the passage of time.
    cheat_block_timestamp_global(cfg.latest_timestamp + 1);

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
    // Setup Contracts
    // -----------------------------------------------------------

    let mut cfg = CometClientConfigTrait::default();

    let (mut core, comet) = setup_contracts(cfg.client_type);

    let mut spy = spy_events();

    // -----------------------------------------------------------
    // Create Client
    // -----------------------------------------------------------

    // Cheat the block timestamp to simulate the passage of time.
    cheat_block_timestamp_global(cfg.latest_timestamp + 1);

    let msg_create_client = cfg.dummy_msg_create_client();

    // Submit a `MsgCreateClient` to the IBC core contract.
    let create_resp = core.create_client(msg_create_client);

    // -----------------------------------------------------------
    // Update Client
    // -----------------------------------------------------------

    spy.drop_all_events();

    // Update the client to a new height.
    let updating_height = cfg.latest_height.clone()
        + Height { revision_number: 0, revision_height: 5 };

    // Create a `MsgUpdateClient` message.
    let msg = cfg
        .dummy_msg_update_client(
            create_resp.client_id, create_resp.height, updating_height.clone()
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
