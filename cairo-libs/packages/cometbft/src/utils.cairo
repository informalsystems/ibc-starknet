use core::sha256::compute_sha256_byte_array;
use protobuf::primitives::numeric::U64AsProtoMessage;
use protobuf::types::message::{
    DecodeContext, DecodeContextImpl, EncodeContext, EncodeContextImpl, ProtoCodecImpl,
    ProtoMessage, ProtoName,
};
use protobuf::types::tag::WireType;

pub const TWO_THIRDS: Fraction = Fraction { numerator: 2, denominator: 3 };

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub struct Fraction {
    pub numerator: u64,
    pub denominator: u64,
}

impl FractionAsProtoMessage of ProtoMessage<Fraction> {
    fn encode_raw(self: @Fraction, ref context: EncodeContext) {
        context.encode_field(1, self.numerator);
        context.encode_field(2, self.denominator);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Fraction> {
        let numerator = context.decode_field(1)?;
        let denominator = context.decode_field(2)?;
        Option::Some(Fraction { numerator, denominator })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl FractionAsProtoName of ProtoName<Fraction> {
    fn type_url() -> ByteArray {
        "Fraction"
    }
}

pub impl SpanU8TryIntoU256 of TryInto<Span<u8>, u256> {
    /// Decodes as big endian.
    fn try_into(self: Span<u8>) -> Option<u256> {
        // Only allows size 32 to ensure all bytes fit exactly into a `u256`.
        if (self.len() != 32) {
            return Option::None;
        }

        const N256: u128 = 0x100;

        // No loop overhead with manual unrolling.
        let mut ret: u256 = 0;
        ret.high = ret.high * N256 + (*self[0]).into();
        ret.high = ret.high * N256 + (*self[1]).into();
        ret.high = ret.high * N256 + (*self[2]).into();
        ret.high = ret.high * N256 + (*self[3]).into();
        ret.high = ret.high * N256 + (*self[4]).into();
        ret.high = ret.high * N256 + (*self[5]).into();
        ret.high = ret.high * N256 + (*self[6]).into();
        ret.high = ret.high * N256 + (*self[7]).into();
        ret.high = ret.high * N256 + (*self[8]).into();
        ret.high = ret.high * N256 + (*self[9]).into();
        ret.high = ret.high * N256 + (*self[10]).into();
        ret.high = ret.high * N256 + (*self[11]).into();
        ret.high = ret.high * N256 + (*self[12]).into();
        ret.high = ret.high * N256 + (*self[13]).into();
        ret.high = ret.high * N256 + (*self[14]).into();
        ret.high = ret.high * N256 + (*self[15]).into();

        ret.low = ret.low * N256 + (*self[16]).into();
        ret.low = ret.low * N256 + (*self[17]).into();
        ret.low = ret.low * N256 + (*self[18]).into();
        ret.low = ret.low * N256 + (*self[19]).into();
        ret.low = ret.low * N256 + (*self[20]).into();
        ret.low = ret.low * N256 + (*self[21]).into();
        ret.low = ret.low * N256 + (*self[22]).into();
        ret.low = ret.low * N256 + (*self[23]).into();
        ret.low = ret.low * N256 + (*self[24]).into();
        ret.low = ret.low * N256 + (*self[25]).into();
        ret.low = ret.low * N256 + (*self[26]).into();
        ret.low = ret.low * N256 + (*self[27]).into();
        ret.low = ret.low * N256 + (*self[28]).into();
        ret.low = ret.low * N256 + (*self[29]).into();
        ret.low = ret.low * N256 + (*self[30]).into();
        ret.low = ret.low * N256 + (*self[31]).into();

        Option::Some(ret)
    }
}

pub fn u32_8_to_byte_array(u32_8: [u32; 8]) -> ByteArray {
    let mut byte_array = "";
    let mut span = u32_8.span();
    while let Some(elem) = span.pop_front() {
        let word = *elem;
        byte_array.append_byte(((word / 0x1000000) & 0xFF).try_into().unwrap());
        byte_array.append_byte(((word / 0x10000) & 0xFF).try_into().unwrap());
        byte_array.append_byte(((word / 0x100) & 0xFF).try_into().unwrap());
        byte_array.append_byte((word & 0xFF).try_into().unwrap());
    }
    byte_array
}

pub fn u32_8_to_array_u8(u32_8: [u32; 8]) -> Array<u8> {
    let mut array_u8 = ArrayTrait::new();
    let mut span = u32_8.span();
    while let Some(elem) = span.pop_front() {
        let word = *elem;
        array_u8.append(((word / 0x1000000) & 0xFF).try_into().unwrap());
        array_u8.append(((word / 0x10000) & 0xFF).try_into().unwrap());
        array_u8.append(((word / 0x100) & 0xFF).try_into().unwrap());
        array_u8.append((word & 0xFF).try_into().unwrap());
    }
    array_u8
}

pub fn next_power_of_two(num: u32) -> u32 {
    let mut two_power = 1;
    while two_power < num {
        two_power *= 2;
    }
    two_power
}

#[generate_trait]
pub impl MerkleHashImpl of MerkleHashTrait {
    fn empty_hash() -> [u32; 8] {
        compute_sha256_byte_array(@"")
    }

    fn leaf_hash(leaf: @ByteArray) -> [u32; 8] {
        let mut hash_bytes = "";
        hash_bytes.append_byte(0x00);
        hash_bytes.append(leaf);
        compute_sha256_byte_array(@hash_bytes)
    }

    fn inner_hash(left: [u32; 8], right: [u32; 8]) -> [u32; 8] {
        let mut hash_bytes = "";
        hash_bytes.append_byte(0x01);
        hash_bytes.append(@u32_8_to_byte_array(left));
        hash_bytes.append(@u32_8_to_byte_array(right));
        compute_sha256_byte_array(@hash_bytes)
    }

    fn hash_byte_vectors(byte_vecs: Span<ByteArray>) -> [u32; 8] {
        let length: u32 = byte_vecs.len();
        match length {
            0 => Self::empty_hash(),
            1 => Self::leaf_hash(byte_vecs[0]),
            _ => {
                let split = next_power_of_two(length) / 2;
                let left = Self::hash_byte_vectors(byte_vecs.slice(0, split));
                let right = Self::hash_byte_vectors(byte_vecs.slice(split, length));
                Self::inner_hash(left, right)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_power_of_two() {
        assert_eq!(next_power_of_two(0), 1);
        assert_eq!(next_power_of_two(1), 1);
        assert_eq!(next_power_of_two(2), 2);
        assert_eq!(next_power_of_two(3), 4);
        assert_eq!(next_power_of_two(4), 4);
        assert_eq!(next_power_of_two(5), 8);
        assert_eq!(next_power_of_two(6), 8);
        assert_eq!(next_power_of_two(7), 8);
        assert_eq!(next_power_of_two(8), 8);
        assert_eq!(next_power_of_two(9), 16);
    }

    #[test]
    fn test_u32_8_to_byte_array() {
        let u32_8 = [
            0x12345678, 0x9ABCDEF0, 0x12345678, 0x9ABCDEF0, 0x12345678, 0x9ABCDEF0, 0x12345678,
            0x9ABCDEF0,
        ];
        let mut byte_array = "";

        byte_array.append_byte(0x12);
        byte_array.append_byte(0x34);
        byte_array.append_byte(0x56);
        byte_array.append_byte(0x78);
        byte_array.append_byte(0x9A);
        byte_array.append_byte(0xBC);
        byte_array.append_byte(0xDE);
        byte_array.append_byte(0xF0);

        byte_array.append_byte(0x12);
        byte_array.append_byte(0x34);
        byte_array.append_byte(0x56);
        byte_array.append_byte(0x78);
        byte_array.append_byte(0x9A);
        byte_array.append_byte(0xBC);
        byte_array.append_byte(0xDE);
        byte_array.append_byte(0xF0);

        byte_array.append_byte(0x12);
        byte_array.append_byte(0x34);
        byte_array.append_byte(0x56);
        byte_array.append_byte(0x78);
        byte_array.append_byte(0x9A);
        byte_array.append_byte(0xBC);
        byte_array.append_byte(0xDE);
        byte_array.append_byte(0xF0);

        byte_array.append_byte(0x12);
        byte_array.append_byte(0x34);
        byte_array.append_byte(0x56);
        byte_array.append_byte(0x78);
        byte_array.append_byte(0x9A);
        byte_array.append_byte(0xBC);
        byte_array.append_byte(0xDE);
        byte_array.append_byte(0xF0);

        assert_eq!(byte_array, u32_8_to_byte_array(u32_8));
    }

    #[test]
    fn test_u32_8_to_array_u8() {
        let u32_8 = [
            0x12345678, 0x9ABCDEF0, 0x12345678, 0x9ABCDEF0, 0x12345678, 0x9ABCDEF0, 0x12345678,
            0x9ABCDEF0,
        ];
        let mut array = array![];

        array.append(0x12);
        array.append(0x34);
        array.append(0x56);
        array.append(0x78);
        array.append(0x9A);
        array.append(0xBC);
        array.append(0xDE);
        array.append(0xF0);

        array.append(0x12);
        array.append(0x34);
        array.append(0x56);
        array.append(0x78);
        array.append(0x9A);
        array.append(0xBC);
        array.append(0xDE);
        array.append(0xF0);

        array.append(0x12);
        array.append(0x34);
        array.append(0x56);
        array.append(0x78);
        array.append(0x9A);
        array.append(0xBC);
        array.append(0xDE);
        array.append(0xF0);

        array.append(0x12);
        array.append(0x34);
        array.append(0x56);
        array.append(0x78);
        array.append(0x9A);
        array.append(0xBC);
        array.append(0xDE);
        array.append(0xF0);

        assert_eq!(array, u32_8_to_array_u8(u32_8));
    }
}
