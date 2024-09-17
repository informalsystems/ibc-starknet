use protobuf::types::tag::{WireType, ProtobufTag, ProtobufTagImpl};
use protobuf::primitives::numeric::NumberAsProtoMessage;
use protobuf::types::wkt::Any;

pub trait ProtoMessage<T> {
    fn decode_raw(ref value: T, serialized: @ByteArray, ref index: usize, length: usize);
    fn encode_raw(self: @T, ref output: ByteArray);
    fn wire_type() -> WireType;
    fn type_url() -> ByteArray;
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

    fn from_any<T, +ProtoMessage<T>, +Drop<T>, +Default<T>>(any: @Any) -> T {
        if any.type_url != @ProtoMessage::<T>::type_url() {
            panic!("unexpected type URL");
        }
        Self::decode::<T>(any.value)
    }

    fn to_any<T, +ProtoMessage<T>>(value: @T) -> Any {
        Any { type_url: ProtoMessage::<T>::type_url(), value: Self::encode(value) }
    }


    fn decode_length_delimited_raw<T, +ProtoMessage<T>, +Drop<T>>(
        field_number: u8, ref value: T, serialized: @ByteArray, ref index: usize, bound: usize
    ) {
        loop {
            assert(bound <= serialized.len(), 'invalid bound');
            if index < serialized.len() && index < bound {
                let tag = ProtobufTagImpl::decode(serialized[index]);
                if tag.field_number == field_number {
                    index += 1;

                    let wire_type = ProtoMessage::<T>::wire_type();

                    assert(wire_type == tag.wire_type, 'unexpected wire type');

                    let mut length = 0;

                    if wire_type == WireType::LengthDelimited {
                        ProtoMessage::<usize>::decode_raw(ref length, serialized, ref index, 0);
                    }

                    ProtoMessage::<T>::decode_raw(ref value, serialized, ref index, length);
                } else if tag.field_number < field_number {
                    panic!(
                        "unexpected field number order: at expected field {} but got older field {}",
                        field_number,
                        tag.field_number
                    );
                } else {
                    break;
                }
            }
        }
    }

    fn encode_length_delimited_raw<T, +ProtoMessage<T>, +Clone<T>, +Drop<T>>(
        field_number: u8, value: @T, ref output: ByteArray
    ) {
        let mut value = value.clone();
        loop {
            // TODO(rano): fix for array.
            let mut bytes = "";
            value.encode_raw(ref bytes);
            if bytes.len() > 0 {
                let wire_type = ProtoMessage::<T>::wire_type();
                output.append_byte(ProtobufTag { field_number, wire_type }.encode());
                if wire_type == WireType::LengthDelimited {
                    ProtoMessage::<usize>::encode_raw(@bytes.len(), ref output);
                }
                output.append(@bytes);
            } else {
                break;
            }
        }
    }
}
