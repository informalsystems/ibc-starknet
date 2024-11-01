use serde_json::{FormatterTrait, CompactFormatter, byte_array_to_array_u8};

pub trait Serialize<T> {
    fn serialize<S, +Drop<S>, +SerializerTrait<S>>(self: T, ref serializer: S);
}

pub trait SerializerTrait<S> {
    fn end(ref self: S);

    fn serialize_bool(ref self: S, v: bool);

    fn serialize_u8(ref self: S, v: u8);

    fn serialize_u32(ref self: S, v: u32);

    fn serialize_u64(ref self: S, v: u64);

    fn serialize_u128(ref self: S, v: u128);

    fn serialize_u256(ref self: S, v: u256);

    fn serialize_felt252(ref self: S, v: felt252);

    fn serialize_unit(ref self: S);

    fn serialize_some<V, +Drop<V>, +Serialize<V>>(ref self: S, v: V);

    fn serialize_none(ref self: S);

    fn serialize_string(ref self: S, v: ByteArray);

    fn serialize_field<V, +Drop<V>, +Serialize<V>>(
        ref self: S, field_name: ByteArray, field_value: V
    );

    fn serialize_variant<V, +Drop<V>, +Serialize<V>>(
        ref self: S, variant_name: ByteArray, variant_value: V
    );
}

pub type DefaultSerializer = Serializer<CompactFormatter>;

#[derive(Drop, Clone)]
pub struct Serializer<F, +FormatterTrait<F>> {
    writer: ByteArray,
    formatter: F,
}

pub impl SerializerImpl of SerializerTrait<DefaultSerializer> {
    fn end(ref self: DefaultSerializer) {
        self.formatter.end_object(ref self.writer);
    }

    fn serialize_bool(ref self: DefaultSerializer, v: bool) {
        self.formatter.write_bool(ref self.writer, v);
    }

    fn serialize_u8(ref self: DefaultSerializer, v: u8) {
        self.formatter.write_u8(ref self.writer, v);
    }

    fn serialize_u32(ref self: DefaultSerializer, v: u32) {
        self.formatter.write_u32(ref self.writer, v);
    }

    fn serialize_u64(ref self: DefaultSerializer, v: u64) {
        self.formatter.write_u64(ref self.writer, v);
    }

    fn serialize_u128(ref self: DefaultSerializer, v: u128) {
        self.formatter.write_u128(ref self.writer, v);
    }

    fn serialize_u256(ref self: DefaultSerializer, v: u256) {
        self.formatter.write_u256(ref self.writer, v);
    }

    fn serialize_felt252(ref self: DefaultSerializer, v: felt252) {
        self.formatter.write_felt252(ref self.writer, v);
    }

    fn serialize_unit(ref self: DefaultSerializer) {
        self.formatter.write_null(ref self.writer);
    }

    fn serialize_some<V, +Drop<V>, +Serialize<V>>(ref self: DefaultSerializer, v: V) {
        v.serialize(ref self);
    }

    fn serialize_none(ref self: DefaultSerializer) {
        self.serialize_unit();
    }

    fn serialize_string(ref self: DefaultSerializer, v: ByteArray) {
        self.formatter.begin_string(ref self.writer);
        self.formatter.write_string(ref self.writer, @v);
        self.formatter.end_string(ref self.writer);
    }

    fn serialize_field<V, +Drop<V>, +Serialize<V>>(
        ref self: DefaultSerializer, field_name: ByteArray, field_value: V
    ) {
        if self.writer.len() == 0 {
            self.formatter.begin_object(ref self.writer);
        } else if self.writer.at(self.writer.len() - 1) == Option::Some(58) {
            self.formatter.begin_object(ref self.writer);
        }
        if self.writer.at(self.writer.len() - 1) != Option::Some(123) {
            self.formatter.end_object_value(ref self.writer);
        }
        self.serialize_string(field_name);
        self.formatter.end_object_key(ref self.writer);
        field_value.serialize(ref self);
    }

    fn serialize_variant<V, +Drop<V>, +Serialize<V>>(
        ref self: DefaultSerializer, variant_name: ByteArray, variant_value: V
    ) {
        if self.writer.len() == 0 {
            self.formatter.begin_object(ref self.writer);
        } else if self.writer.at(self.writer.len() - 1) == Option::Some(58) {
            self.formatter.begin_object(ref self.writer);
        }
        if self.writer.at(self.writer.len() - 1) != Option::Some(123) {
            self.formatter.end_object_value(ref self.writer);
        }
        self.serialize_field(variant_name, variant_value);
    }
}

// ----------------------------------------------------------------
// Implementations of Serialize for primitive types
// ----------------------------------------------------------------

pub impl SerializeBool of Serialize<bool> {
    fn serialize<S, +Drop<S>, +SerializerTrait<S>>(self: bool, ref serializer: S) {
        serializer.serialize_bool(self);
    }
}

pub impl SerializeU8 of Serialize<u8> {
    fn serialize<S, +Drop<S>, +SerializerTrait<S>>(self: u8, ref serializer: S) {
        serializer.serialize_u8(self);
    }
}

pub impl SerializeU32 of Serialize<u32> {
    fn serialize<S, +Drop<S>, +SerializerTrait<S>>(self: u32, ref serializer: S) {
        serializer.serialize_u32(self);
    }
}

pub impl SerializeU64 of Serialize<u64> {
    fn serialize<S, +Drop<S>, +SerializerTrait<S>>(self: u64, ref serializer: S) {
        serializer.serialize_u64(self);
    }
}

pub impl SerializeU128 of Serialize<u128> {
    fn serialize<S, +Drop<S>, +SerializerTrait<S>>(self: u128, ref serializer: S) {
        serializer.serialize_u128(self);
    }
}

pub impl SerializeU256 of Serialize<u256> {
    fn serialize<S, +Drop<S>, +SerializerTrait<S>>(self: u256, ref serializer: S) {
        serializer.serialize_u256(self);
    }
}

pub impl SerializeFelt252 of Serialize<felt252> {
    fn serialize<S, +Drop<S>, +SerializerTrait<S>>(self: felt252, ref serializer: S) {
        serializer.serialize_felt252(self);
    }
}

pub impl SerializeByteArray of Serialize<ByteArray> {
    fn serialize<S, +Drop<S>, +SerializerTrait<S>>(self: ByteArray, ref serializer: S) {
        serializer.serialize_string(self);
    }
}

pub impl SerializeEmpty of Serialize<()> {
    fn serialize<S, +Drop<S>, +SerializerTrait<S>>(self: (), ref serializer: S) {
        serializer.serialize_unit();
    }
}

pub impl SerializeOption<V, +Drop<V>, +Serialize<V>> of Serialize<Option<V>> {
    fn serialize<S, +Drop<S>, +SerializerTrait<S>>(self: Option<V>, ref serializer: S) {
        match self {
            Option::Some(v) => serializer.serialize_some(v),
            Option::None => serializer.serialize_none(),
        }
    }
}

// ----------------------------------------------------------------
// Serializer functions
// ----------------------------------------------------------------

pub fn to_byte_array<T, +Drop<T>, +Serialize<T>>(value: T) -> ByteArray {
    let mut ser = Serializer { writer: "", formatter: CompactFormatter {} };
    value.serialize(ref ser);
    ser.writer
}

pub fn to_array_u8<T, +Drop<T>, +Serialize<T>>(value: T) -> Array<u8> {
    byte_array_to_array_u8(@to_byte_array(value))
}
