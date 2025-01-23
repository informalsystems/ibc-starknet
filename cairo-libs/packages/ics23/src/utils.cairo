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

pub fn array_u32_into_array_u8(input: Array<u32>) -> Array<u8> {
    let mut result: Array<u8> = ArrayTrait::new();
    for i in input {
        let a = i.to_bytes();
        result.append_span(a);
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
        array_u32_into_array_u8(self)
    }
}

pub impl SliceU32IntoArrayU32 of Into<[u32; 8], Array<u8>> {
    fn into(self: [u32; 8]) -> Array<u8> {
        let u32_array: Array<u32> = self.span().into();
        u32_array.into()
    }
}
