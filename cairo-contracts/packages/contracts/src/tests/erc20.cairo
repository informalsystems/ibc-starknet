use openzeppelin_testing::{EventSpyQueue, spy_events};
use snforge_std::start_cheat_caller_address;
use starknet_ibc_apps::transfer::{ERC20Contract, ERC20ContractTrait};
use starknet_ibc_testkit::dummies::{AMOUNT, OWNER, SN_USER, ZERO};
use starknet_ibc_testkit::event_spy::{ERC20EventSpyExt, ERC20EventSpyExtImpl};
use starknet_ibc_testkit::handles::ERC20Handle;
use starknet_ibc_testkit::setup::SetupImpl;

fn setup() -> (ERC20Contract, EventSpyQueue) {
    let setup = SetupImpl::default();
    let erc20 = SetupImpl::deploy_erc20(@setup, OWNER());
    let spy = spy_events();
    (erc20, spy)
}

#[test]
fn test_deploy_erc20_ok() {
    setup();
}

#[test]
fn test_erc20_mint_ok() {
    let (mut erc20, mut spy) = setup();
    start_cheat_caller_address(erc20.address, OWNER());
    erc20.mint(AMOUNT);
    spy.assert_transfer_event(erc20.address, ZERO(), OWNER(), AMOUNT);
    erc20.assert_balance(OWNER(), AMOUNT);
    erc20.assert_total_supply(AMOUNT);
}

#[test]
fn test_erc20_burn_ok() {
    let (mut erc20, mut spy) = setup();
    start_cheat_caller_address(erc20.address, OWNER());
    erc20.mint(AMOUNT);
    erc20.burn(AMOUNT);
    spy.assert_transfer_event(erc20.address, OWNER(), ZERO(), AMOUNT);
    erc20.assert_balance(OWNER(), 0);
    erc20.assert_total_supply(0);
}

#[test]
#[should_panic(expected: 'Caller is not the owner')]
fn test_erc20_unauthorized_mint() {
    let (mut erc20, _) = setup();
    erc20.mint(AMOUNT);
}

#[test]
#[should_panic(expected: 'Caller is not the owner')]
fn test_erc20_unauthorized_burn() {
    let (mut erc20, _) = setup();
    start_cheat_caller_address(erc20.address, OWNER());
    erc20.mint(AMOUNT);
    start_cheat_caller_address(erc20.address, SN_USER());
    erc20.burn(AMOUNT);
}

#[test]
#[should_panic(expected: 'ERC20: insufficient allowance')]
fn test_erc20_transfer_without_user_approval() {
    let (mut erc20, _) = setup();
    start_cheat_caller_address(erc20.address, OWNER());
    erc20.mint(AMOUNT);
    erc20.transfer_from(OWNER(), SN_USER(), AMOUNT);
    erc20.transfer_from(SN_USER(), OWNER(), AMOUNT);
}

#[test]
#[should_panic(expected: 'ERC20: minting amount is zero')]
fn test_erc20_mint_zero_amount() {
    let (mut erc20, _) = setup();
    start_cheat_caller_address(erc20.address, OWNER());
    erc20.mint(0);
}

#[test]
#[should_panic(expected: 'ERC20: burning amount is zero')]
fn test_erc20_burn_zero_amount() {
    let (mut erc20, _) = setup();
    start_cheat_caller_address(erc20.address, OWNER());
    erc20.mint(AMOUNT);
    erc20.burn(0);
}
