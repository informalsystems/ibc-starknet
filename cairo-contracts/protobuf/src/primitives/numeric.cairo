use protobuf::types::message::{
    ProtoMessage, ProtoCodecImpl, EncodeContext, DecodeContext, EncodeContextImpl, DecodeContextImpl
};
use protobuf::types::tag::WireType;
use protobuf::primitives::utils::{
    encode_varint_u64, decode_varint_u64, encode_2_complement_64, decode_2_complement_64,
    encode_2_complement_32, decode_2_complement_32
};

pub impl UnsignedAsProtoMessage<
    T, +Into<T, u64>, +TryInto<u64, T>, +Copy<T>, +Drop<T>
> of ProtoMessage<T> {
    fn encode_raw(self: @T, ref context: EncodeContext) {
        let num = (*self).into();

        let bytes = encode_varint_u64(@num);
        context.buffer.append(@bytes);
    }

    fn decode_raw(ref self: T, ref context: DecodeContext, length: usize) {
        assert(length == 0, 'invalid length for u64');
        self = decode_varint_u64(context.buffer, ref context.index).try_into().unwrap()
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

    fn decode_raw(ref self: i32, ref context: DecodeContext, length: usize) {
        let mut num: u32 = 0;
        num.decode_raw(ref context, length);
        self = decode_2_complement_32(@num)
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

    fn decode_raw(ref self: i64, ref context: DecodeContext, length: usize) {
        let mut num: u64 = 0;
        num.decode_raw(ref context, length);
        self = decode_2_complement_64(@num)
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

    fn decode_raw(ref self: bool, ref context: DecodeContext, length: usize) {
        assert(length == 0, 'invalid length for bool');
        let mut num: u64 = 0;
        num.decode_raw(ref context, length);
        if num != 0 && num != 1 {
            panic!("invalid boolean value");
        }
        self = num == 1
    }

    fn wire_type() -> WireType {
        WireType::Varint
    }
}
