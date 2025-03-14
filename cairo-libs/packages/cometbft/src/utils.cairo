use protobuf::types::message::{
    ProtoMessage, ProtoCodecImpl, EncodeContext, DecodeContext, EncodeContextImpl,
    DecodeContextImpl, ProtoName,
};
use protobuf::primitives::numeric::U64AsProtoMessage;
use protobuf::types::tag::WireType;

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

        // Precomputed powers to remove the overhead of repeated multiplications.
        let mut ret: u256 = 0;
        let two_pow_0 = 0x1;
        let two_pow_1 = 0x100;
        let two_pow_2 = 0x10000;
        let two_pow_3 = 0x1000000;
        let two_pow_4 = 0x100000000;
        let two_pow_5 = 0x10000000000;
        let two_pow_6 = 0x1000000000000;
        let two_pow_7 = 0x100000000000000;
        let two_pow_8 = 0x10000000000000000;
        let two_pow_9 = 0x1000000000000000000;
        let two_pow_10 = 0x100000000000000000000;
        let two_pow_11 = 0x10000000000000000000000;
        let two_pow_12 = 0x1000000000000000000000000;
        let two_pow_13 = 0x100000000000000000000000000;
        let two_pow_14 = 0x10000000000000000000000000000;
        let two_pow_15 = 0x1000000000000000000000000000000;

        // No loop overhead with manual unrolling.
        ret.low += (*self[31]).into() * two_pow_0;
        ret.low += (*self[30]).into() * two_pow_1;
        ret.low += (*self[29]).into() * two_pow_2;
        ret.low += (*self[28]).into() * two_pow_3;
        ret.low += (*self[27]).into() * two_pow_4;
        ret.low += (*self[26]).into() * two_pow_5;
        ret.low += (*self[25]).into() * two_pow_6;
        ret.low += (*self[24]).into() * two_pow_7;
        ret.low += (*self[23]).into() * two_pow_8;
        ret.low += (*self[22]).into() * two_pow_9;
        ret.low += (*self[21]).into() * two_pow_10;
        ret.low += (*self[20]).into() * two_pow_11;
        ret.low += (*self[19]).into() * two_pow_12;
        ret.low += (*self[18]).into() * two_pow_13;
        ret.low += (*self[17]).into() * two_pow_14;
        ret.low += (*self[16]).into() * two_pow_15;

        ret.high += (*self[15]).into() * two_pow_0;
        ret.high += (*self[14]).into() * two_pow_1;
        ret.high += (*self[13]).into() * two_pow_2;
        ret.high += (*self[12]).into() * two_pow_3;
        ret.high += (*self[11]).into() * two_pow_4;
        ret.high += (*self[10]).into() * two_pow_5;
        ret.high += (*self[9]).into() * two_pow_6;
        ret.high += (*self[8]).into() * two_pow_7;
        ret.high += (*self[7]).into() * two_pow_8;
        ret.high += (*self[6]).into() * two_pow_9;
        ret.high += (*self[5]).into() * two_pow_10;
        ret.high += (*self[4]).into() * two_pow_11;
        ret.high += (*self[3]).into() * two_pow_12;
        ret.high += (*self[2]).into() * two_pow_13;
        ret.high += (*self[1]).into() * two_pow_14;
        ret.high += (*self[0]).into() * two_pow_15;

        Option::Some(ret)
    }
}
