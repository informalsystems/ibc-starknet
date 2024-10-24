use openzeppelin_testing::events::EventSpyExt;
use starknet_ibc_apps::transfer::ERC20Contract;
use starknet_ibc_contracts::tests::setup_full;
use starknet_ibc_testkit::configs::{TransferAppConfigTrait, CometClientConfigTrait};
use starknet_ibc_testkit::dummies::{NAME, SYMBOL, SUPPLY, OWNER, COSMOS, STARKNET};
use starknet_ibc_testkit::event_spy::TransferEventSpyExt;
use starknet_ibc_testkit::handles::{AppHandle, CoreHandle, ERC20Handle};
use starknet_ibc_utils::ComputeKey;

#[test]
fn test_escrow_unescrow_roundtrip() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, ics20, mut erc20, mut comet_cfg, mut transfer_cfg, mut spy) = setup_full();

    // -----------------------------------------------------------
    // Create Client
    // -----------------------------------------------------------

    // Create a `MsgCreateClient` message.
    let msg_create_client = comet_cfg.dummy_msg_create_client();

    // Submit the message and create a client.
    core.create_client(msg_create_client);

    // -----------------------------------------------------------
    // Escrow
    // -----------------------------------------------------------

    // Owner approves the amount of allowance for the `TransferApp` contract.
    erc20.approve(OWNER(), ics20.address, transfer_cfg.amount);

    let msg_transfer = transfer_cfg
        .dummy_msg_transfer(transfer_cfg.native_denom.clone(), STARKNET(), COSMOS());

    // Submit a `MsgTransfer` to the `TransferApp` contract.
    ics20.send_transfer(msg_transfer);

    // Assert the `SendEvent` emitted.
    spy
        .assert_send_event(
            ics20.address,
            STARKNET(),
            COSMOS(),
            transfer_cfg.native_denom.clone(),
            transfer_cfg.amount
        );

    // Check the balance of the sender.
    erc20.assert_balance(OWNER(), SUPPLY - transfer_cfg.amount);

    // Check the balance of the `TransferApp` contract.
    erc20.assert_balance(ics20.address, transfer_cfg.amount);

    // -----------------------------------------------------------
    // Unescrow
    // -----------------------------------------------------------

    spy.drop_all_events();

    let prefixed_denom = transfer_cfg.prefix_native_denom();

    let msg_recv_packet = transfer_cfg
        .dummy_msg_recv_packet(prefixed_denom.clone(), COSMOS(), STARKNET());

    core.recv_packet(msg_recv_packet.clone());

    // Assert the `RecvEvent` emitted.
    spy
        .assert_recv_event(
            ics20.address, COSMOS(), STARKNET(), prefixed_denom, transfer_cfg.amount, true
        );

    erc20.assert_balance(ics20.address, 0);

    // Check the balance of the recipient.
    erc20.assert_balance(OWNER(), SUPPLY);
}

#[test]
fn test_mint_burn_roundtrip() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, ics20, _, mut comet_cfg, mut transfer_cfg, mut spy) = setup_full();

    // -----------------------------------------------------------
    // Create Client
    // -----------------------------------------------------------

    // Create a `MsgCreateClient` message.
    let msg_create_client = comet_cfg.dummy_msg_create_client();

    // Submit the message and create a client.
    core.create_client(msg_create_client);

    // -----------------------------------------------------------
    // Mint
    // -----------------------------------------------------------

    let msg_recv_packet = transfer_cfg
        .dummy_msg_recv_packet(transfer_cfg.hosted_denom.clone(), COSMOS(), STARKNET());

    core.recv_packet(msg_recv_packet.clone());

    let prefixed_denom = transfer_cfg.prefix_hosted_denom();

    // Fetch the token address.
    let token_address = ics20.ibc_token_address(prefixed_denom.key()).unwrap();

    // Assert the `CreateTokenEvent` emitted.
    spy
        .assert_create_token_event(
            ics20.address, NAME(), SYMBOL(), token_address, transfer_cfg.amount
        );

    // Assert the `RecvEvent` emitted.
    spy
        .assert_recv_event(
            ics20.address, COSMOS(), STARKNET(), prefixed_denom.clone(), transfer_cfg.amount, true
        );

    let erc20: ERC20Contract = token_address.into();

    // Check the balance of the receiver.
    erc20.assert_balance(OWNER(), transfer_cfg.amount);

    // Check the total supply of the ERC20 contract.
    erc20.assert_total_supply(transfer_cfg.amount);

    // -----------------------------------------------------------
    // Burn
    // -----------------------------------------------------------

    spy.drop_all_events();

    let msg_transfer = transfer_cfg
        .dummy_msg_transfer(prefixed_denom.clone(), STARKNET(), COSMOS());

    // Owner approves the amount of allowance for the `TransferApp` contract.
    ics20.send_transfer(msg_transfer);

    // Assert the `SendEvent` emitted.
    spy.assert_send_event(ics20.address, STARKNET(), COSMOS(), prefixed_denom, transfer_cfg.amount);

    // Check the balance of the sender.
    erc20.assert_balance(OWNER(), 0);

    // Check the balance of the `TransferApp` contract.
    erc20.assert_balance(ics20.address, 0);

    // Chekck the total supply of the ERC20 contract.
    erc20.assert_total_supply(0);
}
