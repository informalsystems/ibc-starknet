use core::sha256::compute_sha256_u32_array;
use ibc_utils::bytes::{IntoArrayU32, SpanU32IntoArrayU8, SpanU8IntoArrayU32};
use ics23::{HashOp, ICS23Errors, InnerOp, KeyBytes, LeafOp, LengthOp, ValueBytes};
use protobuf::varint::encode_varint_to_u8_array;

pub fn apply_inner(inner: @InnerOp, child: Array<u8>) -> [u32; 8] {
    // Sanity checks
    assert(inner.hash == @HashOp::Sha256, ICS23Errors::UNSUPPORTED_HASH_OP);
    assert(child.len() > 0, ICS23Errors::MISSING_CHILD_HASH);

    // Construct the data
    let mut data: Array<u8> = ArrayTrait::new();
    data.append_span(inner.prefix.span());
    data.append_span(child.span());
    data.append_span(inner.suffix.span());

    // Compute the hash
    let (bytes, last_word, last_word_len) = data.span().into_array_u32();
    compute_sha256_u32_array(bytes, last_word, last_word_len)
}

pub fn apply_leaf(leaf_op: @LeafOp, key: KeyBytes, value: ValueBytes) -> [u32; 8] {
    // Sanity check
    assert(leaf_op.hash == @HashOp::Sha256, ICS23Errors::UNSUPPORTED_HASH_OP);

    // Construct the data
    let mut data: Array<u8> = ArrayTrait::new();
    data.append_span(leaf_op.prefix.span());
    let prekey = prepare_leaf_u32_array(leaf_op.prehash_key, leaf_op.length, key);
    data.append_span(prekey.span());
    let preval = prepare_leaf_u32_array(leaf_op.prehash_value, leaf_op.length, value);
    data.append_span(preval.span());

    // Compute the hash
    let (bytes, last_word, last_word_len) = data.span().into_array_u32();
    compute_sha256_u32_array(bytes, last_word, last_word_len)
}

pub fn prepare_leaf_u32_array(prehash: @HashOp, length: @LengthOp, data: Array<u8>) -> Array<u8> {
    assert(data.len() > 0, ICS23Errors::MISSING_LEAF_DATA);
    do_length(length, do_hash(prehash, data))
}

pub fn do_hash(hash_op: @HashOp, data: Array<u8>) -> Array<u8> {
    match hash_op {
        HashOp::NoOp => { data },
        HashOp::Sha256 => {
            let (bytes, last_word, last_word_len) = data.span().into_array_u32();
            compute_sha256_u32_array(bytes, last_word, last_word_len).span().into()
        },
    }
}

pub fn do_length(length_op: @LengthOp, data: Array<u8>) -> Array<u8> {
    match length_op {
        LengthOp::NoPrefix => data,
        LengthOp::VarProto => {
            let mut data = data;
            let mut len = encode_varint_to_u8_array(data.len().into());
            len.append_span(data.span());
            len
        },
    }
}

