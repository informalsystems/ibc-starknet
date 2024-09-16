use protobuf::types::tag::{WireType, ProtobufTag, ProtobufTagImpl};
use protobuf::primitives::numeric::NumberAsProtoMessage;

pub trait ProtoMessage<T> {
    fn decode_raw(ref value: T, serialized: @ByteArray, ref index: usize, length: usize);
    fn encode_raw(self: @T, ref output: ByteArray);
    fn wire_type() -> WireType;
}

pub struct ProtoCodec {}

#[generate_trait]
pub impl ProtoCodecImpl of ProtoCodecTrait {
    fn decode<T, +ProtoMessage<T>, +Drop<T>, +Default<T>>(serialized: @ByteArray) -> T {
        let mut index = 0;
        let length = if ProtoMessage::<T>::wire_type() == WireType::LengthDelimited {
            serialized.len()
        } else {
            0
        };
        let mut value = Default::<T>::default();
        ProtoMessage::<T>::decode_raw(ref value, serialized, ref index, length);
        value
    }

    fn encode<T, +ProtoMessage<T>>(value: @T) -> ByteArray {
        let mut bytes = "";
        value.encode_raw(ref bytes);
        bytes
    }


    fn decode_length_delimited_raw<T, +ProtoMessage<T>, +Drop<T>>(
        field_number: u8, ref value: T, serialized: @ByteArray, ref index: usize
    ) {
        if index < serialized.len() {
            let tag = ProtobufTagImpl::decode(serialized[index]);
            if tag.field_number == field_number {
                index += 1;

                let write_type = ProtoMessage::<T>::wire_type();

                assert(write_type == tag.wire_type, 'unexpected wire type');

                let mut length = 0;

                if write_type == WireType::LengthDelimited {
                    ProtoMessage::<usize>::decode_raw(ref length, serialized, ref index, 0);
                }

                ProtoMessage::<T>::decode_raw(ref value, serialized, ref index, length);
            }
        }
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
