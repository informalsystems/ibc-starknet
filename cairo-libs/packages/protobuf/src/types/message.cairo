use protobuf::primitives::numeric::U32AsProtoMessage;
use protobuf::types::tag::{ProtobufTag, ProtobufTagImpl, WireType};
use super::tag::ProtobufTagTrait;

pub trait ProtoMessage<T> {
    fn decode_raw(ref context: DecodeContext) -> Option<T>;
    fn encode_raw(self: @T, ref context: EncodeContext);
    fn wire_type() -> WireType;
}

pub trait ProtoOneof<T> {
    fn encode_raw(self: @T, ref context: EncodeContext) -> ProtobufTag;
    fn decode_raw(ref context: DecodeContext, tag: u8) -> Option<T>;
}

pub fn decode_raw<T, +Drop<T>, +Default<T>, +ProtoMessage<T>>(
    ref context: DecodeContext,
) -> Option<T> {
    ProtoMessage::<T>::decode_raw(ref context)
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

    fn encode_optional_field<T, +Drop<T>, +Default<T>, +PartialEq<T>, +ProtoMessage<T>>(
        ref self: EncodeContext, field_number: u8, value: @Option<T>,
    ) {
        if let Option::Some(v) = value {
            self.encode_field(field_number, v);
        }
    }

    /// Performs the Protobuf encoding for an enum field.
    fn encode_enum<T, +Drop<T>, +Into<T, u32>>(
        ref self: EncodeContext, field_number: u8, value: T,
    ) {
        let value_u32: u32 = value.into();
        self.encode_field(field_number, @value_u32)
    }

    /// Performs the Protobuf encoding for a `Oneof` field.
    fn encode_oneof<T, +ProtoOneof<T>, +Drop<T>>(ref self: EncodeContext, value: @T) {
        let mut context2 = Self::new();
        let tag = value.encode_raw(ref context2);
        self.buffer.append_byte(tag.encode());
        if tag.wire_type == WireType::LengthDelimited {
            context2.buffer.len().encode_raw(ref self);
        }
        self.buffer.append(@context2.buffer);
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

    fn init_branch(ref self: DecodeContext, length: usize) -> Option<()> {
        let limit = self.index + length;
        if limit > self.buffer.len() {
            return Option::None;
        }
        self.limits.append(self.index + length);
        Option::Some(())
    }

    fn can_read_branch(ref self: DecodeContext) -> bool {
        @self.index < self.limits[self.limits.len() - 1]
    }

    fn decode_raw<T, +Drop<T>, +Default<T>, +ProtoMessage<T>>(
        ref self: DecodeContext,
    ) -> Option<T> {
        ProtoMessage::<T>::decode_raw(ref self)
    }

    fn decode_field<T, +ProtoMessage<T>, +Drop<T>, +Default<T>>(
        ref self: DecodeContext, field_number: u8,
    ) -> Option<T> {
        let mut field = Default::<T>::default();
        if self.can_read_branch() {
            let tag = ProtobufTagImpl::decode(self.buffer[self.index]);
            if tag.field_number == field_number {
                self.index += 1;

                let wire_type = ProtoMessage::<T>::wire_type();

                if wire_type != tag.wire_type {
                    return Option::None;
                }

                if wire_type == WireType::LengthDelimited {
                    let length = self.decode_raw()?;
                    self.init_branch(length)?;
                    field = self.decode_raw()?;
                    self.end_branch()?;
                } else {
                    field = self.decode_raw()?;
                }
            } else if tag.field_number < field_number {
                return Option::None;
            }
        }
        Option::Some(field)
    }

    // for unpacked repeated fields (default for non-scalars)
    fn decode_repeated_field<T, +ProtoMessage<T>, +Default<T>, +Drop<T>>(
        ref self: DecodeContext, field_number: u8,
    ) -> Option<Array<T>> {
        let mut field = ArrayTrait::new();
        let mut failed = false;
        while self.can_read_branch() {
            let tag = ProtobufTagImpl::decode(self.buffer[self.index]);
            if tag.field_number != field_number {
                break;
            }
            if let Option::Some(item) = self.decode_field(field_number) {
                field.append(item)
            } else {
                failed = true;
                break;
            }
        }
        if failed {
            return Option::None;
        }
        Option::Some(field)
    }

    fn decode_optional_field<T, +Drop<T>, +Default<T>, +PartialEq<T>, +ProtoMessage<T>>(
        ref self: DecodeContext, field_number: u8,
    ) -> Option<T> {
        let value = self.decode_field(field_number)?;
        if value == Default::default() {
            return Option::None;
        }
        Option::Some(value)
    }

    /// Performs the Protobuf decoding for an enum field.
    fn decode_enum<T, +Drop<T>, +TryInto<u32, T>>(
        ref self: DecodeContext, field_number: u8,
    ) -> Option<T> {
        let value: u32 = self.decode_field(field_number)?;
        value.try_into()
    }

    /// Performs the Protobuf decoding for a `Oneof` field.
    fn decode_oneof<T, +ProtoOneof<T>, +Drop<T>>(ref self: DecodeContext) -> Option<T> {
        let tag = ProtobufTagImpl::decode(self.buffer[self.index]);
        let value = ProtoOneof::decode_raw(ref self, tag.field_number)?;
        Option::Some(value)
    }

    fn end_branch(ref self: DecodeContext) -> Option<()> {
        // TODO(rano): pop_back is not impl for Array<T>, this is inefficient
        let mut span = self.limits.span();
        let limit = span.pop_back()?;
        self.limits = span.into();
        if limit != @self.index {
            return Option::None;
        }
        Option::Some(())
    }
}

#[generate_trait]
pub impl ProtoCodecImpl of ProtoCodecTrait {
    fn decode<T, +ProtoMessage<T>, +Drop<T>, +Default<T>>(serialized: @ByteArray) -> Option<T> {
        let mut value = Default::<T>::default();
        let mut context = DecodeContextImpl::new(serialized);
        if ProtoMessage::<T>::wire_type() == WireType::LengthDelimited {
            context.init_branch(serialized.len())?;
            value = context.decode_raw()?;
            context.end_branch()?;
        } else {
            value = context.decode_raw()?;
        }
        Option::Some(value)
    }

    fn encode<T, +ProtoMessage<T>>(value: @T) -> ByteArray {
        let mut context = EncodeContextImpl::new();
        value.encode_raw(ref context);
        context.buffer
    }

    fn encode_as_msg<T, +ProtoMessage<T>, +Default<T>, +Drop<T>, +PartialEq<T>>(
        value: @T,
    ) -> ByteArray {
        // TODO(rano): can we avoid this?
        let mut context = EncodeContextImpl::new();
        context.encode_field(1, value);
        context.buffer
    }
}

