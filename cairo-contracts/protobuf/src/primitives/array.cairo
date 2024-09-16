use protobuf::types::message::ProtoMessage;
use protobuf::types::tag::WireType;
use protobuf::primitives::numeric::NumberAsProtoMessage;

pub impl ByteArrayAsProtoMessage of ProtoMessage<ByteArray> {
    fn encode_raw(self: @ByteArray, ref output: ByteArray) {
        output.append(self);
    }

    fn decode_raw(ref value: ByteArray, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;

        while index < bound {
            value.append_byte(serialized[index]);
            index += 1;
        };

        assert(index == bound, 'invalid length for byte array');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

pub impl ArrayAsProtoMessage<T, +ProtoMessage<T>, +Drop<T>, +Default<T>> of ProtoMessage<Array<T>> {
    fn encode_raw(self: @Array<T>, ref output: ByteArray) {
        let mut i = 0;
        while i < self.len() {
            ProtoMessage::<T>::encode_raw(self[i], ref output);
            i += 1;
        };
    }

    fn decode_raw(ref value: Array<T>, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;

        if ProtoMessage::<T>::wire_type() == WireType::LengthDelimited {
            while index < bound {
                let mut length = 0;
                ProtoMessage::<usize>::decode_raw(ref length, serialized, ref index, 0);
                let mut item = Default::<T>::default();
                ProtoMessage::<T>::decode_raw(ref item, serialized, ref index, length);
                value.append(item);
            }
        } else {
            while index < bound {
                let mut item = Default::<T>::default();
                ProtoMessage::<T>::decode_raw(ref item, serialized, ref index, 0);
                value.append(item);
            }
        }

        assert(index == bound, 'invalid length for array');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}
