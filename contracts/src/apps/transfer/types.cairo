use core::array::ArrayTrait;
use core::byte_array::ByteArrayTrait;
use core::num::traits::zero::Zero;
use core::serde::Serde;
use core::to_byte_array::FormatAsByteArray;
use core::traits::TryInto;
use openzeppelin::utils::selectors;
use starknet::ContractAddress;
use starknet::Store;
use starknet::contract_address_const;
use starknet::syscalls::call_contract_syscall;
use starknet_ibc::core::types::{PortId, ChannelId, Sequence, Height, Timestamp};

/// Maximum memo length allowed for ICS-20 transfers. This bound corresponds to
/// the `MaximumMemoLength` in the `ibc-go`.
pub(crate) const MAXIMUM_MEMO_LENGTH: u32 = 32768;

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct MsgTransfer {
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub packet_data: PacketData,
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct Packet {
    pub seq_on_a: Sequence,
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub port_id_on_b: PortId,
    pub chan_id_on_b: ChannelId,
    pub data: PacketData,
    pub timeout_height_on_b: Height,
    pub timeout_timestamp_on_b: Timestamp,
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct PacketData {
    pub token: Token,
    pub sender: ContractAddress,
    pub receiver: ContractAddress,
    pub memo: Memo,
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct Token {
    pub denom: Denom,
    pub amount: u256,
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub enum Denom {
    Native: ContractAddress,
    IBC: ByteArray,
}

pub trait DenomTrait {
    fn is_zero(self: @Denom) -> bool;
    fn native(self: @Denom) -> Option<ContractAddress>;
    fn ibc(self: @Denom) -> Option<felt252>;
}

pub impl DenomImpl of DenomTrait {
    fn is_zero(self: @Denom) -> bool {
        match self {
            Denom::Native(contract_address) => contract_address.is_zero(),
            Denom::IBC(byte_array) => byte_array.len() == 0,
        }
    }

    fn native(self: @Denom) -> Option<ContractAddress> {
        match self {
            Denom::Native(contract_address) => Option::Some(*contract_address),
            Denom::IBC(_) => Option::None,
        }
    }

    fn ibc(self: @Denom) -> Option<felt252> {
        match self {
            Denom::Native(_) => Option::None,
            Denom::IBC(byte_array) => bytes_to_felt252(byte_array),
        }
    }
}

pub impl ContractAddressIntoDenom of Into<ContractAddress, Denom> {
    fn into(self: ContractAddress) -> Denom {
        Denom::Native(self)
    }
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct Memo {
    pub memo: ByteArray,
}

fn bytes_to_felt252(bytes: @ByteArray) -> Option<felt252> {
    if bytes.len() == 0 {
        return Option::Some('');
    }

    if bytes.len() > 31 {
        return Option::None(());
    }

    let mut result: felt252 = 0;
    let mut multiplier: felt252 = 1;

    // Iterate through the bytes in reverse order
    let mut i = bytes.len();
    loop {
        if i == 0 {
            break;
        }
        i -= 1;

        let byte_value = bytes.at(i).unwrap();
        result += byte_value.into() * multiplier;
        multiplier *= 0x100; // 256
    };

    Option::Some(result)
}


mod test {
    use super::bytes_to_felt252;

    #[test]
    fn test_empty_string() {
        let bytes = "";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some(0), "Empty string should convert to 0");
    }

    #[test]
    fn test_single_character() {
        let bytes = "A";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some('A'), "Single character conversion failed");
    }

    #[test]
    fn test_multiple_bytes() {
        let bytes = "abc";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some('abc'), "Multiple byte conversion failed");
    }

    #[test]
    fn test_max_bytes() {
        let bytes = "abcdefghijklmnopqrstuvwxyz12345"; // 31 characters
        let result = bytes_to_felt252(@bytes);
        assert!(
            result == Option::Some('abcdefghijklmnopqrstuvwxyz12345'), "Max bytes conversion failed"
        );
    }

    #[test]
    fn test_too_many_bytes() {
        let bytes = "abcdefghijklmnopqrstuvwxyz123456"; // 32 characters
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::None(()), "More than characters should return None");
    }

    #[test]
    fn test_leading_zeros() {
        let bytes = "\0\0ab";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some('ab'), "Leading zeros not handled correctly");
    }

    #[test]
    fn test_all_zeros() {
        let bytes = "\0\0\0\0";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some(0), "All zeros should convert to 0");
    }

    #[test]
    fn test_special_characters() {
        let bytes = "!@#$";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some('!@#$'), "Special characters conversion failed");
    }
}
