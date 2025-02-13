use protobuf::types::tag::{WireType, ProtobufTag, ProtobufTagImpl};
use protobuf::primitives::numeric::U32AsProtoMessage;

pub trait ProtoMessage<T> {
    fn decode_raw(ref self: T, ref context: DecodeContext);
    fn encode_raw(self: @T, ref context: EncodeContext);
    fn wire_type() -> WireType;
}

pub trait ProtoName<T> {
    fn type_url() -> ByteArray;
}

#[derive(Drop)]
pub struct EncodeContext {
    pub buffer: ByteArray,
}

#[generate_trait]
pub impl EncodeContextImpl of EncodeContextTrait {
    fn new() -> EncodeContext {
        EncodeContext { buffer: "" }
    }

    fn encode_field<T, +ProtoMessage<T>, +Default<T>, +PartialEq<T>, +Drop<T>>(
        ref self: EncodeContext, field_number: u8, value: @T,
    ) {
        // ignore default values
        if value != @Default::<T>::default() {
            let mut context2 = Self::new();
            value.encode_raw(ref context2);
            let wire_type = ProtoMessage::<T>::wire_type();
            self.buffer.append_byte(ProtobufTag { field_number, wire_type }.encode());
            if wire_type == WireType::LengthDelimited {
                context2.buffer.len().encode_raw(ref self);
            }
            self.buffer.append(@context2.buffer);
        }
    }

    // for unpacked repeated fields (default for non-scalars)
    fn encode_repeated_field<T, +ProtoMessage<T>>(
        ref self: EncodeContext, field_number: u8, value: @Array<T>,
    ) {
        let mut i = 0;
        while i < value.len() {
            let mut context2 = Self::new();
            (value[i]).encode_raw(ref context2);
            // do not ignore default values
            let wire_type = ProtoMessage::<T>::wire_type();
            self.buffer.append_byte(ProtobufTag { field_number, wire_type }.encode());
            if wire_type == WireType::LengthDelimited {
                context2.buffer.len().encode_raw(ref self);
            }
            self.buffer.append(@context2.buffer);

            i += 1;
        }
    }
}

#[derive(Drop, Debug)]
pub struct DecodeContext {
    pub buffer: @ByteArray,
    pub index: usize,
    pub limits: Array<usize>,
}

#[generate_trait]
pub impl DecodeContextImpl of DecodeContextTrait {
    fn new(buffer: @ByteArray) -> DecodeContext {
        DecodeContext { buffer, index: 0, limits: array![] }
    }

    fn init_branch(ref self: DecodeContext, length: usize) {
        let limit = self.index + length;
        assert(limit <= self.buffer.len(), 'bigger limit for branch');
        self.limits.append(self.index + length);
    }

    fn can_read_branch(ref self: DecodeContext) -> bool {
        @self.index < self.limits[self.limits.len() - 1]
    }

    fn decode_field<T, +ProtoMessage<T>, +Drop<T>, +Default<T>>(
        ref self: DecodeContext, field_number: u8, ref value: T,
    ) {
        if self.can_read_branch() {
            let tag = ProtobufTagImpl::decode(self.buffer[self.index]);
            if tag.field_number == field_number {
                self.index += 1;

                let wire_type = ProtoMessage::<T>::wire_type();

                assert(wire_type == tag.wire_type, 'unexpected wire type');

                if wire_type == WireType::LengthDelimited {
                    let mut length = 0;
                    length.decode_raw(ref self);
                    self.init_branch(length);
                    value.decode_raw(ref self);
                    self.end_branch();
                } else {
                    value.decode_raw(ref self);
                }
            } else if tag.field_number < field_number {
                panic!(
                    "unexpected field number order: at expected field {} but got older field {}",
                    field_number,
                    tag.field_number,
                );
            }
        }
    }

    // for unpacked repeated fields (default for non-scalars)
    fn decode_repeated_field<T, +ProtoMessage<T>, +Default<T>, +Drop<T>>(
        ref self: DecodeContext, field_number: u8, ref value: Array<T>,
    ) {
        while self.can_read_branch() {
            let tag = ProtobufTagImpl::decode(self.buffer[self.index]);
            if tag.field_number != field_number {
                break;
            }
            let mut item = Default::<T>::default();

            self.decode_field(field_number, ref item);
            value.append(item);
        };
    }

    fn end_branch(ref self: DecodeContext) {
        // TODO(rano): pop_back is not impl for Array<T>, this is inefficient
        let mut span = self.limits.span();
        let limit = span.pop_back().unwrap();
        self.limits = span.into();
        assert(limit == @self.index, 'smaller limit for branch');
    }
}

#[generate_trait]
pub impl ProtoCodecImpl of ProtoCodecTrait {
    fn decode<T, +ProtoMessage<T>, +Drop<T>, +Default<T>>(serialized: @ByteArray) -> T {
        let mut value = Default::<T>::default();
        let mut context = DecodeContextImpl::new(serialized);
        if ProtoMessage::<T>::wire_type() == WireType::LengthDelimited {
            context.init_branch(serialized.len());
            value.decode_raw(ref context);
            context.end_branch();
        } else {
            value.decode_raw(ref context);
        };
        value
    }

    fn encode<T, +ProtoMessage<T>>(value: @T) -> ByteArray {
        let mut context = EncodeContextImpl::new();
        value.encode_raw(ref context);
        context.buffer
    }
}

