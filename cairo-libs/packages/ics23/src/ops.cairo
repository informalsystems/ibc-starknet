use core::sha256::{compute_sha256_u32_array, compute_sha256_byte_array};
use ics23::{
    InnerOp, LeafOp, HashOp, ICS23Errors, byte_array_to_array_u8, LengthOp, ArrayU32IntoArrayU8,
    SliceU32IntoArrayU32, IntoArrayU32,
};

pub fn apply_inner(inner: @InnerOp, child: [u32; 8]) -> [u32; 8] {
    // Sanity checks
    assert(inner.hash == @HashOp::Sha256, ICS23Errors::UNSUPPORTED_HASH_OP);
    assert(child != [0; 8], ICS23Errors::MISSING_CHILD_HASH);

    // Construct the data
    let mut data: Array<u8> = ArrayTrait::new();
    data.append_span(inner.prefix.span());
    let u8_child_array: Array<u8> = child.into();
    data.append_span(u8_child_array.span());
    data.append_span(inner.suffix.span());

    // Compute the hash
    let (bytes, last_word, last_word_len) = data.into_array_u32();
    compute_sha256_u32_array(bytes, last_word, last_word_len)
}

pub fn apply_leaf(leaf_op: @LeafOp, key: @ByteArray, value: Array<u32>) -> [u32; 8] {
    // Sanity check
    assert(leaf_op.hash == @HashOp::Sha256, ICS23Errors::UNSUPPORTED_HASH_OP);

    // Construct the data
    let mut data: Array<u8> = ArrayTrait::new();
    data.append_span(leaf_op.prefix.span());
    let prekey = prepare_leaf_byte_array(leaf_op.prehash_key, leaf_op.length, key);
    data.append_span(prekey.span());
    let preval = prepare_leaf_u32_array(leaf_op.prehash_value, leaf_op.length, value);
    data.append_span(preval.span());

    // Compute the hash
    let (bytes, last_word, last_word_len) = data.into_array_u32();
    compute_sha256_u32_array(bytes, last_word, last_word_len)
}

pub fn prepare_leaf_u32_array(prehash: @HashOp, length: @LengthOp, data: Array<u32>) -> Array<u8> {
    assert(data.len() > 0, ICS23Errors::MISSING_VALUE);
    do_length(length, hash_u32_array(prehash, data))
}

pub fn prepare_leaf_byte_array(prehash: @HashOp, length: @LengthOp, data: @ByteArray) -> Array<u8> {
    assert(data.len() > 0, ICS23Errors::MISSING_KEY);
    do_length(length, hash_byte_array(prehash, data))
}

pub fn hash_u32_array(hash_op: @HashOp, data: Array<u32>) -> Array<u8> {
    match hash_op {
        HashOp::NoOp => data.into(),
        HashOp::Sha256 => { compute_sha256_u32_array(data, 0, 0).into() }
    }
}

pub fn hash_byte_array(hash_op: @HashOp, data: @ByteArray) -> Array<u8> {
    match hash_op {
        HashOp::NoOp => byte_array_to_array_u8(data),
        HashOp::Sha256 => { compute_sha256_byte_array(data).into() }
    }
}

pub fn do_length(length_op: @LengthOp, data: Array<u8>) -> Array<u8> {
    match length_op {
        LengthOp::NoPrefix => data,
        LengthOp::VarProto => {
            let mut data = data;
            let mut len = proto_len(data.len());
            len.append_span(data.span());
            len
        }
    }
}

pub fn proto_len(length: u32) -> Array<u8> {
    let mut result: Array<u8> = ArrayTrait::new();
    let mut len = length;
    for _ in 0
        ..10_u32 {
            if len < 0x80 {
                result.append(len.try_into().unwrap());
                break;
            } else {
                let remaining_len = (len & 0x7F) | 0x80;
                result.append(remaining_len.try_into().unwrap());
                len /= 0x80;
            };
        };
    result
}
