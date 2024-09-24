use protobuf::types::message::{
    ProtoMessage, ProtoCodecImpl, EncodeContext, DecodeContext, EncodeContextImpl, DecodeContextImpl
};
use protobuf::types::tag::WireType;
use protobuf::primitives::numeric::UnsignedAsProtoMessage;

pub impl ByteArrayAsProtoMessage of ProtoMessage<ByteArray> {
    fn encode_raw(self: @ByteArray, ref context: EncodeContext) {
        context.buffer.append(self);
    }

    fn decode_raw(ref self: ByteArray, ref context: DecodeContext, length: usize) {
        context.init_branch(length);

        while context.can_read_branch() {
            self.append_byte(context.buffer[context.index]);
            context.index += 1;
        };

        context.end_branch();
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}


// for packed repeated fields (default for scalars)
pub impl ArrayAsProtoMessage<T, +ProtoMessage<T>, +Drop<T>, +Default<T>> of ProtoMessage<Array<T>> {
    fn encode_raw(self: @Array<T>, ref context: EncodeContext) {
        let mut i = 0;
        if ProtoMessage::<T>::wire_type() == WireType::LengthDelimited {
            while i < self.len() {
                let mut context2 = EncodeContextImpl::new();
                self[i].encode_raw(ref context2);
                context2.buffer.len().encode_raw(ref context);
                context.buffer.append(@context2.buffer);
                i += 1;
            };
        } else {
            while i < self.len() {
                self[i].encode_raw(ref context);
                i += 1;
            };
        }
    }

    fn decode_raw(ref self: Array<T>, ref context: DecodeContext, length: usize) {
        context.init_branch(length);

        if ProtoMessage::<T>::wire_type() == WireType::LengthDelimited {
            while context.can_read_branch() {
                let mut length = 0;
                length.decode_raw(ref context, 0);
                let mut item = Default::<T>::default();
                item.decode_raw(ref context, length);
                self.append(item);
            }
        } else {
            while context.can_read_branch() {
                let mut item = Default::<T>::default();
                item.decode_raw(ref context, 0);
                self.append(item);
            }
        }

        context.end_branch();
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

