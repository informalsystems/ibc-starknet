use core::sha256::compute_sha256_u32_array;
use ibc_utils::bytes::SpanU32IntoArrayU8;
use ibc_utils::numeric::next_power_of_two;
use ibc_utils::sha256::compute_sha256_span_u8;
use protobuf::primitives::numeric::U64AsProtoMessage;
use protobuf::types::message::{
    DecodeContext, DecodeContextImpl, EncodeContext, EncodeContextImpl, ProtoCodecImpl,
    ProtoMessage, ProtoName,
};
use protobuf::types::tag::WireType;

pub const ONE_THIRD: Fraction = Fraction { numerator: 1, denominator: 3 };
pub const TWO_THIRDS: Fraction = Fraction { numerator: 2, denominator: 3 };

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde, starknet::Store)]
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

#[generate_trait]
pub impl MerkleHashImpl of MerkleHashTrait {
    fn empty_hash() -> [u32; 8] {
        compute_sha256_u32_array(array![], 0, 0)
    }

    fn leaf_hash(leaf: Span<u8>) -> [u32; 8] {
        let mut hash_bytes = array![];
        hash_bytes.append(0x00);
        hash_bytes.append_span(leaf);
        compute_sha256_span_u8(hash_bytes.span())
    }

    fn inner_hash(left: [u32; 8], right: [u32; 8]) -> [u32; 8] {
        let mut hash_bytes = array![];
        hash_bytes.append(0x01);

        let left_array_u8: Array<u8> = left.span().into();
        let right_array_u8: Array<u8> = right.span().into();

        hash_bytes.append_span(left_array_u8.span());
        hash_bytes.append_span(right_array_u8.span());
        compute_sha256_span_u8(hash_bytes.span())
    }

    fn hash_byte_vectors(byte_vecs: Span<Span<u8>>) -> [u32; 8] {
        let length: u32 = byte_vecs.len();
        match length {
            0 => Self::empty_hash(),
            1 => Self::leaf_hash(*byte_vecs[0]),
            _ => {
                let split = next_power_of_two(length) / 2;
                let left = Self::hash_byte_vectors(byte_vecs.slice(0, split));
                let right = Self::hash_byte_vectors(byte_vecs.slice(split, length - split));
                Self::inner_hash(left, right)
            },
        }
    }
}
