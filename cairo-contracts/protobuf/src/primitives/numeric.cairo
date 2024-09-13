use protobuf::types::message::ProtoMessage;
use protobuf::types::tag::WireType;
use protobuf::primitives::utils::{
    encode_varint_u64, decode_varint_u64, encode_2_complement_64, decode_2_complement_64,
    encode_2_complement_32, decode_2_complement_32
};

// impl U64AsProtoMessage of ProtoMessage<u64> {
//     fn encode_raw(self: @u64, ref output: ByteArray) {
//         if self != @Default::default() {
//             let bytes = encode_varint_u64(self);
//             output.append(@bytes);
//         }
//     }

//     fn decode_raw(serialized: @ByteArray, ref index: usize, length: usize) -> u64 {
//         assert(length == 0, 'invalid length for u64');
//         decode_varint_u64(serialized, ref index)
//     }

//     fn wire_type() -> WireType {
//         WireType::Varint
//     }
// }

// pub impl U32AsProtoMessage of ProtoMessage<u32> {
//     fn encode_raw(self: @u32, ref output: ByteArray) {
//         if self != @Default::default() {
//             let bytes = encode_varint_u64(@(*self).into());
//             output.append(@bytes);
//         }
//     }

//     fn decode_raw(serialized: @ByteArray, ref index: usize, length: usize) -> u32 {
//         assert(length == 0, 'invalid length for u32');
//         decode_varint_u64(serialized, ref index).try_into().unwrap()
//     }

//     fn wire_type() -> WireType {
//         WireType::Varint
//     }
// }

// pub impl U8AsProtoMessage of ProtoMessage<u8> {
//     fn encode_raw(self: @u8, ref output: ByteArray) {
//         if self != @Default::default() {
//             let bytes = encode_varint_u64(@(*self).into());
//             output.append(@bytes);
//         }
//     }

//     fn decode_raw(serialized: @ByteArray, ref index: usize, length: usize) -> u8 {
//         assert(length == 0, 'invalid length for u8');
//         decode_varint_u64(serialized, ref index).try_into().unwrap()
//     }

//     fn wire_type() -> WireType {
//         WireType::Varint
//     }
// }

pub impl NumberAsProtoMessage<T, +Into<T, u64>, +TryInto<u64, T>, +Copy<T>> of ProtoMessage<T> {
    fn encode_raw(self: @T, ref output: ByteArray) {
        let num = (*self).into();

        if num != Default::default() {
            let bytes = encode_varint_u64(@num);
            output.append(@bytes);
        }
    }

    fn decode_raw(serialized: @ByteArray, ref index: usize, length: usize) -> T {
        assert(length == 0, 'invalid length for u64');
        decode_varint_u64(serialized, ref index).try_into().unwrap()
    }

    fn wire_type() -> WireType {
        WireType::Varint
    }
}

pub impl I32AsProtoMessage of ProtoMessage<i32> {
    fn encode_raw(self: @i32, ref output: ByteArray) {
        let num = encode_2_complement_32(@(*self).into());
        NumberAsProtoMessage::<u32>::encode_raw(@num, ref output);
    }

    fn decode_raw(serialized: @ByteArray, ref index: usize, length: usize) -> i32 {
        let num = NumberAsProtoMessage::<u32>::decode_raw(serialized, ref index, length);
        decode_2_complement_32(@num)
    }

    fn wire_type() -> WireType {
        WireType::Varint
    }
}

pub impl I64AsProtoMessage of ProtoMessage<i64> {
    fn encode_raw(self: @i64, ref output: ByteArray) {
        let num = encode_2_complement_64(@(*self).into());
        NumberAsProtoMessage::<u64>::encode_raw(@num, ref output);
    }

    fn decode_raw(serialized: @ByteArray, ref index: usize, length: usize) -> i64 {
        let num = NumberAsProtoMessage::<u64>::decode_raw(serialized, ref index, length);
        decode_2_complement_64(@num)
    }

    fn wire_type() -> WireType {
        WireType::Varint
    }
}

pub impl BoolAsProtoMessage of ProtoMessage<bool> {
    fn encode_raw(self: @bool, ref output: ByteArray) {
        if self != @Default::default() {
            let num = if *self {
                1
            } else {
                0
            };
            let bytes = encode_varint_u64(@num);
            output.append(@bytes);
        }
    }

    fn decode_raw(serialized: @ByteArray, ref index: usize, length: usize) -> bool {
        assert(length == 0, 'invalid length for bool');
        let num = decode_varint_u64(serialized, ref index);
        if num != 0 && num != 1 {
            panic!("invalid boolean value");
        }
        num == 1
    }

    fn wire_type() -> WireType {
        WireType::Varint
    }
}
