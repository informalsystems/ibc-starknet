pub fn array_u8_into_array_u32(input: Array<u8>) -> (Array<u32>, u32, u32) {
    let mut result: Array<u32> = ArrayTrait::new();
    let mut last_word: u32 = 0;
    let mut last_word_len: u32 = 0;

    let mut i: usize = 0;
    while i < input.len() {
        let mut value: u32 = 0;
        let mut j: usize = 0;
        while j < 4 {
            if i + j >= input.len() {
                break;
            };
            value *= 0x100;
            value = value + (*input.at(i + j)).into();
            j += 1;
        };
        if j % 4 == 0 {
            result.append(value);
        } else {
            last_word = value;
            last_word_len = j.try_into().unwrap();
        }
        i += 4;
    };

    (result, last_word, last_word_len)
}


pub fn byte_array_to_slice_u32(input: ByteArray) -> [u32; 8] {
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

pub impl ArrayU8IntoArrayU32 of IntoArrayU32<Array<u8>> {
    fn into_array_u32(self: Array<u8>) -> (Array<u32>, u32, u32) {
        array_u8_into_array_u32(self)
    }
}

pub impl ByteArrayIntoArrayU32 of IntoArrayU32<ByteArray> {
    fn into_array_u32(self: ByteArray) -> (Array<u32>, u32, u32) {
        let bytes = byte_array_to_array_u8(@self);
        bytes.into_array_u32()
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

pub fn array_u32_into_array_u8(input: Array<u32>, last_word: u32, last_word_len: u32) -> Array<u8> {
    let mut result: Array<u8> = ArrayTrait::new();
    for i in input {
        let a = u32_to_bytes(i, ConvMode::FixedSize);
        result.append_span(a);
    };
    if last_word_len > 0 && last_word != 0 {
        result.append_span(u32_to_bytes(last_word, ConvMode::Compact));
    };
    result
}

pub fn byte_array_to_array_u8(input: @ByteArray) -> Array<u8> {
    let mut output: Array<u8> = array![];
    let mut i = 0;
    while i < input.len() {
        output.append(input[i]);
        i += 1;
    };
    output
}

pub impl ArrayU32IntoArrayU8 of Into<Array<u32>, Array<u8>> {
    fn into(self: Array<u32>) -> Array<u8> {
        array_u32_into_array_u8(self, 0, 0)
    }
}

pub impl SliceU32IntoArrayU8 of Into<[u32; 8], Array<u8>> {
    fn into(self: [u32; 8]) -> Array<u8> {
        let u32_array: Array<u32> = self.span().into();
        u32_array.into()
    }
}

pub impl ByteArrayIntoArrayU8 of Into<ByteArray, Array<u8>> {
    fn into(self: ByteArray) -> Array<u8> {
        byte_array_to_array_u8(@self)
    }
}

pub fn array_u8_to_byte_array(input: @Array<u8>) -> ByteArray {
    let mut output = "";
    let mut i = 0;
    while i < input.len() {
        output.append_byte(*input[i]);
        i += 1;
    };
    output
}

pub fn decode_hex(hex: @ByteArray) -> Array<u8> {
    let mut output: Array<u8> = array![];
    let len = hex.len();
    assert(len % 2 == 0, 'Invalid hex length');
    let mut i = 0;
    while i < len {
        let high = hex[i];
        let low = hex[i + 1];
        assert(is_valid_hex_char(high), 'Invalid hex character');
        assert(is_valid_hex_char(low), 'Invalid hex character');
        let high = if high >= 97 {
            high - 87
        } else {
            high - 48
        };
        let low = if low >= 97 {
            low - 87
        } else {
            low - 48
        };
        output.append(high * 16 + low);
        i += 2;
    };
    output
}

// Only accept lowercase hex characters
pub fn is_valid_hex_char(c: u8) -> bool {
    (c >= 48 && c <= 57) || (c >= 97 && c <= 102)
}

pub fn encode_hex(bytes: Array<u8>) -> ByteArray {
    let mut output = "";
    let hex_chars: ByteArray = "0123456789abcdef";
    for b in bytes {
        let high: u32 = (b / 16).try_into().unwrap();
        let low: u32 = (b % 16).try_into().unwrap();
        output.append_byte(hex_chars[high]);
        output.append_byte(hex_chars[low]);
    };
    output
}

pub fn u64_into_array_u32(value: u64) -> Array<u32> {
    let mut array: Array<u32> = ArrayTrait::new();
    let upper = (value / 0x100000000).try_into().unwrap();
    let lower = (value % 0x100000000).try_into().unwrap();
    array.append(upper);
    array.append(lower);
    array
}

pub impl U64IntoArrayU32 of IntoArrayU32<u64> {
    fn into_array_u32(self: u64) -> (Array<u32>, u32, u32) {
        (u64_into_array_u32(self), 0, 0)
    }
}

pub fn array_u32_to_byte_array(array: Array<u32>) -> ByteArray {
    array_u8_to_byte_array(@array_u32_into_array_u8(array, 0, 0))
}

