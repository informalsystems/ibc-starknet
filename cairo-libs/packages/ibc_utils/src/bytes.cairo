use crate::numeric::{u32_to_u8, u64_into_array_u32};

pub impl ByteArrayIntoArrayU8 of Into<ByteArray, Array<u8>> {
    fn into(self: ByteArray) -> Array<u8> {
        let mut output = array![];
        let mut i = 0;
        while let Some(byte) = self.at(i) {
            output.append(byte);
            i += 1;
        }
        output
    }
}

pub impl SpanU8IntoByteArray of Into<Span<u8>, ByteArray> {
    fn into(mut self: Span<u8>) -> ByteArray {
        let mut output = "";
        while let Some(input) = self.pop_front() {
            output.append_byte(*input);
        }
        output
    }
}

pub fn span_u8_into_array_u32(mut input: Span<u8>) -> (Array<u32>, u32, u32) {
    let input_len = input.len();
    let mut result: Array<u32> = array![];

    while let Some(tuple) = input.multi_pop_front() {
        let [value1, value2, value3, value4] = (*tuple).unbox();

        let value: u32 = value1.into() * 0x1000000
            + value2.into() * 0x10000
            + value3.into() * 0x100
            + value4.into();

        result.append(value);
    }

    let mut last_word: u32 = 0;
    let last_word_len = input_len % 4;

    if last_word_len > 0 {
        while let Some(value) = input.pop_front() {
            last_word *= 0x100;
            last_word = last_word + (*value).into();
        }
    }

    (result, last_word, last_word_len)
}

pub fn array_u8_to_slice_u32(input: Span<u8>) -> [u32; 8] {
    let (b, word, word_len) = input.into_array_u32();
    assert(word == 0 && word_len == 0, 'invalid byte array');
    assert(b.len() == 8, 'invalid byte array');
    [*b[0], *b[1], *b[2], *b[3], *b[4], *b[5], *b[6], *b[7]]
}


/// Converts the give type `T` into an array of `u32` values. If the last word
/// is not a full word, the method returns the last word and its length.
pub trait IntoArrayU32<T> {
    fn into_array_u32(self: T) -> (Array<u32>, u32, u32);
}

pub impl SpanU8IntoArrayU32 of IntoArrayU32<Span<u8>> {
    fn into_array_u32(self: Span<u8>) -> (Array<u32>, u32, u32) {
        span_u8_into_array_u32(self)
    }
}

#[derive(Drop, Debug)]
pub enum ConvMode {
    #[default]
    FixedSize,
    Compact,
}

/// Converts a `u32` into a byte span based on the given mode.
///
/// - `ConvMode::FixedSize` → Always returns 4 bytes, preserving the full size.
/// - `ConvMode::Compact` → Returns minimal number of bytes to represent the value.
pub fn u32_to_bytes(self: u32, mode: ConvMode) -> Span<u8> {
    let val0: u8 = (self & 0xFF).try_into().unwrap();
    let mut val1 = self & 0xFF00;
    let mut val2 = self & 0xFF0000;
    let mut val3 = self & 0xFF000000;
    if val3 != 0 {
        return array![
            (val3 / 0x1000000).try_into().unwrap(),
            (val2 / 0x10000).try_into().unwrap(),
            (val1 / 0x100).try_into().unwrap(),
            val0,
        ]
            .span();
    }

    if val2 != 0 {
        let val2 = (val2 / 0x10000).try_into().unwrap();
        let val1 = (val1 / 0x100).try_into().unwrap();
        match mode {
            ConvMode::FixedSize => { return array![0, val2, val1, val0].span(); },
            ConvMode::Compact => { return array![val2, val1, val0].span(); },
        }
    }

    if val1 != 0 {
        let val1 = (val1 / 0x100).try_into().unwrap();
        match mode {
            ConvMode::FixedSize => { return array![0, 0, val1, val0].span(); },
            ConvMode::Compact => { return array![val1, val0].span(); },
        }
    }

    match mode {
        ConvMode::FixedSize => { return array![0, 0, 0, val0].span(); },
        ConvMode::Compact => { return array![val0].span(); },
    }
}


pub fn array_u32_into_array_u8(
    mut input: Span<u32>, last_word: u32, last_word_len: u32,
) -> Array<u8> {
    let mut result: Array<u8> = array![];
    while let Some(i) = input.pop_front() {
        let a = u32_to_bytes(*i, ConvMode::FixedSize);
        result.append_span(a);
    }
    if last_word_len != 0 && last_word != 0 {
        result.append_span(u32_to_bytes(last_word, ConvMode::Compact));
    }
    result
}


pub impl SpanU32IntoArrayU8 of Into<Span<u32>, Array<u8>> {
    fn into(mut self: Span<u32>) -> Array<u8> {
        let mut result: Array<u8> = array![];
        while let Some(i) = self.pop_front() {
            let (b0, b1, b2, b3) = u32_to_u8(*i);
            result.append(b0);
            result.append(b1);
            result.append(b2);
            result.append(b3);
        }
        result
    }
}

pub impl SpanU32IntoByteArray of Into<Span<u32>, ByteArray> {
    fn into(mut self: Span<u32>) -> ByteArray {
        let mut result: ByteArray = "";
        while let Some(i) = self.pop_front() {
            let (b0, b1, b2, b3) = u32_to_u8(*i);
            result.append_byte(b0);
            result.append_byte(b1);
            result.append_byte(b2);
            result.append_byte(b3);
        }
        result
    }
}

