use protobuf::types::message::{
    ProtoMessage, ProtoOneof, DecodeContext, EncodeContext, EncodeContextTrait, DecodeContextTrait,
    ProtoCodecImpl,
};
use protobuf::types::tag::{WireType, ProtobufTag};
use protobuf::primitives::numeric::U64AsProtoMessage;
use protobuf::primitives::array::ByteArrayAsProtoMessage;

#[derive(Drop, Debug, Default, PartialEq)]
pub struct Basic {
    oneof: BasicOneof,
}

pub impl BasicAsProtoMessage of ProtoMessage<Basic> {
    fn encode_raw(self: @Basic, ref context: EncodeContext) {
        context.encode_oneof(self.oneof);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Basic> {
        let oneof = context.decode_oneof()?;
        Option::Some(Basic { oneof })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Drop, Debug, Default, PartialEq)]
pub enum BasicOneof {
    #[default]
    Int: u64,
    String: ByteArray,
}

pub impl BasicOneofAsProtoOneof of ProtoOneof<BasicOneof> {
    fn encode_raw(self: @BasicOneof, ref context: EncodeContext) -> ProtobufTag {
        match self {
            BasicOneof::Int(n) => {
                n.encode_raw(ref context);
                let wire_type = ProtoMessage::<u64>::wire_type();
                ProtobufTag { field_number: 1, wire_type }
            },
            BasicOneof::String(str) => {
                str.encode_raw(ref context);
                let wire_type = ProtoMessage::<ByteArray>::wire_type();
                ProtobufTag { field_number: 2, wire_type }
            },
        }
    }

    fn decode_raw(ref context: DecodeContext, tag: u8) -> Option<BasicOneof> {
        if tag == 1 {
            let value = context.decode_raw()?;
            Option::Some(BasicOneof::Int(value))
        } else if tag == 2 {
            let value = context.decode_raw()?;
            Option::Some(BasicOneof::String(value))
        } else {
            Option::None
        }
    }
}


fn oneof_roundtrip_fixture(oneof: BasicOneof) {
    let basic = Basic { oneof };
    let encoded = ProtoCodecImpl::encode(@basic);
    let decoded = ProtoCodecImpl::decode::<Basic>(@encoded).unwrap();
    assert_eq!(basic, decoded);
}

#[test]
fn test_oneof_roundtrip() {
    oneof_roundtrip_fixture(BasicOneof::Int(0));
    oneof_roundtrip_fixture(BasicOneof::Int(1));
    oneof_roundtrip_fixture(BasicOneof::Int(18446744073709551615));
    // oneof_roundtrip_fixture(BasicOneof::String(""));
// oneof_roundtrip_fixture(BasicOneof::String("0"));
// oneof_roundtrip_fixture(BasicOneof::String("0x123456789abcdef"));
}
