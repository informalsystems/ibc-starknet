use protobuf::types::message::{
    ProtoMessage, ProtoOneof, DecodeContext, EncodeContext, EncodeContextTrait, DecodeContextTrait,
    ProtoCodecImpl,
};
use protobuf::types::tag::{WireType, ProtobufTag};
use protobuf::primitives::numeric::U64AsProtoMessage;
use protobuf::primitives::array::ByteArrayAsProtoMessage;

#[derive(Drop, Debug, Default, PartialEq)]
pub struct MessageWithOneof {
    oneof: Oneof,
}

pub impl MessageWithOneofAsProtoMessage of ProtoMessage<MessageWithOneof> {
    fn encode_raw(self: @MessageWithOneof, ref context: EncodeContext) {
        context.encode_oneof(self.oneof);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<MessageWithOneof> {
        let oneof = context.decode_oneof()?;
        Option::Some(MessageWithOneof { oneof })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Drop, Debug, Default, PartialEq)]
pub enum Oneof {
    #[default]
    Int: u64,
    String: ByteArray,
    Enum: Enum,
}

pub impl OneofAsProtoOneof of ProtoOneof<Oneof> {
    fn encode_raw(self: @Oneof, ref context: EncodeContext) -> ProtobufTag {
        match self {
            Oneof::Int(n) => {
                n.encode_raw(ref context);
                let wire_type = ProtoMessage::<u64>::wire_type();
                ProtobufTag { field_number: 1, wire_type }
            },
            Oneof::String(str) => {
                str.encode_raw(ref context);
                let wire_type = ProtoMessage::<ByteArray>::wire_type();
                ProtobufTag { field_number: 2, wire_type }
            },
            Oneof::Enum(e) => {
                let n: u32 = e.into();
                n.encode_raw(ref context);
                let wire_type = ProtoMessage::<u32>::wire_type();
                ProtobufTag { field_number: 3, wire_type }
            },
        }
    }

    fn decode_raw(ref context: DecodeContext, tag: ProtobufTag) -> Option<Oneof> {
        if tag.field_number == 1 {
            if tag.wire_type != ProtoMessage::<u64>::wire_type() {
                return Option::None;
            }
            let value = context.decode_field(1)?;
            Option::Some(Oneof::Int(value))
        } else if tag.field_number == 2 {
            if tag.wire_type != ProtoMessage::<ByteArray>::wire_type() {
                return Option::None;
            }
            let value = context.decode_field(2)?;
            Option::Some(Oneof::String(value))
        } else if tag.field_number == 3 {
            if tag.wire_type != ProtoMessage::<u32>::wire_type() {
                return Option::None;
            }
            let value = context.decode_enum(3)?;
            Option::Some(Oneof::Enum(value))
        } else {
            Option::None
        }
    }
}

#[derive(Drop, Debug, Default, PartialEq)]
pub enum Enum {
    #[default]
    A,
    B,
}

pub impl EnumIntoU32 of Into<@Enum, u32> {
    fn into(self: @Enum) -> u32 {
        match self {
            Enum::A => 0,
            Enum::B => 1,
        }
    }
}

pub impl U32TryIntoEnum of TryInto<u32, Enum> {
    fn try_into(self: u32) -> Option<Enum> {
        match self {
            0 => Option::Some(Enum::A),
            1 => Option::Some(Enum::B),
            _ => Option::None,
        }
    }
}

fn oneof_roundtrip_fixture(oneof: Oneof) {
    let message = MessageWithOneof { oneof };
    let encoded = ProtoCodecImpl::encode(@message);
    let decoded = ProtoCodecImpl::decode(@encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_oneof_roundtrip() {
    oneof_roundtrip_fixture(Oneof::Int(0));
    oneof_roundtrip_fixture(Oneof::Int(1));
    oneof_roundtrip_fixture(Oneof::Int(18446744073709551615));
    oneof_roundtrip_fixture(Oneof::String(""));
    oneof_roundtrip_fixture(Oneof::String("0"));
    oneof_roundtrip_fixture(Oneof::String("0x123456789abcdef"));
    oneof_roundtrip_fixture(Oneof::Enum(Enum::A));
    oneof_roundtrip_fixture(Oneof::Enum(Enum::B));
}
