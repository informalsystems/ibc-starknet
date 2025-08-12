use ibc_utils::bytes::{ByteArrayIntoArrayU8, SpanU32IntoArrayU8, SpanU8IntoByteArray};
use ibc_utils::hex::{decode_byte_array as decode_hex, encode_lower as encode_hex};
use ics23::{
    CommitmentProof, ExistenceProof, ExistenceProofImpl, HashOp, InnerOp, InnerSpec, LeafOp,
    LengthOp, Proof, ProofSpec, iavl_spec, smt_spec, tendermint_spec, verify_existence,
    verify_membership, verify_non_existence,
};
use protobuf::types::message::ProtoCodecImpl;
use crate::tests::data::{
    TestData, iavl_exist_left, iavl_exist_middle, iavl_exist_right, iavl_nonexist_left,
    iavl_nonexist_middle, iavl_nonexist_right, smt_exist_left, smt_exist_middle, smt_exist_right,
    smt_nonexist_left, smt_nonexist_middle, smt_nonexist_right, tendermint_exist_left,
    tendermint_exist_middle, tendermint_exist_right, tendermint_nonexist_left,
    tendermint_nonexist_middle, tendermint_nonexist_right,
};

fn encoding_roundtrip_fixture(proof: @ByteArray) {
    let proof_bytes = decode_hex(proof.clone());
    let decoded = ProtoCodecImpl::decode::<CommitmentProof>(proof_bytes.span()).unwrap();
    let encoded = ProtoCodecImpl::encode(@decoded);
    assert_eq!(proof_bytes, encoded);
}

