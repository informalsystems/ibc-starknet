use ics23::tests::data::{
    TestData, iavl_exist_left, iavl_exist_middle, iavl_exist_right, iavl_nonexist_left,
    iavl_nonexist_middle, iavl_nonexist_right, smt_exist_left, smt_exist_middle, smt_exist_right,
    smt_nonexist_left, smt_nonexist_middle, smt_nonexist_right, tendermint_exist_left,
    tendermint_exist_middle, tendermint_exist_right, tendermint_nonexist_left,
    tendermint_nonexist_middle, tendermint_nonexist_right,
};
use ics23::{
    ByteArrayIntoArrayU8, CommitmentProof, ExistenceProof, ExistenceProofImpl, HashOp, InnerOp,
    LeafOp, LengthOp, Proof, ProofSpec, SliceU32IntoArrayU8, byte_array_to_slice_u32, decode_hex,
    encode_hex, iavl_spec, smt_spec, tendermint_spec, verify_existence, verify_non_existence,
};
use protobuf::hex::decode as decode_hex_byte_array;
use protobuf::types::message::ProtoCodecImpl;

fn encoding_roundtrip_fixture(proof: @ByteArray) {
    let proof_bytes = decode_hex_byte_array(proof);
    let decoded = ProtoCodecImpl::decode::<CommitmentProof>(@proof_bytes).unwrap();
    let encoded = ProtoCodecImpl::encode(@decoded);
    assert_eq!(proof_bytes, encoded);
}

fn decode_and_verify(data: @TestData, spec: @ProofSpec) {
    let root = byte_array_to_slice_u32(decode_hex_byte_array(data.root));
    let key = decode_hex_byte_array(data.key).into();
    let value = decode_hex_byte_array(data.value).into();
    let p = ProtoCodecImpl::decode::<CommitmentProof>(@decode_hex_byte_array(data.proof)).unwrap();
    match p.proof {
        Proof::Exist(p) => { verify_existence(spec, @p, @root, @key, @value); },
        Proof::NonExist(p) => {
            assert(value.len() == 0, 'value must not exist');
            verify_non_existence(spec, @p, @root, key);
        },
    };
}

#[test]
fn test_protobuf_encoding_roundtrip() {
    encoding_roundtrip_fixture(@iavl_exist_left().proof);
    encoding_roundtrip_fixture(@iavl_exist_right().proof);
    encoding_roundtrip_fixture(@iavl_exist_middle().proof);
    encoding_roundtrip_fixture(@iavl_nonexist_left().proof);
    encoding_roundtrip_fixture(@iavl_nonexist_right().proof);
    encoding_roundtrip_fixture(@iavl_nonexist_middle().proof);
    encoding_roundtrip_fixture(@tendermint_exist_left().proof);
    encoding_roundtrip_fixture(@tendermint_exist_right().proof);
    encoding_roundtrip_fixture(@tendermint_exist_middle().proof);
    encoding_roundtrip_fixture(@tendermint_nonexist_left().proof);
    encoding_roundtrip_fixture(@tendermint_nonexist_right().proof);
    encoding_roundtrip_fixture(@tendermint_nonexist_middle().proof);
    encoding_roundtrip_fixture(@smt_exist_left().proof);
    encoding_roundtrip_fixture(@smt_exist_right().proof);
    encoding_roundtrip_fixture(@smt_exist_middle().proof);
    encoding_roundtrip_fixture(@smt_nonexist_left().proof);
    encoding_roundtrip_fixture(@smt_nonexist_right().proof);
    encoding_roundtrip_fixture(@smt_nonexist_middle().proof);
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L459
#[test]
fn test_vector_iavl_left() {
    decode_and_verify(@iavl_exist_left(), @iavl_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L480
#[test]
fn test_vector_iavl_left_non() {
    decode_and_verify(@iavl_nonexist_left(), @iavl_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L466
#[test]
fn test_vector_iavl_right() {
    decode_and_verify(@iavl_exist_right(), @iavl_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L487
#[test]
fn test_vector_iavl_right_non() {
    decode_and_verify(@iavl_nonexist_right(), @iavl_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L473
#[test]
fn test_vector_iavl_middle() {
    decode_and_verify(@iavl_exist_middle(), @iavl_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L494
#[test]
fn test_vector_iavl_middle_non() {
    decode_and_verify(@iavl_nonexist_middle(), @iavl_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L501
#[test]
fn test_vector_tendermint_left() {
    decode_and_verify(@tendermint_exist_left(), @tendermint_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L522
#[test]
fn test_vector_tendermint_left_non() {
    decode_and_verify(@tendermint_nonexist_left(), @tendermint_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L508
#[test]
fn test_vector_tendermint_right() {
    decode_and_verify(@tendermint_exist_right(), @tendermint_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L529
#[test]
fn test_vector_tendermint_right_non() {
    decode_and_verify(@tendermint_nonexist_right(), @tendermint_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L515
#[test]
fn test_vector_tendermint_middle() {
    decode_and_verify(@tendermint_exist_middle(), @tendermint_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L536
#[test]
fn test_vector_tendermint_middle_non() {
    decode_and_verify(@tendermint_nonexist_middle(), @tendermint_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#l543
#[test]
fn test_vector_smt_left() {
    decode_and_verify(@smt_exist_left(), @smt_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L564
#[test]
fn test_vector_smt_left_non() {
    decode_and_verify(@smt_nonexist_left(), @smt_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#l550
#[test]
fn test_vector_smt_right() {
    decode_and_verify(@smt_exist_right(), @smt_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L571
#[test]
fn test_vector_smt_right_non() {
    decode_and_verify(@smt_nonexist_right(), @smt_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#l557
#[test]
fn test_vector_smt_middle() {
    decode_and_verify(@smt_exist_middle(), @smt_spec());
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L578
#[test]
fn test_vector_smt_middle_non() {
    decode_and_verify(@smt_nonexist_middle(), @smt_spec());
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
    let proof = ExistenceProof { key: key.into(), value: value.into(), leaf, path: array![] };

    let root = proof.calculate_root();

    assert_eq!(
        encode_hex(root.into()), "b68f5d298e915ae1753dd333da1f9cf605411a5f2e12516be6758f365e6db265",
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
        prefix: array![],
    };
    let inner = InnerOp {
        hash: HashOp::Sha256, prefix: decode_hex(@"deadbeef00cafe00"), suffix: array![],
    };
    let proof = ExistenceProof { key: key.into(), value: value.into(), leaf, path: array![inner] };

    let root = proof.calculate_root();

    assert_eq!(
        encode_hex(root.into()), "836ea236a6902a665c2a004c920364f24cad52ded20b1e4f22c3179bfe25b2a9",
    );
}
