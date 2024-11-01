pub trait FormatterTrait<F> {
    #[inline]
    fn begin_object(self: @F, ref writer: ByteArray);
    #[inline]
    fn end_object(self: @F, ref writer: ByteArray);
    #[inline]
    fn begin_string(self: @F, ref writer: ByteArray);
    #[inline]
    fn end_string(self: @F, ref writer: ByteArray);
    #[inline]
    fn end_object_key(self: @F, ref writer: ByteArray);
    #[inline]
    fn end_object_value(self: @F, ref writer: ByteArray);
    #[inline]
    fn write_null(self: @F, ref writer: ByteArray);
    #[inline]
    fn write_bool(self: @F, ref writer: ByteArray, value: bool);
    #[inline]
    fn write_u8(self: @F, ref writer: ByteArray, value: u8);
    #[inline]
    fn write_u32(self: @F, ref writer: ByteArray, value: u32);
    #[inline]
    fn write_u64(self: @F, ref writer: ByteArray, value: u64);
    #[inline]
    fn write_u128(self: @F, ref writer: ByteArray, value: u128);
    #[inline]
    fn write_u256(self: @F, ref writer: ByteArray, value: u256);
    #[inline]
    fn write_felt252(self: @F, ref writer: ByteArray, value: felt252);
    #[inline]
    fn write_string(self: @F, ref writer: ByteArray, value: @ByteArray) {
        writer.append(value);
    }
}

#[derive(Clone, Drop)]
pub struct CompactFormatter {}

pub impl FormatterImpl of FormatterTrait<CompactFormatter> {
    fn begin_object(self: @CompactFormatter, ref writer: ByteArray) {
        writer.append(@"{");
    }

    fn end_object(self: @CompactFormatter, ref writer: ByteArray) {
        writer.append(@"}");
    }

    fn begin_string(self: @CompactFormatter, ref writer: ByteArray) {
        writer.append(@"\"");
    }

    fn end_string(self: @CompactFormatter, ref writer: ByteArray) {
        writer.append(@"\"");
    }

    fn end_object_key(self: @CompactFormatter, ref writer: ByteArray) {
        writer.append(@":");
    }

    fn end_object_value(self: @CompactFormatter, ref writer: ByteArray) {
        writer.append(@",");
    }

    fn write_null(self: @CompactFormatter, ref writer: ByteArray) {
        writer.append(@"null");
    }

    fn write_bool(self: @CompactFormatter, ref writer: ByteArray, value: bool) {
        if value {
            writer.append(@"true");
        } else {
            writer.append(@"false");
        }
    }

    fn write_u8(self: @CompactFormatter, ref writer: ByteArray, value: u8) {
        writer.append(@format!("{value}"));
    }

    fn write_u32(self: @CompactFormatter, ref writer: ByteArray, value: u32) {
        writer.append(@format!("{value}"));
    }

    fn write_u64(self: @CompactFormatter, ref writer: ByteArray, value: u64) {
        writer.append(@format!("{value}"));
    }

    fn write_u128(self: @CompactFormatter, ref writer: ByteArray, value: u128) {
        writer.append(@format!("{value}"));
    }

    fn write_u256(self: @CompactFormatter, ref writer: ByteArray, value: u256) {
        writer.append(@format!("{value}"));
    }
    fn write_felt252(self: @CompactFormatter, ref writer: ByteArray, value: felt252) {
        writer.append(@format!("{value}"));
    }
}
