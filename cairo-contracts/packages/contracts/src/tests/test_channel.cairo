use starknet_ibc_apps::tests::{TransferAppConfigTrait, COSMOS, STARKNET};
use starknet_ibc_contracts::tests::{SetupImpl, CoreHandle};

#[test]
#[should_panic]
fn test_recv_packet_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let mut transfer_cfg = TransferAppConfigTrait::default();

    let setup = SetupImpl::default();

    let mut core = setup.deploy_core();

    let _comet = setup.deploy_cometbft(ref core);

    let _ics20 = setup.deploy_trasnfer();

    // -----------------------------------------------------------
    // Receive Packet
    // -----------------------------------------------------------

    let msg = transfer_cfg
        .dummy_msg_recv_packet(transfer_cfg.hosted_denom.clone(), COSMOS(), STARKNET());

    core.recv_packet(msg);
}

