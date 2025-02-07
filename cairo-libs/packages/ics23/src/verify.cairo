use ics23::{
    Proof, ProofSpec, ProofSpecTrait, RootBytes, KeyBytes, ValueBytes, ICS23Errors,
    ExistenceProofImpl, NonExistenceProof, NonExistenceProofImpl, SliceU32IntoArrayU8,
    ExistenceProof, LeafOp, HashOp, InnerOp,
};
use protobuf::varint::decode_varint_from_u8_array;

pub fn verify_membership(
    specs: Array<ProofSpec>,
    proofs: @Array<Proof>,
    root: RootBytes,
    keys: Array<KeyBytes>,
    value: ValueBytes,
) {
    let proofs_len = proofs.len();
    assert(proofs_len > 0, ICS23Errors::MISSING_MERKLE_PROOF);
    assert(root != [0; 8], ICS23Errors::ZERO_MERKLE_ROOT);
    assert(value.len() > 0, ICS23Errors::MISSING_VALUE);
    assert(proofs_len == specs.len(), ICS23Errors::MISMATCHED_NUM_OF_PROOFS);
    assert(proofs_len == keys.len(), ICS23Errors::MISMATCHED_NUM_OF_PROOFS);
    let mut subroot = [0; 8];
    let mut subvalue = value;
    let mut i = 0;
    while i < proofs_len {
        if let Proof::Exist(p) = proofs[i] {
            subroot = p.calculate_root();
            verify_existence(specs[i], p, @subroot, keys[proofs_len - 1 - i], @subvalue);
        } else {
            panic!("{}", ICS23Errors::INVALID_PROOF_TYPE);
        }
        subvalue = subroot.into();
        i += 1;
    };
    assert(root == subroot, ICS23Errors::INVALID_MERKLE_PROOF);
}

pub fn verify_non_membership(
    specs: Array<ProofSpec>, proofs: @Array<Proof>, root: RootBytes, keys: Array<KeyBytes>
) {
    let proofs_len = proofs.len();
    assert(proofs_len > 0, ICS23Errors::MISSING_MERKLE_PROOF);
    assert(root == [0; 8], ICS23Errors::ZERO_MERKLE_ROOT);
    assert(proofs_len == specs.len(), ICS23Errors::MISMATCHED_NUM_OF_PROOFS);
    assert(proofs_len == keys.len(), ICS23Errors::MISMATCHED_NUM_OF_PROOFS);
    let mut subroot = [0; 8];
    let mut i = 0;
    while i < proofs_len {
        if let Proof::NonExist(p) = proofs[i] {
            subroot = p.calculate_root();
            verify_non_existence(specs[i], p, @subroot, keys[proofs_len - 1 - i]);

            verify_membership(
                specs.clone(), proofs, root, keys.clone(), subroot.into()
            ) // TODO: add start_index
        } else {
            panic!("{}", ICS23Errors::INVALID_PROOF_TYPE);
        }
        i += 1;
    };
}

pub fn verify_existence(
    spec: @ProofSpec, proof: @ExistenceProof, root: @RootBytes, key: @KeyBytes, value: @ValueBytes
) {
    check_existence_spec(spec, proof);
    assert(proof.key == key, ICS23Errors::MISMATCHED_KEY);
    assert(proof.value == value, ICS23Errors::MISMATCHED_VALUE);
    let calc = proof.calculate_root_for_spec(Option::Some(spec));
    assert(@calc == root, ICS23Errors::MISMATCHED_ROOT)
}

pub fn verify_non_existence(
    spec: @ProofSpec, proof: @NonExistenceProof, root: @RootBytes, key: @KeyBytes
) {}

fn check_existence_spec(spec: @ProofSpec, proof: @ExistenceProof) {
    if spec.is_iavl() {
        ensure_leaf_prefix(proof.leaf.prefix.clone());
    }
    ensure_leaf(proof.leaf, spec.leaf_spec);

    let inner_len = proof.path.len();
    if spec.min_depth != @0 {
        assert(@inner_len >= spec.min_depth, ICS23Errors::INVALID_INNER_OP_SIZE);
        assert(@inner_len <= spec.max_depth, ICS23Errors::INVALID_INNER_OP_SIZE);
    }

    for i in 0
        ..inner_len {
            let inner = proof.path.at(i);
            if spec.is_iavl() {
                ensure_inner_prefix(inner.prefix.clone(), i.try_into().unwrap(), inner.hash);
            }
            ensure_inner(inner, spec.clone());
        }
}

fn ensure_leaf_prefix(prefix: Array<u8>) {
    let rem = ensure_iavl_prefix(prefix, 0);
    assert(rem == 0, ICS23Errors::INVALID_LEAF_PREFIX);
}

fn ensure_iavl_prefix(prefix: Array<u8>, min_height: u64) -> u32 {
    let mut prefix_bytes = prefix;
    let (height, _) = decode_varint_from_u8_array(ref prefix_bytes);
    assert(height > min_height, ICS23Errors::INVALID_IAVL_HEIGHT_PREFIX);
    let (size, _) = decode_varint_from_u8_array(ref prefix_bytes);
    assert(size > 0, ICS23Errors::ZERO_IAVL_SIZE_PREFIX);
    let (version, _) = decode_varint_from_u8_array(ref prefix_bytes);
    assert(version > 0, ICS23Errors::ZERO_IAVL_VERSION_PREFIX);
    prefix_bytes.len()
}

fn ensure_leaf(leaf: @LeafOp, leaf_spec: @LeafOp) {
    assert(leaf.hash == leaf_spec.hash, ICS23Errors::INVALID_HASH_OP);
    assert(leaf.prehash_key == leaf_spec.prehash_key, ICS23Errors::INVALID_PREHASH_KEY);
    assert(leaf.prehash_value == leaf_spec.prehash_value, ICS23Errors::INVALID_PREHASH_VALUE);
    assert(leaf.length == leaf_spec.length, ICS23Errors::INVALID_LENGTH_OP);
    assert(leaf.prefix == leaf_spec.prefix, ICS23Errors::INVALID_LEAF_PREFIX);
}

fn ensure_inner_prefix(prefix: Array<u8>, min_height: u64, hash_op: @HashOp) {
    let rem = ensure_iavl_prefix(prefix, min_height);
    assert(rem == 0 || rem == 1 || rem == 34, ICS23Errors::INVALID_INNER_PREFIX);
    assert(hash_op == @HashOp::Sha256, ICS23Errors::INVALID_HASH_OP);
}

fn ensure_inner(inner: @InnerOp, spec: ProofSpec) {
    let inner_spec = spec.inner_spec;
    assert(inner.hash == @inner_spec.hash, ICS23Errors::INVALID_HASH_OP);
    assert(inner.prefix == @spec.leaf_spec.prefix, ICS23Errors::INVALID_INNER_PREFIX);
    assert(
        inner.prefix.len() >= inner_spec.min_prefix_length, ICS23Errors::INVALID_INNER_PREFIX_LEN
    );
    assert(
        inner.prefix.len() <= inner_spec.min_prefix_length, ICS23Errors::INVALID_INNER_PREFIX_LEN
    );
    assert(inner_spec.child_size > 0, ICS23Errors::ZERO_CHILD_SIZE);
    assert(
        inner_spec.min_prefix_length + inner_spec.child_size > inner_spec.max_prefix_length,
        ICS23Errors::INVALID_INNER_PREFIX_LEN
    );
    assert(inner.suffix.len() % inner_spec.child_size == 0, ICS23Errors::INVALID_INNER_SUFFIX);
}
