use core::byte_array::ByteArrayTrait;
use core::num::traits::CheckedAdd;
use core::num::traits::Zero;
use core::to_byte_array::FormatAsByteArray;
use core::traits::TryInto;
use starknet_ibc_core::host::errors::HostErrors;
use starknet_ibc_utils::{ValidateBasic, ComputeKey, poseidon_hash};

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct ClientId {
    pub client_type: felt252,
    pub sequence: u64,
}

#[generate_trait]
pub impl ClientIdImpl of ClientIdTrait {
    fn new(client_type: felt252, sequence: u64) -> ClientId {
        ClientId { client_type, sequence }
    }

    fn validate(self: @ClientId, client_id_hash: felt252) {}
}

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct ChannelId {
    pub channel_id: ByteArray,
}

#[generate_trait]
pub impl ChannelIdImpl of ChannelIdTrait {
    fn new(sequence: u64) -> ChannelId {
        let mut channel_id: ByteArray = "";
        channel_id.append(@"channel-");
        channel_id.append(@sequence.format_as_byte_array(10));
        ChannelId { channel_id }
    }

    fn sequence(self: @ChannelId) -> u64 {
        let mut i = self.channel_id.len();
        let mut sequence: u256 = 0;
        let mut multiplier: u256 = 1;

        loop {
            if i == 8 {
                break;
            }
            let char_byte = self.channel_id.at(i - 1).unwrap();

            assert_numeric(char_byte);

            sequence += multiplier * (char_byte - 48).into();

            i -= 1;
            multiplier *= 10;
        };

        sequence.try_into().unwrap()
    }

    fn validate(self: @ChannelId) {
        let channel_id_len = self.channel_id.len();

        assert(channel_id_len > 8, HostErrors::INVALID_IDENTIFIER_LENGTH);
        assert(channel_id_len <= 64, HostErrors::INVALID_IDENTIFIER_LENGTH);

        let mut expected_channel_id: ByteArray = "channel-";

        expected_channel_id.append(@self.sequence().format_as_byte_array(10));

        let self_hash = poseidon_hash(self);

        let expected_hash = poseidon_hash(@expected_channel_id);

        assert(self_hash == expected_hash, HostErrors::INVALID_CHANNEL_ID);
    }
}

pub impl ChannelIdIntoByteArray of Into<ChannelId, ByteArray> {
    fn into(self: ChannelId) -> ByteArray {
        self.channel_id
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct PortId {
    pub port_id: ByteArray,
}

#[generate_trait]
pub impl PortIdImpl of PortIdTrait {
    /// Constructs a new port identifier from a byte array with basic
    /// validation.
    fn new(port_id: ByteArray) -> PortId {
        let port_id = PortId { port_id };
        port_id.validate_basic();
        port_id
    }

    fn validate(self: @PortId, port_id_hash: felt252) {
        self.validate_basic();
        assert(self.key() == port_id_hash, HostErrors::INVALID_PORT_ID);
    }
}

impl PortIdValidateBasic of ValidateBasic<PortId> {
    fn validate_basic(self: @PortId) {
        let port_id_len = self.port_id.len();
        assert(port_id_len > 2, HostErrors::INVALID_IDENTIFIER_LENGTH);
        assert(port_id_len <= 128, HostErrors::INVALID_IDENTIFIER_LENGTH);
    }
}

impl PortIdKeyImpl of ComputeKey<PortId> {}

pub impl PortIdIntoByteArray of Into<PortId, ByteArray> {
    fn into(self: PortId) -> ByteArray {
        self.port_id
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct Sequence {
    pub sequence: u64,
}

#[generate_trait]
pub impl SequenceImpl of SequenceTrait {
    fn increment(ref self: Sequence) -> Sequence {
        let maybe_next_sequence = self.sequence.checked_add(1);

        match maybe_next_sequence {
            Option::Some(sequence) => Sequence { sequence },
            Option::None => panic!("{}", HostErrors::OVERFLOWED_SEQUENCE)
        }
    }
}

pub impl SequenceZero of Zero<Sequence> {
    fn zero() -> Sequence {
        Sequence { sequence: 0 }
    }

    fn is_zero(self: @Sequence) -> bool {
        self.sequence.is_zero()
    }

    fn is_non_zero(self: @Sequence) -> bool {
        !self.is_zero()
    }
}

pub impl SequencePartialOrd of PartialOrd<@Sequence> {
    fn le(lhs: @Sequence, rhs: @Sequence) -> bool {
        lhs.sequence <= rhs.sequence
    }
    fn ge(lhs: @Sequence, rhs: @Sequence) -> bool {
        lhs.sequence >= rhs.sequence
    }
    fn lt(lhs: @Sequence, rhs: @Sequence) -> bool {
        lhs.sequence < rhs.sequence
    }
    fn gt(lhs: @Sequence, rhs: @Sequence) -> bool {
        lhs.sequence > rhs.sequence
    }
}

pub impl SequenceIntoByteArray of Into<Sequence, ByteArray> {
    fn into(self: Sequence) -> ByteArray {
        self.sequence.format_as_byte_array(10)
    }
}

pub(crate) fn assert_numeric(char_bytes: u8) {
    assert(char_bytes >= 48 && char_bytes <= 57, HostErrors::INVALID_IDENTIFIER_INDEX);
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, ChannelIdTrait};

    #[test]
    fn test_channel_id_validate() {
        let channel_id = ChannelId { channel_id: "channel-0" };
        channel_id.validate();

        let channel_id = ChannelId { channel_id: "channel-18446744073709551615" };
        channel_id.validate();
    }
}
