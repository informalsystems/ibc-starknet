use ics23::{LeafOp, LengthOp, HashOp, apply_leaf, proto_len};
use alexandria_math::pow;

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
    let value = array![0x62, 0x61, 0x72]; // bar
    let hash = apply_leaf(@leaf, @key, value);

    // https://github.com/cosmos/ics23/blob/c7c728879896fb260fe76b208ea6a17c2b0132a3/rust/src/ops.rs#L210
    let expected = [
        3282800625, 924903597, 2420628793, 1181432969, 1961202370, 4197989706, 962621772, 2935014642
    ];

    assert_eq!(hash, expected);
}

fn check_proto_len(value: u32, expected: Array<u8>) {
    assert_eq!(proto_len(value), expected);
}

#[test]
fn test_proto_len() {
    check_proto_len(pow(2, 0) - 1, array![0x00]);
    check_proto_len(pow(2, 0), array![0x01]); // 1
    check_proto_len(pow(2, 7) - 1, array![0x7F]); // 127
    check_proto_len(pow(2, 7), array![0x80, 0x01]); // 128
    check_proto_len(pow(2, 14) - 1, array![0xFF, 0x7F]); // [255, 127]
    check_proto_len(pow(2, 14), array![0x80, 0x80, 0x01]); // [128, 128, 1]
    check_proto_len(pow(2, 21) - 1, array![0xFF, 0xFF, 0x7F]); // [255, 255, 127]
    check_proto_len(pow(2, 21), array![0x80, 0x80, 0x80, 0x01]); // [128, 128, 128, 1]
    check_proto_len(pow(2, 28) - 1, array![0xFF, 0xFF, 0xFF, 0x7F]); // [255, 255, 255, 127]
    check_proto_len(pow(2, 28), array![0x80, 0x80, 0x80, 0x80, 0x01]); // [128, 128, 128, 128, 1]
    check_proto_len(0xffffffff, array![0xFF, 0xFF, 0xFF, 0xFF, 0x0F]); // [255, 255, 255, 255, 15]
}
