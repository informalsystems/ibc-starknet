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
        let mut fraction = Default::<Fraction>::default();
        if !context.decode_field(1, ref fraction.numerator) {
            return Option::None;
        }
        if !context.decode_field(2, ref fraction.denominator) {
            return Option::None;
        }
        Option::Some(fraction)
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
