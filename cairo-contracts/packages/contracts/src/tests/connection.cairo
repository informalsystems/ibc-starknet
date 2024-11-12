use starknet_ibc_contracts::tests::channel::setup;
use starknet_ibc_testkit::configs::CoreConfigTrait;
use starknet_ibc_testkit::handles::CoreHandle;

#[test]
fn test_conn_open_init_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, _, _, core_cfg, _, _, _) = setup();

    // -----------------------------------------------------------
    // Connection Open Init
    // -----------------------------------------------------------

    let msg = core_cfg.dummy_msg_conn_open_init();

    core.conn_open_init(msg.clone());
}

#[test]
fn test_conn_open_try_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, _, _, core_cfg, _, _, _) = setup();

    // -----------------------------------------------------------
    // Connection Open Try
    // -----------------------------------------------------------

    let msg = core_cfg.dummy_msg_conn_open_try();

    core.conn_open_try(msg.clone());
}

#[test]
fn test_conn_open_ack_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, _, _, core_cfg, _, _, _) = setup();

    // -----------------------------------------------------------
    // Connection Open Ack
    // -----------------------------------------------------------

    let msg = core_cfg.dummy_msg_conn_open_ack();

    core.conn_open_ack(msg.clone());
}

#[test]
fn test_conn_open_confirm_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, _, _, core_cfg, _, _, _) = setup();

    // -----------------------------------------------------------
    // Connection Open Confirm
    // -----------------------------------------------------------

    let msg = core_cfg.dummy_msg_conn_open_confirm();

    core.conn_open_confirm(msg.clone());
}
