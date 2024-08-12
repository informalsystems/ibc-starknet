use core::byte_array::ByteArrayTrait;
use core::hash::{HashStateTrait, HashStateExTrait};
use core::poseidon::PoseidonTrait;
use core::poseidon::poseidon_hash_span;
use core::to_byte_array::FormatAsByteArray;
use core::traits::TryInto;
use starknet::ContractAddress;
use starknet_ibc::core::host::errors::HostErrors;
use starknet_ibc::utils::poseidon_hash;

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct ChannelId {
    pub channel_id: ByteArray,
}

pub trait ChannelIdTrait {
    fn new(index: u64) -> ChannelId;
    fn index(self: @ChannelId) -> u64;
    fn validate(self: @ChannelId);
}

pub impl ChannelIdImpl of ChannelIdTrait {
    fn new(index: u64) -> ChannelId {
        let mut channel_id: ByteArray = "";
        channel_id.append(@"channel-");
        channel_id.append(@index.format_as_byte_array(10));
        ChannelId { channel_id }
    }

    fn index(self: @ChannelId) -> u64 {
        let mut i = self.channel_id.len();
        let mut index: u256 = 0;
        let mut multiplier: u256 = 1;

        loop {
            if i == 8 {
                break;
            }
            let char_byte = self.channel_id.at(i - 1).unwrap();

            assert_numeric(char_byte);

            index += multiplier * (char_byte - 48).into();

            i -= 1;
            multiplier *= 10;
        };

        index.try_into().unwrap()
    }

    fn validate(self: @ChannelId) {
        let channel_id_len = self.channel_id.len();

        assert(channel_id_len > 8, HostErrors::INVALID_IDENTIFIER_LENGTH);
        assert(channel_id_len <= 64, HostErrors::INVALID_IDENTIFIER_LENGTH);

        let mut expected_channel_id: ByteArray = "channel-";

        expected_channel_id.append(@self.index().format_as_byte_array(10));

        let self_hash = poseidon_hash(self);

        let expected_hash = poseidon_hash(@expected_channel_id);

        assert(self_hash == expected_hash, HostErrors::INVALID_CHANNEL_ID);
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct PortId {
    pub port_id: ByteArray,
}

pub trait PortIdTrait {
    fn validate(self: @PortId, port_id_hash: felt252);
}

pub impl PortIdImpl of PortIdTrait {
    fn validate(self: @PortId, port_id_hash: felt252) {
        let port_id_len = self.port_id.len();

        assert(port_id_len > 2, HostErrors::INVALID_IDENTIFIER_LENGTH);
        assert(port_id_len <= 128, HostErrors::INVALID_IDENTIFIER_LENGTH);

        assert(poseidon_hash(self) == port_id_hash, HostErrors::INVALID_PORT_ID);
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct Sequence {
    pub sequence: u64,
}

pub(crate) fn assert_numeric(char_bytes: u8) {
    assert(char_bytes >= 48 && char_bytes <= 57, HostErrors::INVALID_IDENTIFIER_INDEX);
}

#[cfg(test)]
mod tests {
    use super::{PortId, ChannelId, ChannelIdTrait};

    #[test]
    fn test_channel_id_validate() {
        let channel_id = ChannelId { channel_id: "channel-0" };
        channel_id.validate();

        let channel_id = ChannelId { channel_id: "channel-18446744073709551615" };
        channel_id.validate();
    }
}
