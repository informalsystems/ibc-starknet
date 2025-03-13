use starknet::{ClassHash, ContractAddress};
use starknet_ibc_apps::transfer::ERC20Contract;
use starknet_ibc_apps::transfer::types::{Denom, Memo, PacketData, Participant, PrefixedDenom};
use starknet_ibc_core::commitment::{Commitment, compute_packet_commitment};
use starknet_ibc_testkit::dummies::{TIMEOUT_HEIGHT, TIMEOUT_TIMESTAMP};

pub const SUPPLY: u256 = 2000;
pub const DECIMALS_18: u8 = 18_u8;
pub const DECIMAL_ZERO: u8 = 0_u8;
pub const AMOUNT: u256 = 100;
pub const SALT: felt252 = 'SALT';

pub fn NAME() -> ByteArray {
    "UATOM"
}

pub fn SYMBOL() -> ByteArray {
    "IBC/UATOM"
}

pub const fn CLASS_HASH() -> ClassHash {
    'ERC20Mintable'.try_into().unwrap()
}

pub const fn ZERO() -> ContractAddress {
    0.try_into().unwrap()
}

pub const fn ERC20() -> ERC20Contract {
    ERC20Contract {
        address: 0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7
            .try_into()
            .unwrap(),
    }
}

pub const fn OWNER() -> ContractAddress {
    'OWNER'.try_into().unwrap()
}

pub const fn SN_USER() -> ContractAddress {
    'USER'.try_into().unwrap()
}

pub fn CS_USER() -> ByteArray {
    "cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng"
}

pub fn STARKNET() -> Participant {
    SN_USER().into()
}

pub fn COSMOS() -> Participant {
    CS_USER().into()
}

pub fn NATIVE_DENOM() -> PrefixedDenom {
    PrefixedDenom { trace_path: array![], base: Denom::Native(ERC20()) }
}

pub fn HOSTED_DENOM() -> PrefixedDenom {
    PrefixedDenom { trace_path: array![], base: Denom::Hosted(NAME()) }
}

pub fn EMPTY_MEMO() -> Memo {
    Memo { memo: "" }
}

pub fn PACKET_DATA_FROM_SN(token: ERC20Contract) -> PacketData {
    PacketData {
        denom: PrefixedDenom { trace_path: array![], base: Denom::Native(token) },
        amount: AMOUNT,
        sender: STARKNET(),
        receiver: COSMOS(),
        memo: EMPTY_MEMO(),
    }
}

pub fn PACKET_COMMITMENT_ON_SN(token: ERC20Contract) -> Commitment {
    compute_packet_commitment(
        @serde_json::to_byte_array(PACKET_DATA_FROM_SN(token)),
        TIMEOUT_HEIGHT(1000),
        TIMEOUT_TIMESTAMP(1000),
    )
}
