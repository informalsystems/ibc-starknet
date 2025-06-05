use ibc_utils::bytes::IntoArrayU32;

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

