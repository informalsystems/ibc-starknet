use protobuf::types::message::{
    ProtoMessage, ProtoCodecImpl, EncodeContext, DecodeContext, EncodeContextImpl,
    DecodeContextImpl, decode_raw,
};
use protobuf::types::tag::WireType;
use protobuf::primitives::utils::{
    encode_2_complement_64, decode_2_complement_64, encode_2_complement_32, decode_2_complement_32,
};
use protobuf::varint::{encode_varint_to_byte_array, decode_varint_from_byte_array};

pub impl U64AsProtoMessage of ProtoMessage<u64> {
    fn encode_raw(self: @u64, ref context: EncodeContext) {
        let num = (*self).into();

        let bytes = encode_varint_to_byte_array(num);
        context.buffer.append(@bytes);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<u64> {
        decode_varint_from_byte_array(context.buffer, ref context.index).ok()
    }

    fn wire_type() -> WireType {
        WireType::Varint
    }
}

pub impl U32AsProtoMessage of ProtoMessage<u32> {
    fn encode_raw(self: @u32, ref context: EncodeContext) {
        let num = (*self).into();

        let bytes = encode_varint_to_byte_array(num);
        context.buffer.append(@bytes);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<u32> {
        let varint = decode_varint_from_byte_array(context.buffer, ref context.index).ok()?;
        let num = varint.try_into()?;
        Option::Some(num)
    }

    fn wire_type() -> WireType {
        WireType::Varint
    }
}

pub impl I32AsProtoMessage of ProtoMessage<i32> {
    fn encode_raw(self: @i32, ref context: EncodeContext) {
        let num: u32 = encode_2_complement_32(@(*self).into());
        num.encode_raw(ref context);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<i32> {
        let num = decode_raw(ref context)?;
        Option::Some(decode_2_complement_32(@num))
    }

    fn wire_type() -> WireType {
        WireType::Varint
    }
}

pub impl I64AsProtoMessage of ProtoMessage<i64> {
    fn encode_raw(self: @i64, ref context: EncodeContext) {
        let num: u64 = encode_2_complement_64(@(*self).into());
        num.encode_raw(ref context);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<i64> {
        let num = decode_raw(ref context)?;
        Option::Some(decode_2_complement_64(@num))
    }

    fn wire_type() -> WireType {
        WireType::Varint
    }
}

pub impl BoolAsProtoMessage of ProtoMessage<bool> {
    fn encode_raw(self: @bool, ref context: EncodeContext) {
        let num: u64 = if *self {
            1
        } else {
            0
        };
        num.encode_raw(ref context);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<bool> {
        let num: u64 = decode_raw(ref context)?;
        match num {
            0 => { return Option::Some(false); },
            1 => { return Option::Some(true); },
            _ => { return Option::None; },
        }
    }

    fn wire_type() -> WireType {
        WireType::Varint
    }
}
