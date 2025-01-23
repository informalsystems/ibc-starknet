use ics23::IntoArrayU32;

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
