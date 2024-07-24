use starknet::ContractAddress;
use starknet::contract_address_const;

pub(crate) const TOKEN_NAME: felt252 = 'ETH';
pub(crate) const DECIMALS: u8 = 18_u8;
pub(crate) const SUPPLY: u256 = 2000;
pub(crate) const SALT: felt252 = 'SALT';
pub(crate) const OWNER: felt252 = 'OWNER';
pub(crate) const PUBKEY: felt252 =
    0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7;

pub(crate) fn owner() -> ContractAddress {
    contract_address_const::<OWNER>()
}

pub(crate) fn pubkey() -> ContractAddress {
    contract_address_const::<PUBKEY>()
}

