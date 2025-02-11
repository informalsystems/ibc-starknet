use TokenTransferComponent::TransferValidationTrait;
use openzeppelin_testing::events::EventSpyExt;
use snforge_std::cheatcodes::events::EventSpy;
use snforge_std::{spy_events, start_cheat_caller_address};
use starknet::class_hash::class_hash_const;
use starknet_ibc_apps::transfer::ERC20Contract;
use starknet_ibc_apps::transfer::TokenTransferComponent::{
    TransferInitializerImpl, TransferReaderImpl, TransferWriterImpl, TokenTransferQuery
};
use starknet_ibc_apps::transfer::TokenTransferComponent;
use starknet_ibc_core::router::{AppContract, AppContractTrait};
use starknet_ibc_testkit::configs::{TransferAppConfigTrait, TransferAppConfig};
use starknet_ibc_testkit::dummies::CLASS_HASH;
use starknet_ibc_testkit::dummies::{
    AMOUNT, SUPPLY, OWNER, SN_USER, CS_USER, NAME, SYMBOL, COSMOS, STARKNET, HOSTED_DENOM,
    EMPTY_MEMO
};
use starknet_ibc_testkit::event_spy::{TransferEventSpyExt, ERC20EventSpyExt};
use starknet_ibc_testkit::handles::{ERC20Handle, AppHandle};
use starknet_ibc_testkit::mocks::MockTransferApp;
use starknet_ibc_testkit::setup::SetupImpl;
use starknet_ibc_testkit::utils::call_contract;
use starknet_ibc_utils::ComputeKey;

type ComponentState = TokenTransferComponent::ComponentState<MockTransferApp::ContractState>;

fn COMPONENT_STATE() -> ComponentState {
    TokenTransferComponent::component_state_for_testing()
}

fn setup_component() -> ComponentState {
    let mut state = COMPONENT_STATE();
    state.initializer(CLASS_HASH());
    state
}

fn setup() -> (AppContract, ERC20Contract, TransferAppConfig, EventSpy) {
    let mut cfg = TransferAppConfigTrait::default();

    let (ics20, erc20) = SetupImpl::setup_transfer("MockTransferApp");

    cfg.set_native_denom(erc20.address);

    let mut spy = spy_events();

    (ics20, erc20, cfg, spy)
}

#[test]
fn test_init_state() {
    let state = setup_component();
    let class_hash = state.read_erc20_class_hash();
    assert_eq!(class_hash, CLASS_HASH());
}

#[test]
#[should_panic(expected: 'ICS20: erc20 class hash is 0')]
fn test_missing_class_hash() {
    let mut state = setup_component();
    state.write_erc20_class_hash(class_hash_const::<0>());
    state.read_erc20_class_hash();
}

#[test]
#[should_panic(expected: 'ICS20: salt is 0')]
fn test_missing_salt() {
    let mut state = setup_component();
    state.write_salt(0);
    state.read_salt();
}

#[test]
#[should_panic(expected: 'ICS20: missing token address')]
fn test_missing_ibc_token_address() {
    let state = setup_component();
    state.ibc_token_address(0);
}

#[test]
fn test_escrow_ok() {
    let (ics20, mut erc20, cfg, mut spy) = setup();

    start_cheat_caller_address(ics20.address, SN_USER());

    // User approves the amount of allowance for the `TransferApp` contract.
    erc20.approve(SN_USER(), ics20.address, cfg.amount);

    let msg_transfer = cfg.dummy_msg_transfer(cfg.native_denom.clone(), CS_USER());

    call_contract(ics20.address, selector!("send_transfer_internal"), @msg_transfer);

    // Assert the `SendEvent` emitted.
    spy
        .assert_send_event(
            ics20.address, SN_USER(), CS_USER(), cfg.native_denom.clone(), cfg.amount
        );

    // Check the balance of the sender.
    erc20.assert_balance(SN_USER(), SUPPLY - cfg.amount);

    // Check the balance of the transfer contract.
    erc20.assert_balance(ics20.address, cfg.amount);
}

