use core::num::traits::Zero;
use core::starknet::SyscallResultTrait;
use openzeppelin_token::erc20::{ERC20ABIDispatcher, ERC20ABIDispatcherTrait};
use openzeppelin_utils::serde::SerializedAppend;
use starknet::syscalls::deploy_syscall;
use starknet::{ClassHash, ContractAddress};
use starknet_ibc_utils::mintable::{IERC20MintableDispatcher, IERC20MintableDispatcherTrait};

#[derive(Clone, Debug, Drop, Serde)]
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
    fn is_non_zero(self: @ERC20Contract) -> bool {
        self.address.is_non_zero()
    }

    fn dispatcher(self: @ERC20Contract) -> ERC20ABIDispatcher {
        ERC20ABIDispatcher { contract_address: *self.address }
    }

    fn mintable_dispatcher(self: @ERC20Contract) -> IERC20MintableDispatcher {
        IERC20MintableDispatcher { contract_address: *self.address }
    }

    fn create(
        class_hash: ClassHash,
        salt: felt252,
        name: ByteArray,
        symbol: ByteArray,
        amount: u256,
        recipient: ContractAddress,
        owner: ContractAddress
    ) -> ERC20Contract {
        let mut call_data = array![];

        call_data.append_serde(name);
        call_data.append_serde(symbol);
        call_data.append_serde(amount);
        call_data.append_serde(recipient);
        call_data.append_serde(owner);

        let (address, _) = deploy_syscall(class_hash, salt, call_data.span(), false,)
            .unwrap_syscall();

        ERC20Contract { address }
    }

    fn transfer(self: @ERC20Contract, recipient: ContractAddress, amount: u256) -> bool {
        self.dispatcher().transfer(recipient, amount)
    }

    fn transfer_from(
        self: @ERC20Contract, sender: ContractAddress, recipient: ContractAddress, amount: u256
    ) -> bool {
        self.dispatcher().transfer_from(sender, recipient, amount)
    }

    fn mint(self: @ERC20Contract, recipient: ContractAddress, amount: u256) {
        self.mintable_dispatcher().permissioned_mint(recipient, amount)
    }

    fn burn(self: @ERC20Contract, account: ContractAddress, amount: u256) {
        self.mintable_dispatcher().permissioned_burn(account, amount)
    }

    fn balance_of(self: @ERC20Contract, from_account: ContractAddress) -> u256 {
        self.dispatcher().balance_of(from_account)
    }

    fn total_supply(self: @ERC20Contract) -> u256 {
        self.dispatcher().total_supply()
    }
}
