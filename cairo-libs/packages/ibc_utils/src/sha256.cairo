use core::sha256::compute_sha256_u32_array;
use crate::bytes::{SpanU32IntoArrayU8, span_u8_into_array_u32};

pub fn compute_sha256_span_u8(input: Span<u8>) -> [u32; 8] {
    let (word_arr, last, rem) = span_u8_into_array_u32(input);
    compute_sha256_u32_array(word_arr, last, rem.into())
}

pub fn compute_sha256_span_u8_to_u8(input: Span<u8>) -> Array<u8> {
    compute_sha256_span_u8(input).span().into()
}
