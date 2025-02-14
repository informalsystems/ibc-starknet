use super::super::types::message::DecodeContextTrait;
use protobuf::types::message::{
    ProtoMessage, ProtoCodecImpl, EncodeContext, DecodeContext, EncodeContextImpl,
    DecodeContextImpl, decode_raw,
};
use protobuf::types::tag::WireType;

pub impl ByteArrayAsProtoMessage of ProtoMessage<ByteArray> {
    fn encode_raw(self: @ByteArray, ref context: EncodeContext) {
        context.buffer.append(self);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<ByteArray> {
        let mut value: ByteArray = "";
        while context.can_read_branch() {
            value.append_byte(context.buffer[context.index]);
            context.index += 1;
        };
        Option::Some(value)
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

    fn decode_raw(ref context: DecodeContext) -> Option<Array<T>> {
        let mut failed = false;
        let mut value = ArrayTrait::new();
        if ProtoMessage::<T>::wire_type() == WireType::LengthDelimited {
            while context.can_read_branch() {
                let length = decode_raw(ref context);
                if length.is_none() {
                    failed = true;
                    break;
                }
                if !context.init_branch(length.unwrap()) {
                    failed = true;
                    break;
                }
                let item = decode_raw(ref context);
                if item.is_none() {
                    failed = true;
                    break;
                }
                if !context.end_branch() {
                    failed = true;
                    break;
                }
                value.append(item.unwrap());
            }
        } else {
            while context.can_read_branch() {
                let item = decode_raw(ref context);
                if item.is_none() {
                    failed = true;
                    break;
                }
                value.append(item.unwrap());
            }
        }
        if failed {
            return Option::None;
        }
        Option::Some(value)
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

pub impl BytesAsProtoMessage of ProtoMessage<Array<u8>> {
    fn encode_raw(self: @Array<u8>, ref context: EncodeContext) {
        let mut i = 0;
        if self.len() == 0 {
            context.buffer.append_byte(0);
        }
        while i < self.len() {
            context.buffer.append_byte(self[i].clone());
            i += 1;
        };
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Array<u8>> {
        let mut bytes = ArrayTrait::new();
        while context.can_read_branch() {
            bytes.append(context.buffer[context.index]);
            context.index += 1;
        };
        Option::Some(bytes)
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}
