use core::num::traits::Zero;
use starknet_ibc_contracts::tests::channel::setup;
use starknet_ibc_core::connection::{ConnectionState, ConnectionEndTrait};
use starknet_ibc_testkit::configs::CoreConfigTrait;
use starknet_ibc_testkit::dummies::CONNECTION_ID;
use starknet_ibc_testkit::event_spy::ConnectionEventSpyExt;
use starknet_ibc_testkit::handles::CoreHandle;

#[test]
fn test_conn_open_init_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, _, _, core_cfg, _, _, mut spy) = setup();

    // -----------------------------------------------------------
    // Connection Open Init
    // -----------------------------------------------------------

    let msg = core_cfg.dummy_msg_conn_open_init();

    core.conn_open_init(msg.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    let conn_id_on_a = core.connection_end(CONNECTION_ID(0));

    assert_eq!(conn_id_on_a.state(), @ConnectionState::Init);
    assert_eq!(conn_id_on_a.client_id, msg.client_id_on_a.clone());
    assert_eq!(conn_id_on_a.counterparty.client_id, msg.client_id_on_b.clone());
    assert!(conn_id_on_a.counterparty.connection_id.is_zero());

    spy
        .assert_conn_open_init_event(
            core.address, msg.client_id_on_a, CONNECTION_ID(0), msg.client_id_on_b,
        );
}

#[test]
fn test_conn_open_try_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, _, _, core_cfg, _, _, mut spy) = setup();

    // -----------------------------------------------------------
    // Connection Open Try
    // -----------------------------------------------------------

    let msg = core_cfg.dummy_msg_conn_open_try();

    core.conn_open_try(msg.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    let conn_id_on_b = core.connection_end(CONNECTION_ID(0));

    assert_eq!(conn_id_on_b.state(), @ConnectionState::TryOpen);
    assert_eq!(conn_id_on_b.counterparty.client_id, msg.client_id_on_a.clone());
    assert_eq!(conn_id_on_b.counterparty.connection_id, msg.conn_id_on_a.clone());

    spy
        .assert_conn_open_try_event(
            core.address,
            msg.client_id_on_b,
            CONNECTION_ID(0),
            msg.client_id_on_a,
            msg.conn_id_on_a,
        );
}

#[test]
fn test_conn_open_ack_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, _, _, core_cfg, _, _, mut spy) = setup();

    // -----------------------------------------------------------
    // Connection Open Init
    // -----------------------------------------------------------

    let msg = core_cfg.dummy_msg_conn_open_init();

    core.conn_open_init(msg);

    // -----------------------------------------------------------
    // Connection Open Ack
    // -----------------------------------------------------------

    let msg = core_cfg.dummy_msg_conn_open_ack();

    core.conn_open_ack(msg.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    let conn_id_on_a = core.connection_end(CONNECTION_ID(0));

    assert_eq!(conn_id_on_a.state(), @ConnectionState::Open);
    assert_eq!(msg.version, conn_id_on_a.version);

    spy
        .assert_conn_open_ack_event(
            core.address,
            conn_id_on_a.client_id,
            CONNECTION_ID(0),
            conn_id_on_a.counterparty.client_id,
            conn_id_on_a.counterparty.connection_id,
        );
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
