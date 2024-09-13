use protobuf::types::tag::{WireType, ProtobufTag, ProtobufTagImpl};
use protobuf::primitives::numeric::NumberAsProtoMessage;

pub trait ProtoMessage<T> {
    fn decode_raw(serialized: @ByteArray, ref index: usize, length: usize) -> T;
    fn encode_raw(self: @T, ref output: ByteArray);
    fn wire_type() -> WireType;
}

pub struct ProtoCodec {}

#[generate_trait]
pub impl ProtoCodecImpl of ProtoCodecTrait {
    fn decode<T, +ProtoMessage<T>, +Drop<T>>(serialized: @ByteArray) -> T {
        let mut index = 0;
        let length = if ProtoMessage::<T>::wire_type() == WireType::LengthDelimited {
            serialized.len()
        } else {
            0
        };
        ProtoMessage::<T>::decode_raw(serialized, ref index, length)
    }

    fn encode<T, +ProtoMessage<T>>(value: @T) -> ByteArray {
        let mut bytes = "";
        value.encode_raw(ref bytes);
        bytes
    }


    fn decode_length_delimited_raw<T, +ProtoMessage<T>, +Drop<T>>(
        serialized: @ByteArray, ref index: usize
    ) -> T {
        let length = if ProtoMessage::<T>::wire_type() == WireType::LengthDelimited {
            ProtoMessage::<usize>::decode_raw(serialized, ref index, 0)
        } else {
            0
        };
        ProtoMessage::<T>::decode_raw(serialized, ref index, length)
    }

    fn encode_length_delimited_raw<T, +ProtoMessage<T>>(
        field_number: u8, value: @T, ref output: ByteArray
    ) {
        let mut bytes = "";
        value.encode_raw(ref bytes);
        if bytes.len() > 0 {
            let wire_type = ProtoMessage::<T>::wire_type();
            output.append_byte(ProtobufTag { field_number, wire_type }.encode());
            if wire_type == WireType::LengthDelimited {
                ProtoMessage::<usize>::encode_raw(@bytes.len(), ref output);
            }
            output.append(@bytes);
        }
    }
}
