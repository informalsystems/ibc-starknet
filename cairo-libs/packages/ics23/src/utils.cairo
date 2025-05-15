pub fn array_u8_into_array_u32(input: Array<u8>) -> (Array<u32>, u32, u32) {
    let input_len = input.len();
    let mut result: Array<u32> = ArrayTrait::new();
    let mut last_word: u32 = 0;

    let last_word_len = input_len % 4;
    let truncated_len = input_len / 4;

    let mut input_span = input.span();

    for _ in 0..truncated_len {
        let value1: u32 = (*input_span.pop_front().unwrap()).into() * 0x1000000;
        let value2: u32 = (*input_span.pop_front().unwrap()).into() * 0x10000;
        let value3: u32 = (*input_span.pop_front().unwrap()).into() * 0x100;
        let value4: u32 = (*input_span.pop_front().unwrap()).into();
        let value: u32 = value1 + value2 + value3 + value4;

        result.append(value);
    }

    if last_word_len > 0 {
        while let Option::Some(value) = input_span.pop_front() {
            last_word *= 0x100;
            last_word = last_word + (*value).into();
        }
    }

    (result, last_word, last_word_len)
}

pub fn byte_array_to_slice_u32(input: ByteArray) -> [u32; 8] {
    let (b, word, word_len) = input.into_array_u32();
    assert(word == 0 && word_len == 0, 'invalid byte array');
    assert(b.len() == 8, 'invalid byte array');
    [*b[0], *b[1], *b[2], *b[3], *b[4], *b[5], *b[6], *b[7]]
}

pub fn slice_u32_to_byte_array(input: [u32; 8]) -> ByteArray {
    let ar: Array<u8> = input.into();
    array_u8_to_byte_array(@ar)
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
    }
    if last_word_len > 0 && last_word != 0 {
        result.append_span(u32_to_bytes(last_word, ConvMode::Compact));
    }
    result
}

pub fn byte_array_to_array_u8(input: @ByteArray) -> Array<u8> {
    let mut output: Array<u8> = array![];
    let mut i = 0;
    while let Option::Some(value) = input.at(i) {
        output.append(value);
        i += 1;
    }
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
    let mut input_span = input.span();
    while let Option::Some(input) = input_span.pop_front() {
        output.append_byte(*input);
    }
    output
}

pub fn decode_hex(hex: @ByteArray) -> Array<u8> {
    let mut output: Array<u8> = array![];
    let len = hex.len();
    assert(len % 2 == 0, 'Invalid hex length');
    // Since len % 2 == 0, we know i += 2 will eventually be
    // equal to len
    let mut i = 0;
    while i != len {
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
    }
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
    }
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


// ---------------------------------------------------------------
// Implementation of partial ordering for `Array<u8>`
// ---------------------------------------------------------------

pub impl ArrayU8PartialOrd of PartialOrd<Array<u8>> {
    fn le(lhs: Array<u8>, rhs: Array<u8>) -> bool {
        lexicographical_cmp(lhs, rhs) != Ordering::Greater
    }
    fn ge(lhs: Array<u8>, rhs: Array<u8>) -> bool {
        lexicographical_cmp(lhs, rhs) != Ordering::Less
    }
    fn lt(lhs: Array<u8>, rhs: Array<u8>) -> bool {
        lexicographical_cmp(lhs, rhs) == Ordering::Less
    }
    fn gt(lhs: Array<u8>, rhs: Array<u8>) -> bool {
        lexicographical_cmp(lhs, rhs) == Ordering::Greater
    }
}

#[derive(Drop, Debug, PartialEq)]
pub enum Ordering {
    Equal,
    Less,
    Greater,
}

/// Lexicographical comparison of two `u8` arrays.
pub fn lexicographical_cmp(lhs: Array<u8>, rhs: Array<u8>) -> Ordering {
    let lhs_len = lhs.len();
    let rhs_len = rhs.len();
    let mut lhs_span = lhs.span();
    let mut rhs_span = rhs.span();

    let mut ordering = Ordering::Equal;

    while let (Some(l), Some(r)) = (lhs_span.pop_front(), rhs_span.pop_front()) {
        if l < r {
            ordering = Ordering::Less;
            break;
        } else if l > r {
            ordering = Ordering::Greater;
            break;
        }
    }

    if ordering != Ordering::Equal {
        return ordering;
    }

    if lhs_len < rhs_len {
        ordering = Ordering::Less
    }

    if lhs_len > rhs_len {
        ordering = Ordering::Greater
    }

    ordering
}

pub fn felt252_to_u8_array(value: felt252) -> ByteArray {
    let mut value_bytes: Array<u8> = array![];
    let mut i = 0;
    let mut current_value: u256 = value.into();
    while current_value != 0 && i != 31 {
        let low = current_value % 0x100;
        let lsb_u8: u8 = low.try_into().unwrap();
        value_bytes.append(lsb_u8);
        i += 1;
        current_value = current_value / 0x100;
    }
    let reversed_value_bytes = reverse_array(value_bytes);
    array_u8_to_byte_array(@reversed_value_bytes)
}

fn reverse_array(input: Array<u8>) -> Array<u8> {
    let mut input_span = input.span();
    let mut reverse: Array<u8> = array![];

    while let Option::Some(value) = input_span.pop_back() {
        reverse.append(value.clone());
    }
    reverse
}

pub fn u32_from_u8(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
    let b0 = b0.into() * 0x1_00_00_00;
    let b1 = b1.into() * 0x1_00_00;
    let b2 = b2.into() * 0x1_00;
    let b3 = b3.into();
    b0 | b1 | b2 | b3
}

pub fn u32_to_u8(value: u32) -> (u8, u8, u8, u8) {
    let b0 = (value / 0x1_00_00_00).try_into().unwrap();
    let b1 = (value / 0x1_00_00).try_into().unwrap();
    let b2 = (value / 0x1_00).try_into().unwrap();
    let b3 = value.try_into().unwrap();
    (b0, b1, b2, b3)
}

pub fn span_contains<T, +PartialEq<T>>(mut span: Span<T>, value: @T) -> bool {
    let mut result = false;
    while let Option::Some(v) = span.pop_front() {
        if v == value {
            result = true;
            break;
        }
    }
    result
}