pub impl U64IntoArrayU32 of IntoArrayU32<u64> {
    fn into_array_u32(self: u64) -> (Array<u32>, u32, u32) {
        (u64_into_array_u32(self), 0, 0)
    }
}

pub impl SpanU8TryIntoU256 of TryInto<Span<u8>, u256> {
    /// Decodes as big endian.
    fn try_into(self: Span<u8>) -> Option<u256> {
        // Only allows size 32 to ensure all bytes fit exactly into a `u256`.
        if (self.len() != 32) {
            return Option::None;
        }

        const N256: u128 = 0x100;

        // No loop overhead with manual unrolling.
        let mut ret: u256 = 0;
        ret.high = ret.high * N256 + (*self[0]).into();
        ret.high = ret.high * N256 + (*self[1]).into();
        ret.high = ret.high * N256 + (*self[2]).into();
        ret.high = ret.high * N256 + (*self[3]).into();
        ret.high = ret.high * N256 + (*self[4]).into();
        ret.high = ret.high * N256 + (*self[5]).into();
        ret.high = ret.high * N256 + (*self[6]).into();
        ret.high = ret.high * N256 + (*self[7]).into();
        ret.high = ret.high * N256 + (*self[8]).into();
        ret.high = ret.high * N256 + (*self[9]).into();
        ret.high = ret.high * N256 + (*self[10]).into();
        ret.high = ret.high * N256 + (*self[11]).into();
        ret.high = ret.high * N256 + (*self[12]).into();
        ret.high = ret.high * N256 + (*self[13]).into();
        ret.high = ret.high * N256 + (*self[14]).into();
        ret.high = ret.high * N256 + (*self[15]).into();

        ret.low = ret.low * N256 + (*self[16]).into();
        ret.low = ret.low * N256 + (*self[17]).into();
        ret.low = ret.low * N256 + (*self[18]).into();
        ret.low = ret.low * N256 + (*self[19]).into();
        ret.low = ret.low * N256 + (*self[20]).into();
        ret.low = ret.low * N256 + (*self[21]).into();
        ret.low = ret.low * N256 + (*self[22]).into();
        ret.low = ret.low * N256 + (*self[23]).into();
        ret.low = ret.low * N256 + (*self[24]).into();
        ret.low = ret.low * N256 + (*self[25]).into();
        ret.low = ret.low * N256 + (*self[26]).into();
        ret.low = ret.low * N256 + (*self[27]).into();
        ret.low = ret.low * N256 + (*self[28]).into();
        ret.low = ret.low * N256 + (*self[29]).into();
        ret.low = ret.low * N256 + (*self[30]).into();
        ret.low = ret.low * N256 + (*self[31]).into();

        Option::Some(ret)
    }
}


pub impl Slice8U32IntoArrayU8 of Into<[u32; 8], Array<u8>> {
    fn into(self: [u32; 8]) -> Array<u8> {
        let mut array_u8 = array![];
        let mut span = self.span();
        while let Some(elem) = span.pop_front() {
            let word = *elem;
            array_u8.append(((word / 0x1000000) & 0xFF).try_into().unwrap());
            array_u8.append(((word / 0x10000) & 0xFF).try_into().unwrap());
            array_u8.append(((word / 0x100) & 0xFF).try_into().unwrap());
            array_u8.append((word & 0xFF).try_into().unwrap());
        }
        array_u8
    }
}


#[cfg(test)]
mod tests {
    use super::SpanU32IntoArrayU8;

    #[test]
    fn test_u32_8_to_array_u8() {
        let ar_u32 = [
            0x12345678, 0x9ABCDEF0, 0x12345678, 0x9ABCDEF0, 0x12345678, 0x9ABCDEF0, 0x12345678,
            0x9ABCDEF0,
        ];
        let mut ar_u8 = array![];

        ar_u8.append(0x12);
        ar_u8.append(0x34);
        ar_u8.append(0x56);
        ar_u8.append(0x78);
        ar_u8.append(0x9A);
        ar_u8.append(0xBC);
        ar_u8.append(0xDE);
        ar_u8.append(0xF0);

        ar_u8.append(0x12);
        ar_u8.append(0x34);
        ar_u8.append(0x56);
        ar_u8.append(0x78);
        ar_u8.append(0x9A);
        ar_u8.append(0xBC);
        ar_u8.append(0xDE);
        ar_u8.append(0xF0);

        ar_u8.append(0x12);
        ar_u8.append(0x34);
        ar_u8.append(0x56);
        ar_u8.append(0x78);
        ar_u8.append(0x9A);
        ar_u8.append(0xBC);
        ar_u8.append(0xDE);
        ar_u8.append(0xF0);

        ar_u8.append(0x12);
        ar_u8.append(0x34);
        ar_u8.append(0x56);
        ar_u8.append(0x78);
        ar_u8.append(0x9A);
        ar_u8.append(0xBC);
        ar_u8.append(0xDE);
        ar_u8.append(0xF0);

        assert_eq!(ar_u8, SpanU32IntoArrayU8::into(ar_u32.span()));
    }
}
