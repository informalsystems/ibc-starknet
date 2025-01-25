use openzeppelin_testing::deploy;
use openzeppelin_token::erc20::ERC20ABIDispatcherTrait;
use snforge_std::{ContractClass, start_cheat_caller_address};
use starknet::ContractAddress;
use starknet_ibc_apps::transfer::{ERC20Contract, ERC20ContractTrait};
use starknet_ibc_testkit::dummies::{NAME, SYMBOL, SUPPLY, OWNER, USER};

#[generate_trait]
pub impl ERC20HandleImpl of ERC20Handle {
    fn deploy(contract_class: ContractClass) -> ERC20Contract {
        deploy(contract_class, dummy_erc20_call_data()).into()
    }

    fn approve(
        ref self: ERC20Contract, owner: ContractAddress, spender: ContractAddress, amount: u256
    ) {
        start_cheat_caller_address(self.address, owner);
        self.dispatcher().approve(spender, amount);
        start_cheat_caller_address(self.address, spender);
    }

    fn assert_balance(self: @ERC20Contract, account: ContractAddress, expected: u256) {
        let balance = self.balance_of(account);
        assert(balance == expected, 'balance mismatch');
    }

    fn assert_total_supply(self: @ERC20Contract, expected: u256) {
        let total_supply = self.total_supply();
        assert(total_supply == expected, 'total supply mismatch');
    }
}

pub(crate) fn dummy_erc20_call_data() -> Array<felt252> {
    let mut call_data: Array<felt252> = array![];
    Serde::serialize(@NAME(), ref call_data);
    Serde::serialize(@SYMBOL(), ref call_data);
    Serde::serialize(@SUPPLY, ref call_data);
    Serde::serialize(@USER(), ref call_data);
    Serde::serialize(@OWNER(), ref call_data);
    call_data
}
