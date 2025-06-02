use ibc_utils::bytes::ByteArrayIntoArrayU8;
use ics23::{CommitmentProof, ExistenceProof, HashOp, InnerOp, LeafOp, LengthOp, MerkleProof, Proof};
use protobuf::types::message::ProtoCodecImpl;

#[test]
fn test_commitment_decode() {
    #[cairofmt::skip]
    let proof: Array<u8> = array![
        10, 180, 2, 10, 177, 2, 10, 24, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 115, 47, 99,
        111, 110, 110, 101, 99, 116, 105, 111, 110, 45, 48, 18, 90, 10, 9, 48, 56, 45, 119, 97, 115,
        109, 45, 48, 18, 35, 10, 1, 49, 18, 13, 79, 82, 68, 69, 82, 95, 79, 82, 68, 69, 82, 69, 68,
        18, 15, 79, 82, 68, 69, 82, 95, 85, 78, 79, 82, 68, 69, 82, 69, 68, 24, 2, 34, 38, 10, 15,
        48, 55, 45, 116, 101, 110, 100, 101, 114, 109, 105, 110, 116, 45, 48, 18, 12, 99, 111, 110,
        110, 101, 99, 116, 105, 111, 110, 45, 48, 26, 5, 10, 3, 105, 98, 99, 26, 11, 8, 1, 24, 1,
        32, 1, 42, 3, 0, 2, 46, 34, 41, 8, 1, 18, 37, 2, 4, 46, 32, 103, 183, 108, 123, 130, 214,
        14, 190, 231, 244, 29, 209, 26, 2, 83, 76, 26, 22, 239, 167, 12, 33, 115, 16, 53, 98, 48,
        223, 213, 173, 12, 32, 32, 34, 41, 8, 1, 18, 37, 4, 6, 46, 32, 179, 144, 156, 215, 135, 101,
        132, 17, 77, 33, 239, 210, 79, 41, 242, 166, 122, 3, 102, 214, 187, 142, 51, 200, 183, 208,
        154, 230, 22, 154, 45, 173, 32, 34, 43, 8, 1, 18, 4, 6, 14, 46, 32, 26, 33, 32, 124, 169,
        233, 205, 245, 0, 82, 237, 24, 14, 214, 193, 249, 20, 143, 122, 107, 210, 130, 74, 150, 247,
        170, 183, 63, 155, 117, 246, 214, 193, 148, 151, 34, 41, 8, 1, 18, 37, 10, 38, 46, 32, 77,
        200, 71, 85, 112, 227, 135, 85, 157, 72, 196, 94, 133, 233, 112, 46, 173, 125, 217, 12, 20,
        194, 210, 127, 229, 181, 41, 59, 248, 162, 149, 255, 32, 10, 167, 2, 10, 164, 2, 10, 3, 105,
        98, 99, 18, 32, 73, 154, 50, 53, 59, 201, 120, 247, 183, 55, 77, 94, 69, 172, 159, 110, 37,
        24, 45, 165, 173, 58, 169, 102, 53, 16, 9, 165, 150, 234, 112, 231, 26, 9, 8, 1, 24, 1, 32,
        1, 42, 1, 0, 34, 37, 8, 1, 18, 33, 1, 78, 29, 92, 86, 59, 13, 176, 255, 220, 186, 111, 201,
        125, 192, 199, 177, 59, 91, 92, 156, 52, 19, 87, 240, 136, 39, 86, 80, 71, 214, 198, 185,
        34, 39, 8, 1, 18, 1, 1, 26, 32, 16, 43, 226, 80, 158, 2, 200, 133, 199, 73, 61, 110, 15,
        162, 121, 43, 8, 222, 107, 204, 113, 87, 112, 127, 57, 243, 145, 99, 246, 230, 142, 253, 34,
        39, 8, 1, 18, 1, 1, 26, 32, 130, 76, 240, 0, 5, 140, 189, 39, 186, 172, 69, 106, 251, 34,
        190, 12, 16, 24, 114, 62, 217, 153, 0, 119, 84, 197, 71, 45, 252, 187, 4, 23, 34, 39, 8, 1,
        18, 1, 1, 26, 32, 136, 195, 181, 211, 211, 195, 54, 100, 246, 174, 106, 167, 241, 200, 144,
        174, 124, 63, 250, 136, 8, 20, 41, 174, 206, 7, 28, 150, 147, 79, 218, 77, 34, 37, 8, 1, 18,
        33, 1, 96, 236, 233, 236, 180, 20, 4, 69, 161, 67, 104, 164, 38, 151, 56, 209, 21, 238, 175,
        70, 36, 116, 250, 0, 99, 212, 26, 168, 127, 118, 170, 5, 34, 39, 8, 1, 18, 1, 1, 26, 32,
        180, 165, 124, 40, 174, 123, 121, 90, 64, 198, 11, 147, 199, 151, 209, 216, 178, 47, 109,
        111, 10, 240, 101, 212, 194, 186, 183, 100, 23, 62, 42, 201,
    ];
    let maybe_decoded_proof: Option<MerkleProof> = ProtoCodecImpl::decode::<
        MerkleProof,
    >(proof.span());
    assert(maybe_decoded_proof.is_some(), 'expected proof to be non zero');
    let decoded_proof: MerkleProof = maybe_decoded_proof.unwrap();
    for commitment_proof in decoded_proof.proofs {
        if let Proof::Exist(p) = @commitment_proof.proof {
            assert(p.value.len() > 0, 'decoded proof has empty value');
        }
    }
}

#[test]
fn test_encode_and_decode_commitment_proof() {
    // Create dummy CommitmentProof
    let key: ByteArray = "key";
    let value: ByteArray = "some longer text for value";
    let leaf = LeafOp {
        hash: HashOp::Sha256,
        prehash_key: HashOp::NoOp,
        prehash_value: HashOp::NoOp,
        length: LengthOp::VarProto,
        prefix: array![1, 6, 8, 4, 2],
    };
    let inner_op1 = InnerOp {
        hash: HashOp::Sha256, prefix: array![9, 19, 23, 5, 7], suffix: array![11, 21, 5, 3, 2],
    };
    let inner_op2 = InnerOp {
        hash: HashOp::Sha256,
        prefix: array![5, 3, 2, 9, 19],
        suffix: array![11, 21, 5, 3, 2, 9, 19, 23, 5, 7],
    };
    let proof = ExistenceProof {
        key: key.into(), value: value.into(), leaf, path: array![inner_op1, inner_op2],
    };
    let commitment = CommitmentProof { proof: Proof::Exist(proof) };
    // Encode CommitmentProof to ByteArray
    let byte_array_proof = ProtoCodecImpl::encode(@commitment);

    // Decode CommitmentProof from ByteArray
    let maybe_decoded_proof: Option<CommitmentProof> = ProtoCodecImpl::decode::<
        CommitmentProof,
    >(byte_array_proof.span());

    assert(maybe_decoded_proof.is_some(), 'expected proof to be non zero');
    let decoded_proof: CommitmentProof = maybe_decoded_proof.unwrap();
    if let Proof::Exist(p) = @decoded_proof.proof {
        assert(p.value.len() > 0, 'decoded proof has empty value');
    }
    assert(decoded_proof == commitment, 'decoded not equal original');
}
