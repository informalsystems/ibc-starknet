use starknet::class_hash::class_hash_const;
use starknet::contract_address_const;
use starknet::{ContractAddress, ClassHash};
use starknet_ibc_apps::transfer::types::{Denom, Participant, PrefixedDenom, Memo};

pub const SUPPLY: u256 = 2000;
pub const DECIMALS: u8 = 18_u8;
pub const AMOUNT: u256 = 100;
pub const SALT: felt252 = 'SALT';

pub fn NAME() -> ByteArray {
    "UATOM"
}

pub fn SYMBOL() -> ByteArray {
    "IBC/UATOM"
}

pub fn PUBKEY() -> ContractAddress {
    contract_address_const::<0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7>()
}

pub fn OWNER() -> ContractAddress {
    contract_address_const::<'OWNER'>()
}

pub fn CLASS_HASH() -> ClassHash {
    class_hash_const::<'ERC20Mintable'>()
}

pub fn STARKNET() -> Participant {
    OWNER().into()
}

pub fn COSMOS() -> Participant {
    let bech32_address: ByteArray = "cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng";
    bech32_address.into()
}

pub fn NATIVE_DENOM() -> PrefixedDenom {
    PrefixedDenom { trace_path: array![], base: Denom::Native(PUBKEY().into()) }
}

pub fn HOSTED_DENOM() -> PrefixedDenom {
    PrefixedDenom { trace_path: array![], base: Denom::Hosted(NAME()) }
}

pub fn EMPTY_MEMO() -> Memo {
    Memo { memo: "" }
}
