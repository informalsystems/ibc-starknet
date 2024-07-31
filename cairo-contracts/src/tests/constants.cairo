use core::serde::Serde;
use starknet::ContractAddress;
use starknet::contract_address_const;

pub(crate) const SUPPLY: u256 = 2000;
pub(crate) const DECIMALS: u8 = 18_u8;
pub(crate) const AMOUNT: u256 = 100;
pub(crate) const SALT: felt252 = 'SALT';

pub(crate) fn NAME() -> ByteArray {
    "UATOM"
}

pub(crate) fn SYMBOL() -> ByteArray {
    "IBC/UATOM"
}

pub(crate) fn PUBKEY() -> ContractAddress {
    contract_address_const::<0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7>()
}

pub(crate) fn OWNER() -> ContractAddress {
    contract_address_const::<'OWNER'>()
}

pub(crate) fn RECIPIENT() -> ContractAddress {
    contract_address_const::<'RECIPIENT'>()
}

