use snforge_std::start_cheat_caller_address;
use starknet_ibc_apps::transfer::ERC20Contract;
use starknet_ibc_apps::transfer::types::{PrefixedDenomTrait, TracePrefixTrait};
use starknet_ibc_testkit::configs::TransferAppConfigTrait;
use starknet_ibc_testkit::dummies::{
    CHANNEL_ID, COSMOS, CS_USER, DECIMAL_ZERO, NAME, PORT_ID, SN_USER, STARKNET, SUPPLY, SYMBOL,
};
use starknet_ibc_testkit::event_spy::ERC20EventSpyExt;
use starknet_ibc_testkit::event_spy::{ERC20EventSpyExtImpl, TransferEventSpyExt};
use starknet_ibc_testkit::handles::{AppHandle, CoreHandle, ERC20Handle};
use starknet_ibc_testkit::setup::{Mode, setup};
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

    start_cheat_caller_address(ics20.address, SN_USER());

    // User approves the amount of allowance for the `TransferApp` contract.
    erc20.approve(SN_USER(), ics20.address, transfer_cfg.amount);

    let msg_transfer = transfer_cfg
        .dummy_msg_transfer(transfer_cfg.native_denom.clone(), CS_USER());

    // Submit a `MsgTransfer` to the `TransferApp` contract.
    ics20.send_transfer(msg_transfer);

    // Assert the `SendEvent` emitted.
    spy
        .assert_send_event(
            ics20.address,
            SN_USER(),
            CS_USER(),
            transfer_cfg.native_denom.clone(),
            transfer_cfg.amount,
        );

    // Check the balance of the sender.
    erc20.assert_balance(SN_USER(), SUPPLY - transfer_cfg.amount);

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
            ics20.address, CS_USER(), SN_USER(), prefixed_denom, transfer_cfg.amount, true,
        );

    erc20.assert_balance(ics20.address, 0);

    // Check the balance of the recipient.
    erc20.assert_balance(SN_USER(), SUPPLY);
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

    let mut erc20: ERC20Contract = token_address.into();

    // Assert the `CreateTokenEvent` emitted.
    spy.assert_create_token_event(ics20.address, NAME(), SYMBOL(), DECIMAL_ZERO, token_address);

    // Assert if ICS20 performs the mint.
    spy.assert_transfer_event(erc20.address, ics20.address, SN_USER(), transfer_cfg.amount);

    // Assert the `RecvEvent` emitted.
    spy
        .assert_recv_event(
            ics20.address, CS_USER(), SN_USER(), prefixed_denom.clone(), transfer_cfg.amount, true,
        );

    // Assert if the transfer happens from the ICS20 address.
    spy.assert_transfer_event(erc20.address, ics20.address, SN_USER(), transfer_cfg.amount);

    // Check the balance of the receiver.
    erc20.assert_balance(SN_USER(), transfer_cfg.amount);

    // Check the total supply of the ERC20 contract.
    erc20.assert_total_supply(transfer_cfg.amount);

    // -----------------------------------------------------------
    // Burn
    // -----------------------------------------------------------

    start_cheat_caller_address(ics20.address, SN_USER());

    // User approves the amount of burn allowance to the `TransferApp` contract.
    erc20.approve(SN_USER(), ics20.address, transfer_cfg.amount);

    let msg_transfer = transfer_cfg.dummy_msg_transfer(prefixed_denom.clone(), CS_USER());

    // User approves the amount of allowance for the `TransferApp` contract.
    ics20.send_transfer(msg_transfer);

    // Assert the `SendEvent` emitted.
    spy.assert_send_event(ics20.address, SN_USER(), CS_USER(), prefixed_denom, transfer_cfg.amount);

    // Assert if the burn happens by the ICS20 contract.
    spy.assert_transfer_event(erc20.address, SN_USER(), ics20.address, transfer_cfg.amount);

    // Check the balance of the sender.
    erc20.assert_balance(SN_USER(), 0);

    // Check the balance of the `TransferApp` contract.
    erc20.assert_balance(ics20.address, 0);

    // Check the total supply of the ERC20 contract.
    erc20.assert_total_supply(0);
}

