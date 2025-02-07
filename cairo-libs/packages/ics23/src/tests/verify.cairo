use ics23::{
    LeafOp, LengthOp, HashOp, InnerOp, ExistenceProof, ExistenceProofImpl, SliceU32IntoArrayU8,
    Proof, encode_hex, decode_hex, ProofSpec, smt_spec, ByteArrayIntoArrayU8, CommitmentProof,
    verify_existence, verify_non_existence, byte_array_to_slice_u32
};
use ics23::tests::data::{TestData, smt_exist_left, smt_exist_right, smt_exist_middle};
use protobuf::types::message::ProtoCodecImpl;
use protobuf::hex::decode as decode_hex_byte_array;

fn decode_and_verify(data: @TestData, spec: @ProofSpec) {
    let root = byte_array_to_slice_u32(decode_hex_byte_array(data.root));
    let key = decode_hex_byte_array(data.key).into();
    let value = decode_hex_byte_array(data.value).into();
    let p = ProtoCodecImpl::decode::<CommitmentProof>(@decode_hex_byte_array(data.proof));
    match p.proof {
        Proof::Exist(p) => { verify_existence(spec, @p, @root, @key, @value); },
        Proof::NonExist(p) => {
            assert(value.len() == 0, 'value must not exist');
            verify_non_existence(spec, @p, @root, @key);
        }
    };
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L543
#[test]
fn test_vector_smt_left() {
    decode_and_verify(@smt_exist_left(), @smt_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L550
#[test]
fn test_vector_smt_right() {
    decode_and_verify(@smt_exist_right(), @smt_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L557
#[test]
fn test_vector_smt_middle() {
    decode_and_verify(@smt_exist_middle(), @smt_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/verify.rs#L381
#[test]
fn test_calculate_root_from_leaf() {
    let key: ByteArray = "food";
    let value: ByteArray = "some longer text";
    let leaf = LeafOp {
        hash: HashOp::Sha256,
        prehash_key: HashOp::NoOp,
        prehash_value: HashOp::NoOp,
        length: LengthOp::VarProto,
        prefix: array![],
    };
    let proof = ExistenceProof { key: key.into(), value: value.into(), leaf, path: array![], };

    let root = proof.calculate_root();

    assert_eq!(
        encode_hex(root.into()), "b68f5d298e915ae1753dd333da1f9cf605411a5f2e12516be6758f365e6db265"
    );
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/verify.rs#L408
#[test]
fn test_calculate_root_from_leaf_and_inner() {
    let key: ByteArray = "food";
    let value: ByteArray = "some longer text";
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
    let proof = ExistenceProof { key: key.into(), value: value.into(), leaf, path: array![inner], };

    let root = proof.calculate_root();

    assert_eq!(
        encode_hex(root.into()), "836ea236a6902a665c2a004c920364f24cad52ded20b1e4f22c3179bfe25b2a9"
    );
}
