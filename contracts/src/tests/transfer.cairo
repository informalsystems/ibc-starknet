use openzeppelin::token::erc20::ERC20ABIDispatcherTrait;
use starknet::ContractAddress;
use starknet::testing;
use starknet_ibc::apps::transfer::component::ICS20TransferComponent::Event as TransferEvent;
use starknet_ibc::apps::transfer::interface::{
    ISendTransferDispatcherTrait, IRecvPacketDispatcherTrait, ITokenAddressDispatcherTrait
};
use starknet_ibc::tests::setup::{ERC20ContractTrait, ICS20TransferContractTrait};
use starknet_ibc::tests::utils::{
    AMOUNT, SUPPLY, HOSTED_PREFIXED_DENOM, OWNER, RECIPIENT, dummy_msg_transder, dummy_recv_packet,
    NATIVE_PREFIXED_DENOM, BARE_DENOM
};

#[test]
fn test_escrow_unescrow_roundtrip() {
    // -----------------------------------------------------------
    // Setup Contracts
    // -----------------------------------------------------------

    // Deploy an ERC20 contract.
    let erc20 = ERC20ContractTrait::setup();

    // Deploy an ICS20 Token Transfer contract.
    let ics20 = ICS20TransferContractTrait::setup();

    // -----------------------------------------------------------
    // Escrow
    // -----------------------------------------------------------

    // Owner approves the amount of allowance for the `Transfer` contract.
    testing::set_contract_address(OWNER());
    erc20.dispatcher().approve(ics20.contract_address, AMOUNT);

    // Submit a `MsgTransfer` to the `Transfer` contract.
    ics20
        .send_dispatcher()
        .send_execute(dummy_msg_transder(BARE_DENOM(erc20.contract_address), OWNER(), RECIPIENT()));

    // Assert the `SendEvent` emitted.
    let event = ics20.assert_send_event();

    // Check the balance of the sender.
    let sender_balance = erc20.dispatcher().balance_of(event.sender);
    assert_eq!(sender_balance, SUPPLY - AMOUNT);

    // Check the balance of the `Transfer` contract.
    let contract_balance = erc20.dispatcher().balance_of(ics20.contract_address);
    assert_eq!(contract_balance, AMOUNT);

    // -----------------------------------------------------------
    // Unescrow
    // -----------------------------------------------------------

    // Submit a `RecvPacket` to the `Transfer` contract.
    ics20
        .recv_dispatcher()
        .recv_execute(
            dummy_recv_packet(NATIVE_PREFIXED_DENOM(erc20.contract_address), OWNER(), RECIPIENT())
        );

    // Assert the `RecvEvent` emitted.
    let event = ics20.assert_recv_event();

    let contract_balance = erc20.dispatcher().balance_of(ics20.contract_address);
    assert_eq!(contract_balance, 0);

    // Check the balance of the recipient.
    let receiver_balance = erc20.dispatcher().balance_of(event.receiver);
    assert_eq!(receiver_balance, AMOUNT);
}

#[test]
fn test_mint_burn_roundtrip() {
    // -----------------------------------------------------------
    // Setup Contracts
    // -----------------------------------------------------------

    // Deploy an ICS20 Token Transfer contract.
    let ics20 = ICS20TransferContractTrait::setup();

    // -----------------------------------------------------------
    // Mint
    // -----------------------------------------------------------

    // Submit a `RecvPacket`, which will create a new ERC20 contract.
    ics20
        .recv_dispatcher()
        .recv_execute(dummy_recv_packet(HOSTED_PREFIXED_DENOM(), OWNER(), RECIPIENT()));

    // Assert the `RecvEvent` emitted.
    ics20.assert_recv_event();

    // Submit another `RecvPacket`, which will mint the amount of tokens.
    ics20
        .recv_dispatcher()
        .recv_execute(dummy_recv_packet(HOSTED_PREFIXED_DENOM(), OWNER(), RECIPIENT()));

    // Assert the `RecvEvent` emitted.
    let event = ics20.assert_recv_event();

    // Check the balance of the receiver.
    let token_address = ics20.addr_dispatcher().ibc_token_address(HOSTED_PREFIXED_DENOM()).unwrap();

    let erc20 = ERC20ContractTrait::setup_with_addr(token_address);

    let receiver_balance = erc20.dispatcher().balance_of(event.receiver);
    assert_eq!(receiver_balance, AMOUNT * 2);

    // -----------------------------------------------------------
    // Burn
    // -----------------------------------------------------------

    // Owner approves the amount of allowance for the `Transfer` contract.
    ics20
        .send_dispatcher()
        .send_execute(dummy_msg_transder(HOSTED_PREFIXED_DENOM(), RECIPIENT(), OWNER()));

    // Assert the `SendEvent` emitted.
    let event = ics20.assert_send_event();

    // Check the balance of the sender.
    let sender_balance = erc20.dispatcher().balance_of(event.sender);
    assert_eq!(sender_balance, AMOUNT);

    let contract_balance = erc20.dispatcher().balance_of(ics20.contract_address);
    assert_eq!(contract_balance, 0);
}
