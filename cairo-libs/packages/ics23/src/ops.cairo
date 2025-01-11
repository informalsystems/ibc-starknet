use core::sha256::{compute_sha256_u32_array, compute_sha256_byte_array};
use ics23::{
    InnerOp, LeafOp, HashOp, ICS23Errors, array_u8_into_array_u32, array_u32_into_array_u8, LengthOp
};

pub fn apply_inner(inner: @InnerOp, child: [u32; 8]) -> [u32; 8] {
    assert(inner.hash == @HashOp::Sha256, ICS23Errors::UNSUPPORTED_HASH_OP);
    assert(child != [0; 8], ICS23Errors::MISSING_CHILD_HASH);
    let mut data: Array<u8> = ArrayTrait::new();
    data.append_span(inner.prefix.span());
    let child_as_u32_array: Array<u32> = child.span().into();
    data.append_span(array_u32_into_array_u8(child_as_u32_array).span());
    data.append_span(inner.suffix.span());
    let (bytes, last_word, last_word_len) = array_u8_into_array_u32(data);
    compute_sha256_u32_array(bytes, last_word, last_word_len)
}

pub fn apply_leaf(leaf_op: @LeafOp, key: @ByteArray, value: Array<u32>) -> [u32; 8] {
    assert(leaf_op.hash == @HashOp::Sha256, ICS23Errors::UNSUPPORTED_HASH_OP);
    assert(key.len() > 0, ICS23Errors::MISSING_KEY);
    assert(value.len() > 0, ICS23Errors::MISSING_VALUE);
    let (mut data, last_word, last_word_len) = array_u8_into_array_u32(leaf_op.prefix.clone());
    let prekey = prepare_leaf_byte_array_data(leaf_op.prehash_key, leaf_op.length, key);
    data.append_span(prekey.span());
    let preval = prepare_leaf_u32_array_data(leaf_op.prehash_value, leaf_op.length, value);
    data.append_span(preval.span());
    compute_sha256_u32_array(data, last_word, last_word_len)
}

pub fn prepare_leaf_u32_array_data(
    prehash: @HashOp, length: @LengthOp, data: Array<u32>
) -> Array<u32> {
    assert(data.len() > 0, ICS23Errors::MISSING_VALUE);
    assert(prehash == @HashOp::Sha256, ICS23Errors::UNSUPPORTED_HASH_OP);
    let h = compute_sha256_u32_array(data, 0, 0);
    calc_length(length, h.span().into())
}

pub fn prepare_leaf_byte_array_data(
    prehash: @HashOp, length: @LengthOp, data: @ByteArray
) -> Array<u32> {
    assert(data.len() > 0, ICS23Errors::MISSING_VALUE);
    assert(prehash == @HashOp::Sha256, ICS23Errors::UNSUPPORTED_HASH_OP);
    let h = compute_sha256_byte_array(data);
    calc_length(length, h.span().into())
}

pub fn calc_length(length_op: @LengthOp, data: Array<u32>) -> Array<u32> {
    match length_op {
        LengthOp::NoPrefix => data,
        LengthOp::VarProto => {
            let mut data = data;
            data.append(data.len());
            data
        }
    }
}
