use ibc_utils::numeric::{
    decode_2_complement_128, decode_2_complement_32, decode_2_complement_64,
    encode_2_complement_128, encode_2_complement_32, encode_2_complement_64, u64_from_little_endian,
    u64_to_little_endian,
};
use protobuf::types::message::{
    DecodeContext, DecodeContextImpl, EncodeContext, EncodeContextImpl, ProtoCodecImpl,
    ProtoMessage, decode_raw,
};
use protobuf::types::tag::WireType;
use protobuf::varint::{decode_varint_from_u8_array_with_index, encode_varint_to_u8_array};

pub impl U128AsProtoMessage of ProtoMessage<u128> {
    fn encode_raw(self: @u128, ref context: EncodeContext) {
        let num = (*self).into();

        let bytes = encode_varint_to_u8_array(num);
        context.buffer.append_span(bytes.span());
    }

    fn decode_raw(ref context: DecodeContext) -> Option<u128> {
        decode_varint_from_u8_array_with_index(context.buffer, ref context.index).ok()
    }

    fn wire_type() -> WireType {
        WireType::Varint
    }
}

pub impl U64AsProtoMessage of ProtoMessage<u64> {
    fn encode_raw(self: @u64, ref context: EncodeContext) {
        let num = (*self).into();

        let bytes = encode_varint_to_u8_array(num);
        context.buffer.append_span(bytes.span());
    }

    fn decode_raw(ref context: DecodeContext) -> Option<u64> {
        decode_varint_from_u8_array_with_index(context.buffer, ref context.index)
            .ok()
            .map(|num| num.try_into().unwrap())
    }

    fn wire_type() -> WireType {
        WireType::Varint
    }
}

pub impl U32AsProtoMessage of ProtoMessage<u32> {
    fn encode_raw(self: @u32, ref context: EncodeContext) {
        let num = (*self).into();

        let bytes = encode_varint_to_u8_array(num);
        context.buffer.append_span(bytes.span());
    }

    fn decode_raw(ref context: DecodeContext) -> Option<u32> {
        let varint = decode_varint_from_u8_array_with_index(context.buffer, ref context.index)
            .ok()?;
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

pub impl I128AsProtoMessage of ProtoMessage<i128> {
    fn encode_raw(self: @i128, ref context: EncodeContext) {
        let num: u128 = encode_2_complement_128(@(*self).into());
        num.encode_raw(ref context);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<i128> {
        let num = decode_raw(ref context)?;
        Option::Some(decode_2_complement_128(@num))
    }

    fn wire_type() -> WireType {
        WireType::Varint
    }
}

pub impl SFixed64AsProtoMessage of ProtoMessage<i64> {
    // number is encoded as little-ending chunks
    // https://protobuf.dev/programming-guides/encoding/#cheat-sheet

    fn encode_raw(self: @i64, ref context: EncodeContext) {
        let num: u64 = encode_2_complement_64(@(*self).into());
        let mut bytes = u64_to_little_endian(num).span();
        while let Some(byte) = bytes.pop_front() {
            context.buffer.append(*byte);
        }
    }

    fn decode_raw(ref context: DecodeContext) -> Option<i64> {
        let bytes = [
            *context.buffer[context.index], *context.buffer[context.index + 1],
            *context.buffer[context.index + 2], *context.buffer[context.index + 3],
            *context.buffer[context.index + 4], *context.buffer[context.index + 5],
            *context.buffer[context.index + 6], *context.buffer[context.index + 7],
        ];
        context.index += 8;
        let num = u64_from_little_endian(bytes);
        Option::Some(decode_2_complement_64(@num))
    }

    fn wire_type() -> WireType {
        WireType::Fixed64
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
            0 => Option::Some(false),
            1 => Option::Some(true),
            _ => Option::None,
        }
    }

    fn wire_type() -> WireType {
        WireType::Varint
    }
}
