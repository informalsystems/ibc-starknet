use openzeppelin_testing::events::EventSpyExt;
use snforge_std::cheatcodes::events::EventSpy;
use snforge_std::spy_events;
use starknet_ibc_apps::transfer::ERC20Contract;
use starknet_ibc_apps::transfer::TokenTransferComponent::{
    TransferInitializerImpl, TransferReaderImpl
};
use starknet_ibc_apps::transfer::TokenTransferComponent;
use starknet_ibc_testkit::configs::{TransferAppConfigTrait, TransferAppConfig};
use starknet_ibc_testkit::dummies::CLASS_HASH;
use starknet_ibc_testkit::dummies::{SUPPLY, OWNER, NAME, SYMBOL, COSMOS, STARKNET};
use starknet_ibc_testkit::event_spy::TransferEventSpyExt;
use starknet_ibc_testkit::handles::{ERC20Handle, AppHandle, AppContract};
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
fn test_escrow_ok() {
    let (ics20, mut erc20, cfg, mut spy) = setup();

    // Owner approves the amount of allowance for the `TransferApp` contract.
    erc20.approve(OWNER(), ics20.address, cfg.amount);

    let msg_transfer = cfg.dummy_msg_transfer(cfg.native_denom.clone(), STARKNET(), COSMOS());

    call_contract(ics20.address, selector!("send_transfer_internal"), @msg_transfer);

    // Assert the `SendEvent` emitted.
    spy
        .assert_send_event(
            ics20.address, STARKNET(), COSMOS(), cfg.native_denom.clone(), cfg.amount
        );

    // Check the balance of the sender.
    erc20.assert_balance(OWNER(), SUPPLY - cfg.amount);

    // Check the balance of the transfer contract.
    erc20.assert_balance(ics20.address, cfg.amount);
}

#[test]
fn test_unescrow_ok() {
    let (ics20, mut erc20, cfg, mut spy) = setup();

    // Owner approves the amount of allowance for the `TransferApp` contract.
    erc20.approve(OWNER(), ics20.address, cfg.amount);

    let msg_transfer = cfg.dummy_msg_transfer(cfg.native_denom.clone(), STARKNET(), COSMOS());

    call_contract(ics20.address, selector!("send_transfer_internal"), @msg_transfer);

    spy.drop_all_events();

    let prefixed_denom = cfg.prefix_native_denom();

    let recv_packet = cfg.dummy_packet(prefixed_denom.clone(), COSMOS(), STARKNET());

    // Submit a `RecvPacket` to the `TransferApp` contract.
    ics20.on_recv_packet(recv_packet);

    // Assert the `RecvEvent` emitted.
    spy.assert_recv_event(ics20.address, COSMOS(), STARKNET(), prefixed_denom, cfg.amount, true);

    erc20.assert_balance(ics20.address, 0);

    // Check the balance of the recipient.
    erc20.assert_balance(OWNER(), SUPPLY);
}

#[test]
fn test_mint_ok() {
    let (ics20, _, cfg, mut spy) = setup();

    let recv_packet = cfg.dummy_packet(cfg.hosted_denom.clone(), COSMOS(), STARKNET());

    // Submit a `RecvPacket`, which will create a new ERC20 contract.
    ics20.on_recv_packet(recv_packet.clone());

    let prefixed_denom = cfg.prefix_hosted_denom();

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
}

#[test]
fn test_burn_ok() {
    let (ics20, _, cfg, mut spy) = setup();

    let recv_packet = cfg.dummy_packet(cfg.hosted_denom.clone(), COSMOS(), STARKNET());

    // Submit a `RecvPacket`, which will create a new ERC20 contract.
    ics20.on_recv_packet(recv_packet);

    let prefixed_denom = cfg.prefix_hosted_denom();

    let token_address = ics20.ibc_token_address(prefixed_denom.key()).unwrap();

    let erc20: ERC20Contract = token_address.into();

    spy.drop_all_events();

    let msg_transfer = cfg.dummy_msg_transfer(prefixed_denom.clone(), STARKNET(), COSMOS());

    call_contract(ics20.address, selector!("send_transfer_internal"), @msg_transfer);

    // Assert the `SendEvent` emitted.
    spy.assert_send_event(ics20.address, STARKNET(), COSMOS(), prefixed_denom, cfg.amount);

    // Check the balance of the sender.
    erc20.assert_balance(OWNER(), 0);

    // Check the balance of the `TransferApp` contract.
    erc20.assert_balance(ics20.address, 0);

    // Chekck the total supply of the ERC20 contract.
    erc20.assert_total_supply(0);
}
