use ics23::{
    LeafOp, LengthOp, HashOp, InnerOp, ExistenceProof, ExistenceProofImpl, SliceU32IntoArrayU8,
    byte_array_to_array_u8, encode_hex, decode_hex
};

#[test]
fn test_calculate_root_from_leaf() {
    let leaf = LeafOp {
        hash: HashOp::Sha256,
        prehash_key: HashOp::NoOp,
        prehash_value: HashOp::NoOp,
        length: LengthOp::VarProto,
        prefix: array![],
    };

    let proof = ExistenceProof {
        key: "food", value: byte_array_to_array_u8(@"some longer text"), leaf, path: array![],
    };

    let root = proof.calculate_root();

    assert_eq!(
        encode_hex(root.into()), "b68f5d298e915ae1753dd333da1f9cf605411a5f2e12516be6758f365e6db265"
    );
}

#[test]
fn test_calculate_root_from_leaf_and_inner() {
    let leaf = LeafOp {
        hash: HashOp::Sha256,
        prehash_key: HashOp::NoOp,
        prehash_value: HashOp::NoOp,
        length: LengthOp::VarProto,
        prefix: array![]
    };

    let inner = InnerOp {
        hash: HashOp::Sha256, prefix: decode_hex(@"deadbeef00cafe00"), suffix: array![]
    };

    let proof = ExistenceProof {
        key: "food", value: byte_array_to_array_u8(@"some longer text"), leaf, path: array![inner],
    };

    let root = proof.calculate_root();

    assert_eq!(
        encode_hex(root.into()), "836ea236a6902a665c2a004c920364f24cad52ded20b1e4f22c3179bfe25b2a9"
    );
}
