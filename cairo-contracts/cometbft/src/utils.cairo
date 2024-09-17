use protobuf::types::message::{ProtoMessage, ProtoCodecImpl};
use protobuf::types::tag::WireType;

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub struct Fraction {
    pub numerator: u64,
    pub denominator: u64,
}

impl FractionAsProtoMessage of ProtoMessage<Fraction> {
    fn encode_raw(self: @Fraction, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.numerator, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.denominator, ref output);
    }

    fn decode_raw(ref value: Fraction, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;

        ProtoCodecImpl::decode_length_delimited_raw(
            1, ref value.numerator, serialized, ref index, bound
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            2, ref value.denominator, serialized, ref index, bound
        );

        assert(index == bound, 'invalid length for Fraction');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }

    fn type_url() -> ByteArray {
        "Fraction"
    }
}