fn decode_and_verify(data: @TestData, spec: @ProofSpec) {
    let key = decode_hex(data.key.clone());
    let value = decode_hex(data.value.clone());
    let proof = decode_hex(data.proof.clone());
    let p = ProtoCodecImpl::decode::<CommitmentProof>(proof.span()).unwrap();
    match p.proof {
        Proof::Exist(p) => { verify_existence(spec, @p, @key, @value); },
        Proof::NonExist(p) => {
            assert(value.is_empty(), 'value must not exist');
            verify_non_existence(spec, @p, key);
        },
    };
}
#[cairofmt::skip]
fn get_verification_params() -> (
    Array<ProofSpec>, Array<Proof>, [u32; 8], Array<Array<u8>>, Array<u8>, u32,
) {
    let specs = array![
        ProofSpec {
            leaf_spec: LeafOp {
                hash: HashOp::Sha256(()),
                prehash_key: HashOp::NoOp(()),
                prehash_value: HashOp::Sha256(()),
                length: LengthOp::VarProto(()),
                prefix: array![0],
            },
            inner_spec: InnerSpec {
                child_order: array![0, 1],
                child_size: 33,
                min_prefix_length: 4,
                max_prefix_length: 12,
                empty_child: array![],
                hash: HashOp::Sha256(()),
            },
            max_depth: 0,
            min_depth: 0,
            prehash_key_before_comparison: false,
        },
        ProofSpec {
            leaf_spec: LeafOp {
                hash: HashOp::Sha256(()),
                prehash_key: HashOp::NoOp(()),
                prehash_value: HashOp::Sha256(()),
                length: LengthOp::VarProto(()),
                prefix: array![0],
            },
            inner_spec: InnerSpec {
                child_order: array![0, 1],
                child_size: 32,
                min_prefix_length: 1,
                max_prefix_length: 1,
                empty_child: array![],
                hash: HashOp::Sha256(()),
            },
            max_depth: 0,
            min_depth: 0,
            prehash_key_before_comparison: false,
        },
    ];
    let proofs = array![
        Proof::Exist(
            ExistenceProof {
                key: array![
                    99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 115, 47, 99, 111, 110, 110, 101,
                    99, 116, 105, 111, 110, 45, 48,
                ],
                value: array![
                    10, 9, 48, 56, 45, 119, 97, 115, 109, 45, 48, 18, 35, 10, 1, 49, 18, 13, 79, 82,
                    68, 69, 82, 95, 79, 82, 68, 69, 82, 69, 68, 18, 15, 79, 82, 68, 69, 82, 95, 85,
                    78, 79, 82, 68, 69, 82, 69, 68, 24, 2, 34, 38, 10, 15, 48, 55, 45, 116, 101,
                    110, 100, 101, 114, 109, 105, 110, 116, 45, 48, 18, 12, 99, 111, 110, 110, 101,
                    99, 116, 105, 111, 110, 45, 48, 26, 5, 10, 3, 105, 98, 99,
                ],
                leaf: LeafOp {
                    hash: HashOp::Sha256(()),
                    prehash_key: HashOp::NoOp(()),
                    prehash_value: HashOp::Sha256(()),
                    length: LengthOp::VarProto(()),
                    prefix: array![0, 2, 46],
                },
                path: array![
                    InnerOp {
                        hash: HashOp::Sha256(()),
                        prefix: array![
                            2, 4, 46, 32, 103, 183, 108, 123, 130, 214, 14, 190, 231, 244, 29, 209,
                            26, 2, 83, 76, 26, 22, 239, 167, 12, 33, 115, 16, 53, 98, 48, 223, 213,
                            173, 12, 32, 32,
                        ],
                        suffix: array![],
                    },
                    InnerOp {
                        hash: HashOp::Sha256(()),
                        prefix: array![
                            4, 6, 46, 32, 155, 127, 121, 48, 181, 210, 25, 89, 222, 215, 112, 146,
                            120, 123, 181, 30, 146, 159, 147, 70, 86, 149, 69, 29, 33, 171, 67, 123,
                            70, 137, 11, 173, 32,
                        ],
                        suffix: array![],
                    },
                    InnerOp {
                        hash: HashOp::Sha256(()),
                        prefix: array![6, 14, 46, 32],
                        suffix: array![
                            32, 124, 169, 233, 205, 245, 0, 82, 237, 24, 14, 214, 193, 249, 20, 143,
                            122, 107, 210, 130, 74, 150, 247, 170, 183, 63, 155, 117, 246, 214, 193,
                            148, 151,
                        ],
                    },
                    InnerOp {
                        hash: HashOp::Sha256(()),
                        prefix: array![
                            10, 38, 46, 32, 23, 238, 211, 143, 159, 173, 21, 243, 33, 61, 211, 217,
                            135, 232, 86, 99, 134, 121, 220, 133, 131, 238, 81, 170, 53, 159, 111,
                            231, 2, 91, 173, 47, 32,
                        ],
                        suffix: array![],
                    },
                ],
            },
        ),
        Proof::Exist(
            ExistenceProof {
                key: array![105, 98, 99],
                value: array![
                    169, 198, 253, 254, 219, 184, 1, 9, 160, 106, 211, 180, 41, 135, 1, 25, 160,
                    130, 234, 77, 75, 205, 210, 181, 177, 76, 244, 57, 149, 186, 236, 217,
                ],
                leaf: LeafOp {
                    hash: HashOp::Sha256(()),
                    prehash_key: HashOp::NoOp(()),
                    prehash_value: HashOp::Sha256(()),
                    length: LengthOp::VarProto(()),
                    prefix: array![0],
                },
                path: array![
                    InnerOp {
                        hash: HashOp::Sha256(()),
                        prefix: array![
                            1, 78, 29, 92, 86, 59, 13, 176, 255, 220, 186, 111, 201, 125, 192, 199,
                            177, 59, 91, 92, 156, 52, 19, 87, 240, 136, 39, 86, 80, 71, 214, 198,
                            185,
                        ],
                        suffix: array![],
                    },
                    InnerOp {
                        hash: HashOp::Sha256(()),
                        prefix: array![1],
                        suffix: array![
                            16, 43, 226, 80, 158, 2, 200, 133, 199, 73, 61, 110, 15, 162, 121, 43,
                            8, 222, 107, 204, 113, 87, 112, 127, 57, 243, 145, 99, 246, 230, 142,
                            253,
                        ],
                    },
                    InnerOp {
                        hash: HashOp::Sha256(()),
                        prefix: array![1],
                        suffix: array![
                            130, 76, 240, 0, 5, 140, 189, 39, 186, 172, 69, 106, 251, 34, 190, 12,
                            16, 24, 114, 62, 217, 153, 0, 119, 84, 197, 71, 45, 252, 187, 4, 23,
                        ],
                    },
                    InnerOp {
                        hash: HashOp::Sha256(()),
                        prefix: array![1],
                        suffix: array![
                            147, 244, 66, 247, 144, 61, 183, 57, 85, 166, 4, 88, 91, 228, 197, 2,
                            231, 156, 184, 227, 124, 191, 42, 127, 158, 80, 133, 223, 14, 211, 139,
                            72,
                        ],
                    },
                    InnerOp {
                        hash: HashOp::Sha256(()),
                        prefix: array![
                            1, 146, 81, 36, 89, 158, 30, 171, 81, 161, 148, 14, 251, 50, 118, 234,
                            138, 243, 17, 239, 59, 249, 53, 115, 195, 183, 222, 46, 54, 28, 20, 246,
                            26,
                        ],
                        suffix: array![],
                    },
                    InnerOp {
                        hash: HashOp::Sha256(()),
                        prefix: array![1],
                        suffix: array![
                            180, 165, 124, 40, 174, 123, 121, 90, 64, 198, 11, 147, 199, 151, 209,
                            216, 178, 47, 109, 111, 10, 240, 101, 212, 194, 186, 183, 100, 23, 62,
                            42, 201,
                        ],
                    },
                ],
            },
        ),
    ];
    let root: [u32; 8] = [
        1873810883, 3319552262, 2244397051, 3941308800, 3095490340, 3765148856, 684595155,
        1885881727,
    ];
    let keys = array![
        array![105, 98, 99],
        array![
            99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 115, 47, 99, 111, 110, 110, 101, 99,
            116, 105, 111, 110, 45, 48,
        ],
    ];
    let value: Array<u8> = array![
        10, 9, 48, 56, 45, 119, 97, 115, 109, 45, 48, 18, 35, 10, 1, 49, 18, 13, 79, 82, 68, 69, 82,
        95, 79, 82, 68, 69, 82, 69, 68, 18, 15, 79, 82, 68, 69, 82, 95, 85, 78, 79, 82, 68, 69, 82,
        69, 68, 24, 2, 34, 38, 10, 15, 48, 55, 45, 116, 101, 110, 100, 101, 114, 109, 105, 110, 116,
        45, 48, 18, 12, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 45, 48, 26, 5, 10, 3, 105,
        98, 99,
    ];
    let index = 0;
    (specs, proofs, root, keys, value, index)
}
#[cairofmt::skip]
#[test]
fn test_channel_ack_verification_verify_existence_1() {
    let spec = ProofSpec {
        leaf_spec: LeafOp {
            hash: HashOp::Sha256(()),
            prehash_key: HashOp::NoOp(()),
            prehash_value: HashOp::Sha256(()),
            length: LengthOp::VarProto(()),
            prefix: array![0],
        },
        inner_spec: InnerSpec {
            child_order: array![0, 1],
            child_size: 33,
            min_prefix_length: 4,
            max_prefix_length: 12,
            empty_child: array![],
            hash: HashOp::Sha256(()),
        },
        max_depth: 0,
        min_depth: 0,
        prehash_key_before_comparison: false,
    };
    let proof = ExistenceProof {
        key: array![
            99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 115, 47, 99, 111, 110, 110, 101, 99,
            116, 105, 111, 110, 45, 48,
        ],
        value: array![
            10, 9, 48, 56, 45, 119, 97, 115, 109, 45, 48, 18, 35, 10, 1, 49, 18, 13, 79, 82, 68, 69,
            82, 95, 79, 82, 68, 69, 82, 69, 68, 18, 15, 79, 82, 68, 69, 82, 95, 85, 78, 79, 82, 68,
            69, 82, 69, 68, 24, 2, 34, 38, 10, 15, 48, 55, 45, 116, 101, 110, 100, 101, 114, 109,
            105, 110, 116, 45, 48, 18, 12, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 45, 48,
            26, 5, 10, 3, 105, 98, 99,
        ],
        leaf: LeafOp {
            hash: HashOp::Sha256(()),
            prehash_key: HashOp::NoOp(()),
            prehash_value: HashOp::Sha256(()),
            length: LengthOp::VarProto(()),
            prefix: array![0, 2, 46],
        },
        path: array![
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![
                    2, 4, 46, 32, 103, 183, 108, 123, 130, 214, 14, 190, 231, 244, 29, 209, 26, 2,
                    83, 76, 26, 22, 239, 167, 12, 33, 115, 16, 53, 98, 48, 223, 213, 173, 12, 32,
                    32,
                ],
                suffix: array![],
            },
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![
                    4, 6, 46, 32, 155, 127, 121, 48, 181, 210, 25, 89, 222, 215, 112, 146, 120, 123,
                    181, 30, 146, 159, 147, 70, 86, 149, 69, 29, 33, 171, 67, 123, 70, 137, 11, 173,
                    32,
                ],
                suffix: array![],
            },
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![6, 14, 46, 32],
                suffix: array![
                    32, 124, 169, 233, 205, 245, 0, 82, 237, 24, 14, 214, 193, 249, 20, 143, 122,
                    107, 210, 130, 74, 150, 247, 170, 183, 63, 155, 117, 246, 214, 193, 148, 151,
                ],
            },
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![
                    10, 38, 46, 32, 23, 238, 211, 143, 159, 173, 21, 243, 33, 61, 211, 217, 135,
                    232, 86, 99, 134, 121, 220, 133, 131, 238, 81, 170, 53, 159, 111, 231, 2, 91,
                    173, 47, 32,
                ],
                suffix: array![],
            },
        ],
    };
    let key = array![
        99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 115, 47, 99, 111, 110, 110, 101, 99, 116,
        105, 111, 110, 45, 48,
    ];
    let subvalue = array![
        10, 9, 48, 56, 45, 119, 97, 115, 109, 45, 48, 18, 35, 10, 1, 49, 18, 13, 79, 82, 68, 69, 82,
        95, 79, 82, 68, 69, 82, 69, 68, 18, 15, 79, 82, 68, 69, 82, 95, 85, 78, 79, 82, 68, 69, 82,
        69, 68, 24, 2, 34, 38, 10, 15, 48, 55, 45, 116, 101, 110, 100, 101, 114, 109, 105, 110, 116,
        45, 48, 18, 12, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 45, 48, 26, 5, 10, 3, 105,
        98, 99,
    ];

    verify_existence(@spec, @proof, @key, @subvalue);
}
#[cairofmt::skip]
#[test]
fn test_channel_ack_verification_verify_existence_2() {
    let spec = ProofSpec {
        leaf_spec: LeafOp {
            hash: HashOp::Sha256(()),
            prehash_key: HashOp::NoOp(()),
            prehash_value: HashOp::Sha256(()),
            length: LengthOp::VarProto(()),
            prefix: array![0],
        },
        inner_spec: InnerSpec {
            child_order: array![0, 1],
            child_size: 32,
            min_prefix_length: 1,
            max_prefix_length: 1,
            empty_child: array![],
            hash: HashOp::Sha256(()),
        },
        max_depth: 0,
        min_depth: 0,
        prehash_key_before_comparison: false,
    };
    let proof = ExistenceProof {
        key: array![105, 98, 99],
        value: array![
            169, 198, 253, 254, 219, 184, 1, 9, 160, 106, 211, 180, 41, 135, 1, 25, 160, 130, 234,
            77, 75, 205, 210, 181, 177, 76, 244, 57, 149, 186, 236, 217,
        ],
        leaf: LeafOp {
            hash: HashOp::Sha256(()),
            prehash_key: HashOp::NoOp(()),
            prehash_value: HashOp::Sha256(()),
            length: LengthOp::VarProto(()),
            prefix: array![0],
        },
        path: array![
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![
                    1, 78, 29, 92, 86, 59, 13, 176, 255, 220, 186, 111, 201, 125, 192, 199, 177, 59,
                    91, 92, 156, 52, 19, 87, 240, 136, 39, 86, 80, 71, 214, 198, 185,
                ],
                suffix: array![],
            },
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![1],
                suffix: array![
                    16, 43, 226, 80, 158, 2, 200, 133, 199, 73, 61, 110, 15, 162, 121, 43, 8, 222,
                    107, 204, 113, 87, 112, 127, 57, 243, 145, 99, 246, 230, 142, 253,
                ],
            },
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![1],
                suffix: array![
                    130, 76, 240, 0, 5, 140, 189, 39, 186, 172, 69, 106, 251, 34, 190, 12, 16, 24,
                    114, 62, 217, 153, 0, 119, 84, 197, 71, 45, 252, 187, 4, 23,
                ],
            },
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![1],
                suffix: array![
                    147, 244, 66, 247, 144, 61, 183, 57, 85, 166, 4, 88, 91, 228, 197, 2, 231, 156,
                    184, 227, 124, 191, 42, 127, 158, 80, 133, 223, 14, 211, 139, 72,
                ],
            },
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![
                    1, 146, 81, 36, 89, 158, 30, 171, 81, 161, 148, 14, 251, 50, 118, 234, 138, 243,
                    17, 239, 59, 249, 53, 115, 195, 183, 222, 46, 54, 28, 20, 246, 26,
                ],
                suffix: array![],
            },
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![1],
                suffix: array![
                    180, 165, 124, 40, 174, 123, 121, 90, 64, 198, 11, 147, 199, 151, 209, 216, 178,
                    47, 109, 111, 10, 240, 101, 212, 194, 186, 183, 100, 23, 62, 42, 201,
                ],
            },
        ],
    };
    let key = array![105, 98, 99];
    let subvalue = array![
        169, 198, 253, 254, 219, 184, 1, 9, 160, 106, 211, 180, 41, 135, 1, 25, 160, 130, 234, 77,
        75, 205, 210, 181, 177, 76, 244, 57, 149, 186, 236, 217,
    ];

    verify_existence(@spec, @proof, @key, @subvalue);
}
#[cairofmt::skip]
#[test]
fn test_channel_ack_verification_calculate_root_for_spec_1() {
    let spec = ProofSpec {
        leaf_spec: LeafOp {
            hash: HashOp::Sha256(()),
            prehash_key: HashOp::NoOp(()),
            prehash_value: HashOp::Sha256(()),
            length: LengthOp::VarProto(()),
            prefix: array![0],
        },
        inner_spec: InnerSpec {
            child_order: array![0, 1],
            child_size: 33,
            min_prefix_length: 4,
            max_prefix_length: 12,
            empty_child: array![],
            hash: HashOp::Sha256(()),
        },
        max_depth: 0,
        min_depth: 0,
        prehash_key_before_comparison: false,
    };
    let proof = ExistenceProof {
        key: array![
            99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 115, 47, 99, 111, 110, 110, 101, 99,
            116, 105, 111, 110, 45, 48,
        ],
        value: array![
            10, 9, 48, 56, 45, 119, 97, 115, 109, 45, 48, 18, 35, 10, 1, 49, 18, 13, 79, 82, 68, 69,
            82, 95, 79, 82, 68, 69, 82, 69, 68, 18, 15, 79, 82, 68, 69, 82, 95, 85, 78, 79, 82, 68,
            69, 82, 69, 68, 24, 2, 34, 38, 10, 15, 48, 55, 45, 116, 101, 110, 100, 101, 114, 109,
            105, 110, 116, 45, 48, 18, 12, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 45, 48,
            26, 5, 10, 3, 105, 98, 99,
        ],
        leaf: LeafOp {
            hash: HashOp::Sha256(()),
            prehash_key: HashOp::NoOp(()),
            prehash_value: HashOp::Sha256(()),
            length: LengthOp::VarProto(()),
            prefix: array![0, 2, 46],
        },
        path: array![
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![
                    2, 4, 46, 32, 103, 183, 108, 123, 130, 214, 14, 190, 231, 244, 29, 209, 26, 2,
                    83, 76, 26, 22, 239, 167, 12, 33, 115, 16, 53, 98, 48, 223, 213, 173, 12, 32,
                    32,
                ],
                suffix: array![],
            },
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![
                    4, 6, 46, 32, 155, 127, 121, 48, 181, 210, 25, 89, 222, 215, 112, 146, 120, 123,
                    181, 30, 146, 159, 147, 70, 86, 149, 69, 29, 33, 171, 67, 123, 70, 137, 11, 173,
                    32,
                ],
                suffix: array![],
            },
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![6, 14, 46, 32],
                suffix: array![
                    32, 124, 169, 233, 205, 245, 0, 82, 237, 24, 14, 214, 193, 249, 20, 143, 122,
                    107, 210, 130, 74, 150, 247, 170, 183, 63, 155, 117, 246, 214, 193, 148, 151,
                ],
            },
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![
                    10, 38, 46, 32, 23, 238, 211, 143, 159, 173, 21, 243, 33, 61, 211, 217, 135,
                    232, 86, 99, 134, 121, 220, 133, 131, 238, 81, 170, 53, 159, 111, 231, 2, 91,
                    173, 47, 32,
                ],
                suffix: array![],
            },
        ],
    };

    proof.calculate_root_for_spec(Option::Some(@spec));
}
#[cairofmt::skip]
#[test]
fn test_channel_ack_verification_calculate_root_for_spec_2() {
    let spec = ProofSpec {
        leaf_spec: LeafOp {
            hash: HashOp::Sha256(()),
            prehash_key: HashOp::NoOp(()),
            prehash_value: HashOp::Sha256(()),
            length: LengthOp::VarProto(()),
            prefix: array![0],
        },
        inner_spec: InnerSpec {
            child_order: array![0, 1],
            child_size: 32,
            min_prefix_length: 1,
            max_prefix_length: 1,
            empty_child: array![],
            hash: HashOp::Sha256(()),
        },
        max_depth: 0,
        min_depth: 0,
        prehash_key_before_comparison: false,
    };
    let proof = ExistenceProof {
        key: array![105, 98, 99],
        value: array![
            169, 198, 253, 254, 219, 184, 1, 9, 160, 106, 211, 180, 41, 135, 1, 25, 160, 130, 234,
            77, 75, 205, 210, 181, 177, 76, 244, 57, 149, 186, 236, 217,
        ],
        leaf: LeafOp {
            hash: HashOp::Sha256(()),
            prehash_key: HashOp::NoOp(()),
            prehash_value: HashOp::Sha256(()),
            length: LengthOp::VarProto(()),
            prefix: array![0],
        },
        path: array![
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![
                    1, 78, 29, 92, 86, 59, 13, 176, 255, 220, 186, 111, 201, 125, 192, 199, 177, 59,
                    91, 92, 156, 52, 19, 87, 240, 136, 39, 86, 80, 71, 214, 198, 185,
                ],
                suffix: array![],
            },
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![1],
                suffix: array![
                    16, 43, 226, 80, 158, 2, 200, 133, 199, 73, 61, 110, 15, 162, 121, 43, 8, 222,
                    107, 204, 113, 87, 112, 127, 57, 243, 145, 99, 246, 230, 142, 253,
                ],
            },
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![1],
                suffix: array![
                    130, 76, 240, 0, 5, 140, 189, 39, 186, 172, 69, 106, 251, 34, 190, 12, 16, 24,
                    114, 62, 217, 153, 0, 119, 84, 197, 71, 45, 252, 187, 4, 23,
                ],
            },
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![1],
                suffix: array![
                    147, 244, 66, 247, 144, 61, 183, 57, 85, 166, 4, 88, 91, 228, 197, 2, 231, 156,
                    184, 227, 124, 191, 42, 127, 158, 80, 133, 223, 14, 211, 139, 72,
                ],
            },
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![
                    1, 146, 81, 36, 89, 158, 30, 171, 81, 161, 148, 14, 251, 50, 118, 234, 138, 243,
                    17, 239, 59, 249, 53, 115, 195, 183, 222, 46, 54, 28, 20, 246, 26,
                ],
                suffix: array![],
            },
            InnerOp {
                hash: HashOp::Sha256(()),
                prefix: array![1],
                suffix: array![
                    180, 165, 124, 40, 174, 123, 121, 90, 64, 198, 11, 147, 199, 151, 209, 216, 178,
                    47, 109, 111, 10, 240, 101, 212, 194, 186, 183, 100, 23, 62, 42, 201,
                ],
            },
        ],
    };

    proof.calculate_root_for_spec(Option::Some(@spec));
}

