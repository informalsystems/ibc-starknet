use openzeppelin_testing::events::EventSpyExt;
use snforge_std::{start_cheat_caller_address, spy_events};
use starknet_ibc_apps::tests::TransferEventSpyExt;
use starknet_ibc_apps::tests::{
    TransferAppConfigTrait, NAME, SYMBOL, SUPPLY, OWNER, COSMOS, STARKNET
};
use starknet_ibc_apps::transfer::ERC20Contract;
use starknet_ibc_contracts::tests::{SetupImpl, ERC20Handle, AppHandle};
use starknet_ibc_utils::ComputeKey;

#[test]
fn test_escrow_unescrow_roundtrip() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let mut cfg = TransferAppConfigTrait::default();

    let setup = SetupImpl::default();

    let mut erc20 = setup.deploy_erc20();

    let mut ics20 = setup.deploy_trasnfer();

    cfg.set_native_denom(erc20.address);

    // Set the caller address to `OWNER`, as ICS-20 callbacks are permissioned.
    start_cheat_caller_address(ics20.address, OWNER());

    let mut spy = spy_events();

    // -----------------------------------------------------------
    // Escrow
    // -----------------------------------------------------------

    // Owner approves the amount of allowance for the `TransferApp` contract.
    erc20.approve(OWNER(), ics20.address, cfg.amount);

    let msg_transfer = cfg.dummy_msg_transder(cfg.native_denom.clone(), STARKNET(), COSMOS());

    // Submit a `MsgTransfer` to the `TransferApp` contract.
    ics20.send_transfer(msg_transfer);

    // Assert the `SendEvent` emitted.
    spy
        .assert_send_event(
            ics20.address, STARKNET(), COSMOS(), cfg.native_denom.clone(), cfg.amount
        );

    // Check the balance of the sender.
    erc20.assert_balance(OWNER(), SUPPLY - cfg.amount);

    // Check the balance of the `TransferApp` contract.
    erc20.assert_balance(ics20.address, cfg.amount);

    // -----------------------------------------------------------
    // Unescrow
    // -----------------------------------------------------------

    spy.drop_all_events();

    let prefixed_denom = cfg.prefix_native_denom();

    let recv_packet = cfg.dummy_recv_packet(prefixed_denom.clone(), COSMOS(), STARKNET());

    // Submit a `RecvPacket` to the `TransferApp` contract.
    ics20.on_recv_packet(recv_packet);

    // Assert the `RecvEvent` emitted.
    spy.assert_recv_event(ics20.address, COSMOS(), STARKNET(), prefixed_denom, cfg.amount, true);

    erc20.assert_balance(ics20.address, 0);

    // Check the balance of the recipient.
    erc20.assert_balance(OWNER(), SUPPLY);
}

#[test]
fn test_mint_burn_roundtrip() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let mut cfg = TransferAppConfigTrait::default();

    let setup = SetupImpl::default();

    let mut ics20 = setup.deploy_trasnfer();

    // Set the caller address, as callbacks are permissioned.
    start_cheat_caller_address(ics20.address, OWNER());

    let mut spy = spy_events();

    // -----------------------------------------------------------
    // Mint
    // -----------------------------------------------------------

    let recv_packet = cfg.dummy_recv_packet(cfg.hosted_denom.clone(), COSMOS(), STARKNET());

    // Submit a `RecvPacket`, which will create a new ERC20 contract.
    ics20.on_recv_packet(recv_packet.clone());

    let prefixed_denom = cfg.prefix_hosted_denom();

    // Fetch the token address.
    let token_address = ics20.ibc_token_address(prefixed_denom.key()).unwrap();

    // Assert the `CreateTokenEvent` emitted.
    spy.assert_create_token_event(ics20.address, NAME(), SYMBOL(), token_address, cfg.amount);

    // Assert the `RecvEvent` emitted.
    spy
        .assert_recv_event(
            ics20.address, COSMOS(), STARKNET(), prefixed_denom.clone(), cfg.amount, true
        );

    spy.drop_all_events();

    // Submit another `RecvPacket`, which will mint the amount of tokens.
    ics20.on_recv_packet(recv_packet);

    // Assert the `RecvEvent` emitted.
    spy
        .assert_recv_event(
            ics20.address, COSMOS(), STARKNET(), prefixed_denom.clone(), cfg.amount, true
        );

    let erc20: ERC20Contract = token_address.into();

    // Check the balance of the receiver.
    erc20.assert_balance(OWNER(), cfg.amount * 2);

    // Check the total supply of the ERC20 contract.
    erc20.assert_total_supply(cfg.amount * 2);

    // -----------------------------------------------------------
    // Burn
    // -----------------------------------------------------------

    spy.drop_all_events();

    let msg_transfer = cfg.dummy_msg_transder(prefixed_denom.clone(), STARKNET(), COSMOS());

    // Owner approves the amount of allowance for the `TransferApp` contract.
    ics20.send_transfer(msg_transfer);

    // Assert the `SendEvent` emitted.
    spy.assert_send_event(ics20.address, STARKNET(), COSMOS(), prefixed_denom, cfg.amount);

    // Check the balance of the sender.
    erc20.assert_balance(OWNER(), cfg.amount);

    // Check the balance of the `TransferApp` contract.
    erc20.assert_balance(ics20.address, 0);

    // Chekck the total supply of the ERC20 contract.
    erc20.assert_total_supply(cfg.amount);
}
