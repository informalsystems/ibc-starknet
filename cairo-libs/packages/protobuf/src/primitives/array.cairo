use protobuf::types::message::{
    DecodeContext, DecodeContextImpl, EncodeContext, EncodeContextImpl, ProtoCodecImpl,
    ProtoMessage, decode_raw,
};
use protobuf::types::tag::WireType;
use super::super::types::message::DecodeContextTrait;

pub impl ByteArrayAsProtoMessage of ProtoMessage<ByteArray> {
    fn encode_raw(self: @ByteArray, ref context: EncodeContext) {
        context.buffer.append(self);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<ByteArray> {
        let mut value: ByteArray = "";
        while context.can_read_branch() {
            value.append_byte(context.buffer[context.index]);
            context.index += 1;
        }
        Option::Some(value)
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

// for packed repeated fields (default for scalars)
pub impl ArrayAsProtoMessage<T, +ProtoMessage<T>, +Drop<T>, +Default<T>> of ProtoMessage<Array<T>> {
    fn encode_raw(self: @Array<T>, ref context: EncodeContext) {
        let mut self_span = self.span();
        while let Option::Some(item) = self_span.pop_front() {
            if ProtoMessage::<T>::wire_type() == WireType::LengthDelimited {
                let mut context2 = EncodeContextImpl::new();
                item.encode_raw(ref context2);
                context2.buffer.len().encode_raw(ref context);
                context.buffer.append(@context2.buffer);
            } else {
                item.encode_raw(ref context);
            }
        }
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Array<T>> {
        let mut value = ArrayTrait::new();
        if ProtoMessage::<T>::wire_type() == WireType::LengthDelimited {
            while context.can_read_branch() {
                let length = decode_raw(ref context);
                if length.is_none() {
                    return Option::None;
                }
                if context.init_branch(length.unwrap()).is_none() {
                    return Option::None;
                }
                let item = decode_raw(ref context);
                if item.is_none() {
                    return Option::None;
                }
                if context.end_branch().is_none() {
                    return Option::None;
                }
                value.append(item.unwrap());
            }
        } else {
            while context.can_read_branch() {
                let item = decode_raw(ref context);
                if item.is_none() {
                    return Option::None;
                }
                value.append(item.unwrap());
            }
        }
        Option::Some(value)
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

pub impl BytesAsProtoMessage of ProtoMessage<Array<u8>> {
    fn encode_raw(self: @Array<u8>, ref context: EncodeContext) {
        if self.len() == 0 {
            context.buffer.append_byte(0);
        }
        let mut self_span = self.span();
        while let Option::Some(item) = self_span.pop_front() {
            context.buffer.append_byte(*item);
        };
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Array<u8>> {
        let mut bytes = ArrayTrait::new();
        while context.can_read_branch() {
            bytes.append(context.buffer[context.index]);
            context.index += 1;
        }
        Option::Some(bytes)
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}
