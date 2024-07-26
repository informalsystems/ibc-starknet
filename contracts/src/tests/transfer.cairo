use core::starknet::SyscallResultTrait;
use core::traits::TryInto;
use openzeppelin::tests::utils::{deploy, pop_log};
use openzeppelin::token::erc20::{ERC20ABIDispatcher, ERC20ABIDispatcherTrait};
use openzeppelin::utils::serde::SerializedAppend;
use starknet::ContractAddress;
use starknet::testing;
use starknet_ibc::apps::transfer::component::ICS20TransferComponent;
use starknet_ibc::apps::transfer::interface::{
    ISendTransferDispatcher, ISendTransferDispatcherTrait, IRecvPacketDispatcher,
    IRecvPacketDispatcherTrait
};
use starknet_ibc::apps::transfer::types::Denom;
use starknet_ibc::presets::{Transfer, ERC20};
use starknet_ibc::tests::utils::{
    AMOUNT, SUPPLY, PREFIXED_DENOM, OWNER, RECIPIENT, dummy_erc20_call_data, dummy_msg_transder,
    dummy_recv_packet
};

fn setup_erc20() -> (ERC20ABIDispatcher, ContractAddress) {
    let contract_address = deploy(ERC20::TEST_CLASS_HASH, dummy_erc20_call_data());
    (ERC20ABIDispatcher { contract_address }, contract_address)
}

fn setup_ics20() -> (ISendTransferDispatcher, IRecvPacketDispatcher, ContractAddress) {
    let mut call_data = array![];
    call_data.append_serde(ERC20::TEST_CLASS_HASH);

    let contract_address = deploy(Transfer::TEST_CLASS_HASH, call_data);

    (
        ISendTransferDispatcher { contract_address },
        IRecvPacketDispatcher { contract_address },
        contract_address
    )
}

#[test]
fn test_escrow() {
    // Deploy an ERC20 contract.
    let (erc20_dispatcher, token_address) = setup_erc20();

    // Deploy an ICS20 Token Transfer contract.
    let (send_disptacher, _, transfer_address) = setup_ics20();

    // Owner approves the amount of allowance for the `Transfer` contract.
    testing::set_contract_address(OWNER());
    erc20_dispatcher.approve(transfer_address, AMOUNT);

    // Submit a `MsgTransfer` to the `Transfer` contract.
    send_disptacher.send_execute(dummy_msg_transder(token_address.into()));

    // Check the balance of the owner.
    let balance = erc20_dispatcher.balance_of(OWNER());
    assert_eq!(balance, SUPPLY - AMOUNT);

    // Check the balance of the `Transfer` contract.
    let balance = erc20_dispatcher.balance_of(transfer_address);
    assert_eq!(balance, AMOUNT);
}

#[test]
fn test_mint() {
    // Deploy an ICS20 Token Transfer contract.
    let (_, recv_disptacher, _) = setup_ics20();

    // Submit a `RecvPacket` to the `Transfer` contract.
    recv_disptacher.recv_validate(dummy_recv_packet(Denom::IBC(PREFIXED_DENOM())));
}

