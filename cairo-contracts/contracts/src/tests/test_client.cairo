use snforge_std::cheat_block_timestamp_global;
use starknet_ibc_contracts::tests::configs::CometClientConfigTrait;
use starknet_ibc_contracts::tests::setups::{
    IBCCoreHandle, IBCCoreHandleTrait, CometClientHandle, CometClientHandleTrait
};
use starknet_ibc_core::client::StatusTrait;
use starknet_ibc_core::client::{UpdateResponse, Height};

// Deploys the IBC core and Comet client contracts, and registers the Comet
// client into the IBC core.
fn setup_contracts(client_type: felt252) -> (IBCCoreHandle, CometClientHandle) {
    // Deploy an IBC core contract.
    let mut ibc = IBCCoreHandleTrait::setup();

    // Deploy a Comet client contract.
    let comet = CometClientHandleTrait::setup();

    // Register the Comet client into the IBC core contract.
    ibc.register_client(client_type, comet.contract_address);

    (ibc, comet)
}

#[test]
fn test_create_comet_client_ok() {
    // -----------------------------------------------------------
    // Setup Contracts
    // -----------------------------------------------------------

    let mut cfg = CometClientConfigTrait::default();

    let (mut ibc, comet) = setup_contracts(cfg.client_type);

    // -----------------------------------------------------------
    // Create Client
    // -----------------------------------------------------------

    // Cheat the block timestamp to simulate the passage of time.
    cheat_block_timestamp_global(cfg.latest_timestamp + 1);

    let msg = cfg.dummy_msg_create_client();

    // Submit a `MsgCreateClient` to the IBC core contract.
    let resp = ibc.create_client(msg);

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    // Assert the `CreateClientEvent` emitted.
    ibc.assert_create_event(resp.client_id, resp.height);

    assert(comet.client_type() == cfg.client_type, 'client type mismatch');

    assert(comet.latest_height(0) == cfg.latest_height, 'latest height mismatch');

    assert(comet.status(0).is_active(), 'status mismatch');
}

#[test]
fn test_update_comet_client_ok() {
    // -----------------------------------------------------------
    // Setup Contracts
    // -----------------------------------------------------------

    let mut cfg = CometClientConfigTrait::default();

    let (mut ibc, comet) = setup_contracts(cfg.client_type);

    // -----------------------------------------------------------
    // Create Client
    // -----------------------------------------------------------

    // Cheat the block timestamp to simulate the passage of time.
    cheat_block_timestamp_global(cfg.latest_timestamp + 1);

    let msg_create_client = cfg.dummy_msg_create_client();

    // Submit a `MsgCreateClient` to the IBC core contract.
    let create_resp = ibc.create_client(msg_create_client);

    // -----------------------------------------------------------
    // Update Client
    // -----------------------------------------------------------

    ibc.drop_all_events();

    // Update the client to a new height.
    let updating_height = cfg.latest_height.clone()
        + Height { revision_number: 0, revision_height: 5 };

    // Create a `MsgUpdateClient` message.
    let msg = cfg
        .dummy_msg_update_client(
            create_resp.client_id, create_resp.height, updating_height.clone()
        );

    // Submit a `MsgUpdateClient` to the IBC core contract.
    let update_resp = ibc.update_client(msg.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    if let UpdateResponse::Success(heights) = update_resp {
        // Assert the `UpdateClientEvent` emitted.
        ibc.assert_update_event(msg.client_id, heights, msg.client_message);
    } else {
        panic!("update client failed");
    }

    assert(comet.client_type() == cfg.client_type, 'client type mismatch');

    assert(comet.latest_height(0) == updating_height, 'latest height mismatch');

    assert(comet.status(0).is_active(), 'status mismatch');
}
