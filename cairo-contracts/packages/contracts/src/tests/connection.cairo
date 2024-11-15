use core::num::traits::Zero;
use snforge_std::{spy_events, EventSpy};
use starknet_ibc_apps::transfer::ERC20Contract;
use starknet_ibc_core::connection::{ConnectionState, ConnectionEndTrait};
use starknet_ibc_core::router::AppContract;
use starknet_ibc_testkit::configs::{
    TransferAppConfig, TransferAppConfigTrait, CoreConfig, CoreConfigTrait, CometClientConfig,
    CometClientConfigTrait
};
use starknet_ibc_testkit::dummies::CONNECTION_ID;
use starknet_ibc_testkit::event_spy::ConnectionEventSpyExt;
use starknet_ibc_testkit::handles::{CoreContract, CoreHandle};
use starknet_ibc_testkit::setup::SetupImpl;

fn setup() -> (
    CoreContract,
    AppContract,
    ERC20Contract,
    CoreConfig,
    CometClientConfig,
    TransferAppConfig,
    EventSpy
) {
    // -----------------------------------------------------------
    // Setup Contracts
    // -----------------------------------------------------------

    let mut core_cfg = CoreConfigTrait::default();

    let mut comet_cfg = CometClientConfigTrait::default();

    let mut transfer_cfg = TransferAppConfigTrait::default();

    let (core, ics20, mut erc20) = SetupImpl::setup_full("IBCCore", "CometClient", "TransferApp");

    transfer_cfg.set_native_denom(erc20.address);

    let mut spy = spy_events();

    // -----------------------------------------------------------
    // Create Client
    // -----------------------------------------------------------

    let msg_create_client = comet_cfg.dummy_msg_create_client();

    core.create_client(msg_create_client);

    (core, ics20, erc20, core_cfg, comet_cfg, transfer_cfg, spy)
}

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

    let conn_id_on_a = core.connection_end(msg.conn_id_on_a.clone());

    assert_eq!(conn_id_on_a.state(), @ConnectionState::Open);
    assert_eq!(conn_id_on_a.counterparty.connection_id.clone(), msg.conn_id_on_b.clone());
    assert_eq!(msg.version, conn_id_on_a.version);

    spy
        .assert_conn_open_ack_event(
            core.address,
            conn_id_on_a.client_id,
            msg.conn_id_on_a,
            conn_id_on_a.counterparty.client_id,
            conn_id_on_a.counterparty.connection_id,
        );
}

#[test]
fn test_conn_open_confirm_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, _, _, core_cfg, _, _, mut spy) = setup();

    // -----------------------------------------------------------
    // Connection Open Try
    // -----------------------------------------------------------

    let msg = core_cfg.dummy_msg_conn_open_try();

    core.conn_open_try(msg);

    // -----------------------------------------------------------
    // Connection Open Confirm
    // -----------------------------------------------------------

    let msg = core_cfg.dummy_msg_conn_open_confirm();

    core.conn_open_confirm(msg.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    let conn_id_on_b = core.connection_end(msg.conn_id_on_b.clone());

    assert_eq!(conn_id_on_b.state(), @ConnectionState::Open);

    spy
        .assert_conn_open_confirm_event(
            core.address,
            conn_id_on_b.client_id,
            msg.conn_id_on_b,
            conn_id_on_b.counterparty.client_id,
            conn_id_on_b.counterparty.connection_id,
        );
}
