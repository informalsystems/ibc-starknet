use core::num::traits::Zero;
use core::starknet::SyscallResultTrait;
use openzeppelin_token::erc20::{ERC20ABIDispatcher, ERC20ABIDispatcherTrait};
use starknet::syscalls::{call_contract_syscall, deploy_syscall};
use starknet::{ClassHash, ContractAddress, contract_address_const};

#[derive(Copy, Debug, Drop, Serde)]
pub struct ERC20Contract {
    pub address: ContractAddress,
}

impl ContractAddressIntoTokenAddr of Into<ContractAddress, ERC20Contract> {
    fn into(self: ContractAddress) -> ERC20Contract {
        ERC20Contract { address: self }
    }
}

impl ERC20ContractIntoFelt252 of Into<ERC20Contract, felt252> {
    fn into(self: ERC20Contract) -> felt252 {
        self.address.into()
    }
}

#[generate_trait]
pub impl ERC20ContractImpl of ERC20ContractTrait {
    fn dispatcher(self: @ERC20Contract) -> ERC20ABIDispatcher {
        ERC20ABIDispatcher { contract_address: *self.address }
    }

    fn create(
        class_hash: ClassHash,
        salt: felt252,
        name: ByteArray,
        symbol: ByteArray,
        decimals: u8,
        owner: ContractAddress,
    ) -> ERC20Contract {
        let mut call_data = array![];

        (name, symbol, decimals, owner).serialize(ref call_data);

        let (address, _) = deploy_syscall(class_hash, salt, call_data.span(), false)
            .unwrap_syscall();

        ERC20Contract { address }
    }

    fn transfer(self: @ERC20Contract, recipient: ContractAddress, amount: u256) -> bool {
        self.dispatcher().transfer(recipient, amount)
    }

    fn transfer_from(
        self: @ERC20Contract, sender: ContractAddress, recipient: ContractAddress, amount: u256,
    ) -> bool {
        self.dispatcher().transfer_from(sender, recipient, amount)
    }

    fn mint(self: @ERC20Contract, amount: u256) {
        let mut calldata = array![];
        amount.serialize(ref calldata);
        call_contract_syscall(*self.address, selector!("mint"), calldata.span()).unwrap_syscall();
    }

    fn burn(self: @ERC20Contract, amount: u256) {
        let mut calldata = array![];
        amount.serialize(ref calldata);
        call_contract_syscall(*self.address, selector!("burn"), calldata.span()).unwrap_syscall();
    }

    fn balance_of(self: @ERC20Contract, from_account: ContractAddress) -> u256 {
        self.dispatcher().balance_of(from_account)
    }

    fn total_supply(self: @ERC20Contract) -> u256 {
        self.dispatcher().total_supply()
    }
}

impl ERC20ContractZero of Zero<ERC20Contract> {
    fn zero() -> ERC20Contract {
        ERC20Contract { address: contract_address_const::<0>() }
    }

    fn is_zero(self: @ERC20Contract) -> bool {
        self.address.is_zero()
    }

    fn is_non_zero(self: @ERC20Contract) -> bool {
        !self.is_zero()
    }
}
