use ics23::{
    InnerOp, LeafOp, LengthOp, HashOp, apply_leaf, apply_inner, encode_hex, decode_hex,
    SliceU32IntoArrayU8, ByteArrayIntoArrayU32, byte_array_to_array_u8, ByteArrayIntoArrayU8,
};

// https://github.com/cosmos/ics23/blob/c7c728879896fb260fe76b208ea6a17c2b0132a3/rust/src/ops.rs#L210
#[test]
fn test_apply_leaf_hash() {
    let leaf = LeafOp {
        hash: HashOp::Sha256,
        prehash_key: HashOp::NoOp,
        prehash_value: HashOp::NoOp,
        length: LengthOp::NoPrefix,
        prefix: array![],
    };
    let key: ByteArray = "foo";
    let value: ByteArray = "bar";
    let hash = apply_leaf(@leaf, key.into(), value.into());

    assert_eq!(
        encode_hex(hash.into()), "c3ab8ff13720e8ad9047dd39466b3c8974e592c2fa383d4a3960714caef0c4f2",
    );
}

// https://github.com/cosmos/ics23/blob/c7c728879896fb260fe76b208ea6a17c2b0132a3/rust/src/ops.rs#L246
#[test]
fn test_apply_leaf_hash_length() {
    let leaf = LeafOp {
        hash: HashOp::Sha256,
        prehash_key: HashOp::NoOp,
        prehash_value: HashOp::NoOp,
        length: LengthOp::VarProto,
        prefix: array![],
    };
    let key: ByteArray = "food";
    let value: ByteArray = "some longer text";
    let hash = apply_leaf(@leaf, key.into(), value.into());

    assert_eq!(
        encode_hex(hash.into()), "b68f5d298e915ae1753dd333da1f9cf605411a5f2e12516be6758f365e6db265",
    );
}

// https://github.com/cosmos/ics23/blob/c7c728879896fb260fe76b208ea6a17c2b0132a3/rust/src/ops.rs#L282
#[test]
fn test_apply_leaf_prehash_value() {
    let leaf = LeafOp {
        hash: HashOp::Sha256,
        prehash_key: HashOp::NoOp,
        prehash_value: HashOp::Sha256,
        length: LengthOp::VarProto,
        prefix: array![],
    };
    let key: ByteArray = "food";
    let value: ByteArray = "yet another long string";
    let hash = apply_leaf(@leaf, key.into(), value.into());

    assert_eq!(
        encode_hex(hash.into()), "87e0483e8fb624aef2e2f7b13f4166cda485baa8e39f437c83d74c94bedb148f",
    );
}

// https://github.com/cosmos/ics23/blob/c7c728879896fb260fe76b208ea6a17c2b0132a3/rust/src/ops.rs#L299
#[test]
fn test_apply_inner_prefix_suffix() {
    let inner = InnerOp {
        hash: HashOp::Sha256, prefix: decode_hex(@"0123456789"), suffix: decode_hex(@"deadbeef"),
    };
    let child = decode_hex(@"00cafe00");
    let hash = apply_inner(@inner, child);

    assert_eq!(
        encode_hex(hash.into()), "0339f76086684506a6d42a60da4b5a719febd4d96d8b8d85ae92849e3a849a5e",
    );
}

// https://github.com/cosmos/ics23/blob/c7c728879896fb260fe76b208ea6a17c2b0132a3/rust/src/ops.rs#L299
#[test]
fn test_apply_inner_prefix_only() {
    let inner = InnerOp {
        hash: HashOp::Sha256, prefix: decode_hex(@"00204080a0c0e0"), suffix: array![],
    };
    let child = decode_hex(@"ffccbb997755331100");
    let hash = apply_inner(@inner, child);

    assert_eq!(
        encode_hex(hash.into()), "45bece1678cf2e9f4f2ae033e546fc35a2081b2415edcb13121a0e908dca1927",
    );
}

// https://github.com/cosmos/ics23/blob/c7c728879896fb260fe76b208ea6a17c2b0132a3/rust/src/ops.rs#L316
#[test]
fn test_apply_inner_suffix_only() {
    let inner = InnerOp {
        hash: HashOp::Sha256, prefix: array![], suffix: byte_array_to_array_u8(@" just kidding!"),
    };
    let child = byte_array_to_array_u8(@"this is a sha256 hash, really....");
    let hash = apply_inner(@inner, child);

    assert_eq!(
        encode_hex(hash.into()), "79ef671d27e42a53fba2201c1bbc529a099af578ee8a38df140795db0ae2184b",
    );
}