#[test]
fn test_mint_with_pre_created_token() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, ics20, _, _, _, transfer_cfg, mut spy) = setup(Mode::WithChannel);

    let prefixed_denom = transfer_cfg.prefix_hosted_denom();

    /// Pre-creates the IBC token
    let token_address = ics20.create_ibc_token(prefixed_denom.clone());

    // -----------------------------------------------------------
    // Mint
    // -----------------------------------------------------------

    let msg_recv_packet = transfer_cfg
        .dummy_msg_recv_packet(transfer_cfg.hosted_denom.clone(), COSMOS(), STARKNET());

    core.recv_packet(msg_recv_packet.clone());

    spy
        .assert_recv_event(
            ics20.address, CS_USER(), SN_USER(), prefixed_denom.clone(), transfer_cfg.amount, true,
        );

    let mut erc20: ERC20Contract = token_address.into();

    spy.assert_transfer_event(erc20.address, ics20.address, SN_USER(), transfer_cfg.amount);

    erc20.assert_balance(SN_USER(), transfer_cfg.amount);

    erc20.assert_total_supply(transfer_cfg.amount);
}

#[test]
fn test_create_ibc_token_ok() {
    let (_, ics20, _, _, _, transfer_cfg, mut spy) = setup(Mode::WithChannel);
    let prefixed_denom = transfer_cfg.prefix_hosted_denom();
    let address = ics20.create_ibc_token(prefixed_denom.clone());
    spy.assert_create_token_event(ics20.address, NAME(), SYMBOL(), DECIMAL_ZERO, address);
    let queried = ics20.ibc_token_address(prefixed_denom.key());
    assert_eq!(address, queried);
}

#[test]
fn test_create_ibc_token_with_multihop() {
    let (_, ics20, _, _, _, mut transfer_cfg, mut spy) = setup(Mode::WithChannel);

    /// Prefix for the source chain with an arbitrary channel ID
    let trace_prefix = TracePrefixTrait::new(PORT_ID(), CHANNEL_ID(10));
    transfer_cfg.hosted_denom.add_prefix(trace_prefix);

    /// Second prefix for the intermediate chain, right before coming into Starknet.
    let prefixed_denom = transfer_cfg.prefix_hosted_denom();

    let address = ics20.create_ibc_token(prefixed_denom.clone());
    spy.assert_create_token_event(ics20.address, NAME(), SYMBOL(), DECIMAL_ZERO, address);
    let queried = ics20.ibc_token_address(prefixed_denom.key());
    assert_eq!(address, queried);
}

#[test]
#[should_panic(expected: 'ICS20: token already exists')]
fn test_recreate_existing_ibc_token() {
    let (_, ics20, _, _, _, transfer_cfg, _) = setup(Mode::WithChannel);
    let prefixed_denom = transfer_cfg.prefix_hosted_denom();
    ics20.create_ibc_token(prefixed_denom.clone());

    // The second time creation should panic.
    ics20.create_ibc_token(prefixed_denom.clone());
}

#[test]
#[should_panic(expected: 'ICS20: missing trace prefix')]
fn test_create_ibc_token_without_prefix() {
    let (_, ics20, _, _, _, transfer_cfg, _) = setup(Mode::WithChannel);
    ics20.create_ibc_token(transfer_cfg.hosted_denom);
}

#[test]
#[should_panic(expected: 'ICS20: invalid denom')]
fn test_create_ibc_token_with_wrong_base() {
    let (_, ics20, _, _, _, transfer_cfg, _) = setup(Mode::WithChannel);
    let prefixed_denom = transfer_cfg.prefix_native_denom();
    ics20.create_ibc_token(prefixed_denom);
}

#[test]
#[should_panic(expected: 'ICS04: missing channel end')]
fn test_create_ibc_token_without_channel() {
    let (_, ics20, _, _, _, transfer_cfg, _) = setup(Mode::WithConnection);
    let prefixed_denom = transfer_cfg.prefix_hosted_denom();
    ics20.create_ibc_token(prefixed_denom.clone());
}
