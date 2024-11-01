#[derive(Drop, Clone)]
pub struct U32Collector {
    pub value: Array<u32>,
}

#[generate_trait]
pub impl U32CollectorImpl of U32CollectorTrait {
    fn init() -> U32Collector {
        U32Collector { value: ArrayTrait::new() }
    }

    fn append(ref self: U32Collector, value: u32) {
        self.value.append(value);
    }

    // NOTE: This method does not capture the last word if it is incomplete.
    // Refactor the logic if necessary in the future.
    fn extend<T, +IntoArrayU32<T>>(ref self: U32Collector, value: T) {
        let (array, _, _) = value.into_array_u32();
        self.value.append_span(array.span());
    }

    fn extend_from_chunk(ref self: U32Collector, slice: [u32; 8]) {
        self.value.append_span(slice.span());
    }

    fn value(self: U32Collector) -> Array<u32> {
        self.value
    }
}

/// Converts the give type `T` into an array of `u32` values. If the last word
/// is not a full word, the method returns the last word and its length.
pub trait IntoArrayU32<T> {
    fn into_array_u32(self: T) -> (Array<u32>, u32, u32);
}

pub impl U64IntoArrayU32 of IntoArrayU32<u64> {
    fn into_array_u32(self: u64) -> (Array<u32>, u32, u32) {
        (u64_into_array_u32(self), 0, 0)
    }
}

pub fn u64_into_array_u32(value: u64) -> Array<u32> {
    let mut array: Array<u32> = ArrayTrait::new();
    let upper = (value / 0x100000000).try_into().unwrap();
    let lower = (value % 0x100000000).try_into().unwrap();
    array.append(upper);
    array.append(lower);
    array
}

pub impl ArrayU8IntoArrayU32 of IntoArrayU32<Array<u8>> {
    fn into_array_u32(self: Array<u8>) -> (Array<u32>, u32, u32) {
        array_u8_into_array_u32(self)
    }
}

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
