use alexandria_numeric::integers::UIntBytes;

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

pub fn array_u32_into_array_u8(input: Array<u32>, last_word: u32, last_word_len: u32) -> Array<u8> {
    let mut result: Array<u8> = ArrayTrait::new();
    for i in input {
        let a = i.to_bytes();
        result.append_span(a);
    };
    if last_word_len > 0 && last_word != 0 {
        result.append_span(last_word.to_bytes());
    };
    result
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

pub impl ByteArrayIntoArrayU32 of IntoArrayU32<ByteArray> {
    fn into_array_u32(self: ByteArray) -> (Array<u32>, u32, u32) {
        let bytes = byte_array_to_array_u8(@self);
        bytes.into_array_u32()
    }
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

pub fn array_u8_to_byte_array(input: @Array<u8>) -> ByteArray {
    let mut output = "";
    let mut i = 0;
    while i < input.len() {
        output.append_byte(*input[i]);
        i += 1;
    };
    output
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