#[test]
fn test_unescrow_ok() {
    let (ics20, mut erc20, cfg, mut spy) = setup();

    start_cheat_caller_address(ics20.address, SN_USER());

    // User approves the amount of allowance for the `TransferApp` contract.
    erc20.approve(SN_USER(), ics20.address, cfg.amount);

    let msg_transfer = cfg.dummy_msg_transfer(cfg.native_denom.clone(), CS_USER());

    call_contract(ics20.address, selector!("send_transfer_internal"), @msg_transfer);

    spy.drop_all_events();

    start_cheat_caller_address(ics20.address, OWNER());

    let prefixed_denom = cfg.prefix_native_denom();

    let recv_packet = cfg.dummy_incoming_packet(prefixed_denom.clone(), COSMOS(), STARKNET());

    // Submit a `RecvPacket` to the `TransferApp` contract.
    ics20.on_recv_packet(recv_packet);

    // Assert the `RecvEvent` emitted.
    spy.assert_recv_event(ics20.address, CS_USER(), SN_USER(), prefixed_denom, cfg.amount, true);

    erc20.assert_balance(ics20.address, 0);

    // Check the balance of the recipient.
    erc20.assert_balance(SN_USER(), SUPPLY);
}

#[test]
fn test_mint_ok() {
    let (ics20, _, cfg, mut spy) = setup();

    let recv_packet = cfg.dummy_incoming_packet(cfg.hosted_denom.clone(), COSMOS(), STARKNET());

    // Submit a `RecvPacket`, which will create a new ERC20 contract.
    ics20.on_recv_packet(recv_packet.clone());

    let prefixed_denom = cfg.prefix_hosted_denom();

    let token_address = ics20.ibc_token_address(prefixed_denom.key());

    // Assert the `CreateTokenEvent` emitted.
    spy.assert_create_token_event(ics20.address, NAME(), SYMBOL(), token_address, cfg.amount);

    // Assert the `RecvEvent` emitted.
    spy
        .assert_recv_event(
            ics20.address, CS_USER(), SN_USER(), prefixed_denom.clone(), cfg.amount, true
        );

    let erc20: ERC20Contract = token_address.into();

    // Assert if the trasfer happens from the ICS20 address.
    spy.assert_transfer_event(erc20.address, ics20.address, SN_USER(), cfg.amount);

    spy.drop_all_events();

    // Submit another `RecvPacket`, which will mint the amount of tokens.
    ics20.on_recv_packet(recv_packet);

    // Assert the `RecvEvent` emitted.
    spy
        .assert_recv_event(
            ics20.address, CS_USER(), SN_USER(), prefixed_denom.clone(), cfg.amount, true
        );

    // Check the balance of the receiver.
    erc20.assert_balance(SN_USER(), cfg.amount * 2);

    // Check the total supply of the ERC20 contract.
    erc20.assert_total_supply(cfg.amount * 2);
}

#[test]
fn test_burn_ok() {
    let (ics20, _, cfg, mut spy) = setup();

    let recv_packet = cfg.dummy_incoming_packet(cfg.hosted_denom.clone(), COSMOS(), STARKNET());

    // Submit a `RecvPacket`, which will create a new ERC20 contract.
    ics20.on_recv_packet(recv_packet);

    let prefixed_denom = cfg.prefix_hosted_denom();

    let token_address = ics20.ibc_token_address(prefixed_denom.key());

    let mut erc20: ERC20Contract = token_address.into();

    spy.drop_all_events();

    start_cheat_caller_address(ics20.address, SN_USER());

    // User approves the amount of allowance for the `TransferApp` contract.
    erc20.approve(SN_USER(), ics20.address, cfg.amount);

    let msg_transfer = cfg.dummy_msg_transfer(prefixed_denom.clone(), CS_USER());

    call_contract(ics20.address, selector!("send_transfer_internal"), @msg_transfer);

    // Assert the `SendEvent` emitted.
    spy.assert_send_event(ics20.address, SN_USER(), CS_USER(), prefixed_denom, cfg.amount);

    // Check the balance of the sender.
    erc20.assert_balance(SN_USER(), 0);

    // Check the balance of the `TransferApp` contract.
    erc20.assert_balance(ics20.address, 0);

    // Check the total supply of the ERC20 contract.
    erc20.assert_total_supply(0);
}

#[test]
#[should_panic(expected: 'ICS20: missing token address')]
fn test_burn_non_existence_ibc_token() {
    let state = setup_component();
    state.burn_validate(SN_USER(), HOSTED_DENOM(), AMOUNT, EMPTY_MEMO());
}
