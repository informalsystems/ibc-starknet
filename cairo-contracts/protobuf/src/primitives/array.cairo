use protobuf::types::message::ProtoMessage;
use protobuf::types::tag::WireType;
use protobuf::primitives::numeric::NumberAsProtoMessage;

pub impl ByteArrayAsProtoMessage of ProtoMessage<ByteArray> {
    fn encode_raw(self: @ByteArray, ref output: ByteArray) {
        output.append(self);
    }

    fn decode_raw(serialized: @ByteArray, ref index: usize, length: usize) -> ByteArray {
        let bound = index + length;

        let mut byte_array = "";
        while index < bound {
            byte_array.append_byte(serialized[index]);
            index += 1;
        };

        assert(index == bound, 'invalid length for byte array');

        byte_array
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

pub impl ArrayAsProtoMessage<T, +ProtoMessage<T>, +Drop<T>> of ProtoMessage<Array<T>> {
    fn encode_raw(self: @Array<T>, ref output: ByteArray) {
        let mut i = 0;
        while i < self.len() {
            ProtoMessage::<T>::encode_raw(self[i], ref output);
            i += 1;
        };
    }

    fn decode_raw(serialized: @ByteArray, ref index: usize, length: usize) -> Array<T> {
        let bound = index + length;

        let mut items = array![];

        if ProtoMessage::<T>::wire_type() == WireType::LengthDelimited {
            while index < bound {
                let length = ProtoMessage::<usize>::decode_raw(serialized, ref index, 0);
                items.append(ProtoMessage::<T>::decode_raw(serialized, ref index, length));
            }
        } else {
            while index < bound {
                items.append(ProtoMessage::<T>::decode_raw(serialized, ref index, 0));
            }
        }

        assert(index == bound, 'invalid length for array');

        items
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}
