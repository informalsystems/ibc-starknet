use core::byte_array::ByteArrayTrait;
use core::hash::{HashStateTrait, HashStateExTrait};
use core::poseidon::PoseidonTrait;
use core::poseidon::poseidon_hash_span;
use core::to_byte_array::FormatAsByteArray;
use core::traits::TryInto;
use starknet::ContractAddress;
use starknet::Store;
use starknet_ibc::core::host::errors::HostErrors;
use starknet_ibc::utils::ComputeKeyTrait;

#[derive(Clone, Debug, Drop, PartialEq, Eq, Serde, Store)]
pub struct ChannelId {
    pub channel_id: ByteArray,
}

impl ChannelIdKey of ComputeKeyTrait<ChannelId> {
    fn compute_key(self: @ChannelId) -> felt252 {
        let mut serialized_channel_id: Array<felt252> = ArrayTrait::new();
        Serde::serialize(self, ref serialized_channel_id);
        PoseidonTrait::new().update(poseidon_hash_span(serialized_channel_id.span())).finalize()
    }
}

pub trait ChannelIdTrait {
    fn new(index: u64) -> ChannelId;
    fn index(self: @ChannelId) -> u64;
    fn validate(self: @ChannelId);
    fn abbr(self: @ChannelId) -> ByteArray;
}

pub impl ChannelIdImpl of ChannelIdTrait {
    fn new(index: u64) -> ChannelId {
        let mut channel_id: ByteArray = "";
        channel_id.append(@"channel-");
        channel_id.append(@index.format_as_byte_array(10));
        ChannelId { channel_id }
    }

    fn index(self: @ChannelId) -> u64 {
        let mut i = self.channel_id.len() - 1;
        let mut j: u64 = 0;
        let mut index: u64 = 0;

        loop {
            if i == 8 {
                break;
            }
            let char_byte = self.channel_id.at(i).unwrap();
            index += j * 10 * (char_byte - 48).into();
            i -= 1;
            j += 1;
        };

        index
    }

    fn validate(self: @ChannelId) {
        let channel_id_len = self.channel_id.len();

        assert(channel_id_len > 8, HostErrors::INVALID_IDENTIFIER_LENGTH);
        assert(channel_id_len < 32, HostErrors::INVALID_IDENTIFIER_LENGTH);

        let prefix: ByteArray = "channel-";

        let mut i = 0;

        loop {
            if i == channel_id_len - 1 {
                break;
            }

            let char_byte = self.channel_id.at(i).unwrap();

            validate_char(char_byte);

            if i <= 7 {
                assert(char_byte == prefix.at(i).unwrap(), HostErrors::INVALID_IDENTIFIER_PREFIX);
            } else {
                /// Checks if the index starts with 0 it does not contain any leading zeros.
                if i == 8 && char_byte == 48 {
                    assert(i == channel_id_len - 1, HostErrors::INVALID_IDENTIFIER_INDEX);
                }
                assert_numeric(char_byte);
            }
        }
    }

    fn abbr(self: @ChannelId) -> ByteArray {
        let mut abbr: ByteArray = "";
        abbr.append(@"ch");
        abbr.append(@self.index().format_as_byte_array(10));
        abbr
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Eq, Serde, Store)]
pub struct PortId {
    pub port_id: ByteArray,
}

impl PortIdKey of ComputeKeyTrait<PortId> {
    fn compute_key(self: @PortId) -> felt252 {
        let mut serialized_port_id: Array<felt252> = ArrayTrait::new();
        Serde::serialize(self, ref serialized_port_id);
        PoseidonTrait::new().update(poseidon_hash_span(serialized_port_id.span())).finalize()
    }
}

pub trait PortIdTrait {
    fn validate(self: @PortId);
    fn transfer() -> PortId;
}

pub impl PortIdImpl of PortIdTrait {
    fn validate(self: @PortId) {
        let port_id_len = self.port_id.len();

        assert(port_id_len > 2, HostErrors::INVALID_IDENTIFIER_LENGTH);
        assert(port_id_len < 32, HostErrors::INVALID_IDENTIFIER_LENGTH);

        let mut i = 0;

        loop {
            if i == port_id_len - 1 {
                break;
            }

            let char_byte = self.port_id.at(i).unwrap();

            validate_char(char_byte);
        }
    }

    fn transfer() -> PortId {
        PortId { port_id: "transfer", }
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Eq, Serde, Store)]
pub struct Sequence {
    pub sequence: u64,
}

/// Validates if the given byte is a valid identifier character.
pub(crate) fn validate_char(char_bytes: u8) {
    assert(char_bytes != 47, HostErrors::INVALID_IDENTIFIER_CHAR); // '/'
    assert(char_bytes >= 33, HostErrors::INVALID_IDENTIFIER_CHAR); // Non-printable ASCII characters
}

pub(crate) fn assert_numeric(char_bytes: u8) {
    assert(char_bytes >= 48 && char_bytes <= 57, HostErrors::INVALID_IDENTIFIER_CHAR);
}
