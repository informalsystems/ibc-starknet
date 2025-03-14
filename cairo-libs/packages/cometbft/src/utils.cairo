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
