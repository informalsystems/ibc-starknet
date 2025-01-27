use snforge_std::start_cheat_caller_address;
use starknet_ibc_apps::transfer::ERC20Contract;
use starknet_ibc_testkit::configs::TransferAppConfigTrait;
use starknet_ibc_testkit::dummies::{NAME, SYMBOL, SUPPLY, COSMOS, STARKNET, USER, CS_USER};
use starknet_ibc_testkit::event_spy::TransferEventSpyExt;
use starknet_ibc_testkit::handles::{AppHandle, CoreHandle, ERC20Handle};
use starknet_ibc_testkit::setup::{setup, Mode};
use starknet_ibc_utils::ComputeKey;

#[test]
fn test_escrow_unescrow_roundtrip() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, ics20, mut erc20, _, _, transfer_cfg, mut spy) = setup(Mode::WithChannel);

    // -----------------------------------------------------------
    // Escrow
    // -----------------------------------------------------------

    start_cheat_caller_address(ics20.address, USER());

    // User approves the amount of allowance for the `TransferApp` contract.
    erc20.approve(USER(), ics20.address, transfer_cfg.amount);

    let msg_transfer = transfer_cfg
        .dummy_msg_transfer(transfer_cfg.native_denom.clone(), CS_USER());

    // Submit a `MsgTransfer` to the `TransferApp` contract.
    ics20.send_transfer(msg_transfer);

    // Assert the `SendEvent` emitted.
    spy
        .assert_send_event(
            ics20.address, USER(), CS_USER(), transfer_cfg.native_denom.clone(), transfer_cfg.amount
        );

    // Check the balance of the sender.
    erc20.assert_balance(USER(), SUPPLY - transfer_cfg.amount);

    // Check the balance of the `TransferApp` contract.
    erc20.assert_balance(ics20.address, transfer_cfg.amount);

    // -----------------------------------------------------------
    // Unescrow
    // -----------------------------------------------------------

    start_cheat_caller_address(ics20.address, core.address);

    let prefixed_denom = transfer_cfg.prefix_native_denom();

    let msg_recv_packet = transfer_cfg
        .dummy_msg_recv_packet(prefixed_denom.clone(), COSMOS(), STARKNET());

    core.recv_packet(msg_recv_packet.clone());

    // Assert the `RecvEvent` emitted.
    spy
        .assert_recv_event(
            ics20.address, CS_USER(), USER(), prefixed_denom, transfer_cfg.amount, true
        );

    erc20.assert_balance(ics20.address, 0);

    // Check the balance of the recipient.
    erc20.assert_balance(USER(), SUPPLY);
}

#[test]
fn test_mint_burn_roundtrip() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, ics20, _, _, _, transfer_cfg, mut spy) = setup(Mode::WithChannel);

    // -----------------------------------------------------------
    // Mint
    // -----------------------------------------------------------

    let msg_recv_packet = transfer_cfg
        .dummy_msg_recv_packet(transfer_cfg.hosted_denom.clone(), COSMOS(), STARKNET());

    core.recv_packet(msg_recv_packet.clone());

    let prefixed_denom = transfer_cfg.prefix_hosted_denom();

    // Fetch the token address.
    let token_address = ics20.ibc_token_address(prefixed_denom.key());

    // Assert the `CreateTokenEvent` emitted.
    spy
        .assert_create_token_event(
            ics20.address, NAME(), SYMBOL(), token_address, transfer_cfg.amount
        );

    // Assert the `RecvEvent` emitted.
    spy
        .assert_recv_event(
            ics20.address, CS_USER(), USER(), prefixed_denom.clone(), transfer_cfg.amount, true
        );

    let erc20: ERC20Contract = token_address.into();

    // Check the balance of the receiver.
    erc20.assert_balance(USER(), transfer_cfg.amount);

    // Check the total supply of the ERC20 contract.
    erc20.assert_total_supply(transfer_cfg.amount);

    // -----------------------------------------------------------
    // Burn
    // -----------------------------------------------------------

    start_cheat_caller_address(ics20.address, USER());

    let msg_transfer = transfer_cfg.dummy_msg_transfer(prefixed_denom.clone(), CS_USER());

    // User approves the amount of allowance for the `TransferApp` contract.
    ics20.send_transfer(msg_transfer);

    // Assert the `SendEvent` emitted.
    spy.assert_send_event(ics20.address, USER(), CS_USER(), prefixed_denom, transfer_cfg.amount);

    // Check the balance of the sender.
    erc20.assert_balance(USER(), 0);

    // Check the balance of the `TransferApp` contract.
    erc20.assert_balance(ics20.address, 0);

    // Check the total supply of the ERC20 contract.
    erc20.assert_total_supply(0);
}