#[test]
fn test_channel_ack_verification_while_loop() {
    let (specs, proofs, _, keys, value, index) = get_verification_params();

    let proofs_len = proofs.len();
    let mut subroot = [0; 8];
    let mut subvalue = value;
    let mut i = index;
    while i != proofs_len {
        if let Proof::Exist(p) = proofs[i] {
            let spec = specs[i];
            subroot = p.calculate_root_for_spec(Option::Some(spec));
            verify_existence(spec, p, keys[proofs_len - 1 - i], @subvalue);
        }
        subvalue = subroot.span().into();
        i += 1;
    }
}

#[test]
fn test_channel_ack_verification() {
    let (specs, proofs, root, keys, value, index) = get_verification_params();

    verify_membership(specs, @proofs, root, keys, value, index);
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
    let root_u8: Array<u8> = root.span().into();

    assert_eq!(
        SpanU8IntoByteArray::into(encode_hex(root_u8.span()).span()),
        "b68f5d298e915ae1753dd333da1f9cf605411a5f2e12516be6758f365e6db265",
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
        hash: HashOp::Sha256, prefix: decode_hex("deadbeef00cafe00"), suffix: array![],
    };
    let proof = ExistenceProof { key: key.into(), value: value.into(), leaf, path: array![inner] };

    let root = proof.calculate_root();
    let root_u8: Array<u8> = root.span().into();

    assert_eq!(
        SpanU8IntoByteArray::into(encode_hex(root_u8.span()).span()),
        "836ea236a6902a665c2a004c920364f24cad52ded20b1e4f22c3179bfe25b2a9",
    );
}
