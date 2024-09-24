use protobuf::types::message::{
    ProtoMessage, ProtoCodecImpl, EncodeContext, DecodeContext, EncodeContextImpl,
    DecodeContextImpl, ProtoName
};
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

    fn decode_raw(ref self: Fraction, ref context: DecodeContext) {
        context.decode_field(1, ref self.numerator);
        context.decode_field(2, ref self.denominator);
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
