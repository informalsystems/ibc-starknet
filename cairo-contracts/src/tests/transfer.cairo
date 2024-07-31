use openzeppelin::token::erc20::ERC20ABIDispatcherTrait;
use starknet::ContractAddress;
use starknet::testing;
use starknet_ibc::apps::transfer::component::ICS20TransferComponent::Event as TransferEvent;
use starknet_ibc::apps::transfer::interface::{
    ISendTransferDispatcherTrait, IRecvPacketDispatcherTrait, ITokenAddressDispatcherTrait
};
use starknet_ibc::apps::transfer::types::PrefixedDenomTrait;
use starknet_ibc::tests::config::TestConfigTrait;
use starknet_ibc::tests::constants::{SUPPLY, OWNER, RECIPIENT,};
use starknet_ibc::tests::setup::{ERC20ContractTrait, ICS20TransferContractTrait};

#[test]
fn test_escrow_unescrow_roundtrip() {
    // -----------------------------------------------------------
    // Setup Contracts
    // -----------------------------------------------------------

    // Deploy an ERC20 contract.
    let erc20 = ERC20ContractTrait::setup();

    // Deploy an ICS20 Token Transfer contract.
    let ics20 = ICS20TransferContractTrait::setup();

    let mut cfg = TestConfigTrait::default();
    cfg.set_native_denom(erc20.contract_address);

    // -----------------------------------------------------------
    // Escrow
    // -----------------------------------------------------------

    // Owner approves the amount of allowance for the `Transfer` contract.
    erc20.approve(OWNER(), ics20.contract_address, cfg.amount);

    let msg_transfer = cfg.dummy_msg_transder(cfg.native_denom.clone(), OWNER(), RECIPIENT());

    // Submit a `MsgTransfer` to the `Transfer` contract.
    ics20.send_execute(msg_transfer);

    // Assert the `SendEvent` emitted.
    let event = ics20.assert_send_event();

    // Check the balance of the sender.
    erc20.assert_balance(event.sender, SUPPLY - cfg.amount);

    // Check the balance of the `Transfer` contract.
    erc20.assert_balance(ics20.contract_address, cfg.amount);

    // -----------------------------------------------------------
    // Unescrow
    // -----------------------------------------------------------

    cfg.prefix_native_denom();

    let recv_packet = cfg.dummy_recv_packet(cfg.native_denom, OWNER(), RECIPIENT());

    // Submit a `RecvPacket` to the `Transfer` contract.
    ics20.recv_execute(recv_packet);

    // Assert the `RecvEvent` emitted.
    let event = ics20.assert_recv_event();

    erc20.assert_balance(ics20.contract_address, 0);

    // Check the balance of the recipient.
    erc20.assert_balance(event.receiver, cfg.amount);
}

#[test]
fn test_mint_burn_roundtrip() {
    // -----------------------------------------------------------
    // Setup Contracts
    // -----------------------------------------------------------

    // Deploy an ICS20 Token Transfer contract.
    let ics20 = ICS20TransferContractTrait::setup();

    let mut cfg = TestConfigTrait::default();

    // -----------------------------------------------------------
    // Mint
    // -----------------------------------------------------------

    let recv_packet = cfg.dummy_recv_packet(cfg.hosted_denom.clone(), OWNER(), RECIPIENT());

    // Submit a `RecvPacket`, which will create a new ERC20 contract.
    ics20.recv_execute(recv_packet.clone());

    // Assert the `RecvEvent` emitted.
    ics20.assert_recv_event();

    // Submit another `RecvPacket`, which will mint the amount of tokens.
    ics20.recv_execute(recv_packet);

    // Assert the `RecvEvent` emitted.
    let event = ics20.assert_recv_event();

    cfg.prefix_hosted_denom();

    // Check the balance of the receiver.
    let token_address = ics20.ibc_token_address(cfg.hosted_denom.compute_key()).unwrap();

    let erc20 = ERC20ContractTrait::setup_with_addr(token_address);

    erc20.assert_balance(event.receiver, cfg.amount * 2);

    // Check the total supply of the ERC20 contract.
    erc20.assert_total_supply(cfg.amount * 2);

    // -----------------------------------------------------------
    // Burn
    // -----------------------------------------------------------

    let msg_transfer = cfg.dummy_msg_transder(cfg.hosted_denom, RECIPIENT(), OWNER());

    // Owner approves the amount of allowance for the `Transfer` contract.
    ics20.send_execute(msg_transfer);

    // Assert the `SendEvent` emitted.
    let event = ics20.assert_send_event();

    // Check the balance of the sender.
    erc20.assert_balance(event.sender, cfg.amount);

    // Check the balance of the `Transfer` contract.
    erc20.assert_balance(ics20.contract_address, 0);

    // Chekck the total supply of the ERC20 contract.
    erc20.assert_total_supply(cfg.amount);
}
