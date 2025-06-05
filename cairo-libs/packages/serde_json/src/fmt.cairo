use ibc_utils::bytes::ByteArrayIntoArrayU8;

pub trait FormatterTrait<F> {
    #[inline]
    fn begin_object(self: @F, ref writer: Array<u8>);
    #[inline]
    fn end_object(self: @F, ref writer: Array<u8>);
    #[inline]
    fn begin_string(self: @F, ref writer: Array<u8>);
    #[inline]
    fn end_string(self: @F, ref writer: Array<u8>);
    #[inline]
    fn end_object_key(self: @F, ref writer: Array<u8>);
    #[inline]
    fn end_object_value(self: @F, ref writer: Array<u8>);
    #[inline]
    fn write_null(self: @F, ref writer: Array<u8>);
    #[inline]
    fn write_bool(self: @F, ref writer: Array<u8>, value: bool);
    #[inline]
    fn write_u8(self: @F, ref writer: Array<u8>, value: u8);
    #[inline]
    fn write_u32(self: @F, ref writer: Array<u8>, value: u32);
    #[inline]
    fn write_u64(self: @F, ref writer: Array<u8>, value: u64);
    #[inline]
    fn write_u128(self: @F, ref writer: Array<u8>, value: u128);
    #[inline]
    fn write_u256(self: @F, ref writer: Array<u8>, value: u256);
    #[inline]
    fn write_felt252(self: @F, ref writer: Array<u8>, value: felt252);
    #[inline]
    fn write_string(
        self: @F, ref writer: Array<u8>, value: ByteArray,
    ) {
        writer.append_span(ByteArrayIntoArrayU8::into(value).span());
    }
}

#[derive(Clone, Drop)]
pub struct CompactFormatter {}

pub impl FormatterImpl of FormatterTrait<CompactFormatter> {
    fn begin_object(self: @CompactFormatter, ref writer: Array<u8>) {
        self.write_string(ref writer, "{");
    }

    fn end_object(self: @CompactFormatter, ref writer: Array<u8>) {
        self.write_string(ref writer, "}");
    }

    fn begin_string(self: @CompactFormatter, ref writer: Array<u8>) {
        self.write_string(ref writer, "\"");
    }

    fn end_string(self: @CompactFormatter, ref writer: Array<u8>) {
        self.write_string(ref writer, "\"");
    }

    fn end_object_key(self: @CompactFormatter, ref writer: Array<u8>) {
        self.write_string(ref writer, ":");
    }

    fn end_object_value(self: @CompactFormatter, ref writer: Array<u8>) {
        self.write_string(ref writer, ",");
    }

    fn write_null(self: @CompactFormatter, ref writer: Array<u8>) {
        self.write_string(ref writer, "null");
    }

    fn write_bool(self: @CompactFormatter, ref writer: Array<u8>, value: bool) {
        if value {
            self.write_string(ref writer, "true");
        } else {
            self.write_string(ref writer, "false");
        }
    }

    fn write_u8(self: @CompactFormatter, ref writer: Array<u8>, value: u8) {
        self.write_string(ref writer, format!("{value}"));
    }

    fn write_u32(self: @CompactFormatter, ref writer: Array<u8>, value: u32) {
        self.write_string(ref writer, format!("{value}"));
    }

    fn write_u64(self: @CompactFormatter, ref writer: Array<u8>, value: u64) {
        self.write_string(ref writer, format!("{value}"));
    }

    fn write_u128(self: @CompactFormatter, ref writer: Array<u8>, value: u128) {
        self.write_string(ref writer, format!("{value}"));
    }

    fn write_u256(self: @CompactFormatter, ref writer: Array<u8>, value: u256) {
        self.write_string(ref writer, format!("{value}"));
    }
    fn write_felt252(self: @CompactFormatter, ref writer: Array<u8>, value: felt252) {
        self.write_string(ref writer, format!("{value}"));
    }
}
