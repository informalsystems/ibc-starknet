use core::traits::TryInto;
use openzeppelin_testing::declare_class;
use snforge_std::start_cheat_caller_address;
use starknet::ContractAddress;
use starknet_ibc_apps::transfer::ERC20Contract;
use starknet_ibc_contracts::tests::configs::transfer::TransferAppConfigTrait;
use starknet_ibc_contracts::tests::constants::{NAME, SYMBOL, SUPPLY, OWNER, COSMOS, STARKNET};
use starknet_ibc_contracts::tests::setups::{ERC20ContractTrait, TransferAppHandleTrait};
use starknet_ibc_utils::ComputeKeyTrait;

#[test]
fn test_escrow_unescrow_roundtrip() {
    // -----------------------------------------------------------
    // Setup Contracts
    // -----------------------------------------------------------

    let mut cfg = TransferAppConfigTrait::default();

    // Declare the ERC20 contract class.
    let erc20_contract_class = declare_class("ERC20Mintable");

    // Deploy an ERC20 contract.
    let mut erc20 = ERC20ContractTrait::setup(erc20_contract_class);

    cfg.set_native_denom(erc20.address);

    // Deploy an ICS20 Token Transfer contract.
    let mut ics20 = TransferAppHandleTrait::setup(erc20_contract_class);

    // -----------------------------------------------------------
    // Escrow
    // -----------------------------------------------------------

    // Owner approves the amount of allowance for the `TransferApp` contract.
    erc20.approve(OWNER(), ics20.contract_address, cfg.amount);

    let msg_transfer = cfg.dummy_msg_transder(cfg.native_denom.clone(), STARKNET(), COSMOS());

    // Submit a `MsgTransfer` to the `TransferApp` contract.
    ics20.send_execute(msg_transfer);

    // Assert the `SendEvent` emitted.
    ics20.assert_send_event(STARKNET(), COSMOS(), cfg.native_denom.clone(), cfg.amount);

    // Check the balance of the sender.
    erc20.assert_balance(OWNER(), SUPPLY - cfg.amount);

    // Check the balance of the `TransferApp` contract.
    erc20.assert_balance(ics20.contract_address, cfg.amount);

    // -----------------------------------------------------------
    // Unescrow
    // -----------------------------------------------------------

    ics20.drop_all_events();

    let prefixed_denom = cfg.prefix_native_denom();

    let recv_packet = cfg.dummy_recv_packet(prefixed_denom.clone(), COSMOS(), STARKNET());

    // Submit a `RecvPacket` to the `TransferApp` contract.
    ics20.recv_execute(recv_packet);

    // Assert the `RecvEvent` emitted.
    ics20.assert_recv_event(COSMOS(), STARKNET(), prefixed_denom, cfg.amount, true);

    erc20.assert_balance(ics20.contract_address, 0);

    // Check the balance of the recipient.
    erc20.assert_balance(OWNER(), SUPPLY);
}

#[test]
fn test_mint_burn_roundtrip() {
    // -----------------------------------------------------------
    // Setup Contracts
    // -----------------------------------------------------------

    let mut cfg = TransferAppConfigTrait::default();

    // Declare the ERC20 contract class.
    let erc20_contract_class = declare_class("ERC20Mintable");

    // Deploy an ICS20 Token Transfer contract.
    let mut ics20 = TransferAppHandleTrait::setup(erc20_contract_class);

    // -----------------------------------------------------------
    // Mint
    // -----------------------------------------------------------

    let recv_packet = cfg.dummy_recv_packet(cfg.hosted_denom.clone(), COSMOS(), STARKNET());

    // Submit a `RecvPacket`, which will create a new ERC20 contract.
    ics20.recv_execute(recv_packet.clone());

    let prefixed_denom = cfg.prefix_hosted_denom();

    // Fetch the token address.
    let token_address = ics20.ibc_token_address(prefixed_denom.compute_key()).unwrap();

    // Assert the `CreateTokenEvent` emitted.
    ics20.assert_create_token_event(NAME(), SYMBOL(), token_address, cfg.amount);

    // Assert the `RecvEvent` emitted.
    ics20.assert_recv_event(COSMOS(), STARKNET(), prefixed_denom.clone(), cfg.amount, true);

    ics20.drop_all_events();

    // Submit another `RecvPacket`, which will mint the amount of tokens.
    ics20.recv_execute(recv_packet);

    // Assert the `RecvEvent` emitted.
    ics20.assert_recv_event(COSMOS(), STARKNET(), prefixed_denom.clone(), cfg.amount, true);

    let erc20: ERC20Contract = token_address.into();

    // Check the balance of the receiver.
    erc20.assert_balance(OWNER(), cfg.amount * 2);

    // Check the total supply of the ERC20 contract.
    erc20.assert_total_supply(cfg.amount * 2);

    // -----------------------------------------------------------
    // Burn
    // -----------------------------------------------------------

    ics20.drop_all_events();

    let msg_transfer = cfg.dummy_msg_transder(prefixed_denom.clone(), STARKNET(), COSMOS());

    // Owner approves the amount of allowance for the `TransferApp` contract.
    ics20.send_execute(msg_transfer);

    // Assert the `SendEvent` emitted.
    ics20.assert_send_event(STARKNET(), COSMOS(), prefixed_denom, cfg.amount);

    // Check the balance of the sender.
    erc20.assert_balance(OWNER(), cfg.amount);

    // Check the balance of the `TransferApp` contract.
    erc20.assert_balance(ics20.contract_address, 0);

    // Chekck the total supply of the ERC20 contract.
    erc20.assert_total_supply(cfg.amount);
}
