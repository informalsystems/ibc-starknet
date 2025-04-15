use core::num::traits::{CheckedSub, Zero};
use ics23::{
    ArrayU8PartialOrd, ExistenceProof, ExistenceProofImpl, HashOp, ICS23Errors, InnerOp, InnerSpec,
    KeyBytes, LeafOp, NonExistenceProof, NonExistenceProofImpl, Proof, ProofSpec, ProofSpecTrait,
    RootBytes, SliceU32IntoArrayU8, ValueBytes,
};
use protobuf::varint::decode_varint_from_u8_array;

pub fn verify_membership(
    specs: Array<ProofSpec>,
    proofs: @Array<Proof>,
    root: RootBytes,
    keys: Array<KeyBytes>,
    value: ValueBytes,
    index: u32,
) {
    let proofs_len = proofs.len();
    assert(proofs_len > 0, ICS23Errors::MISSING_MERKLE_PROOF);
    assert(root != [0; 8], ICS23Errors::ZERO_MERKLE_ROOT);
    assert(value.len() > 0, ICS23Errors::MISSING_VALUE);
    assert(proofs_len == specs.len(), ICS23Errors::MISMATCHED_NUM_OF_PROOFS);
    assert(proofs_len == keys.len(), ICS23Errors::MISMATCHED_NUM_OF_PROOFS);
    let mut subroot = [0; 8];
    let mut subvalue = value;
    let mut i = index;
    while i != proofs_len {
        if let Proof::Exist(p) = proofs[i] {
            let spec = specs[i];
            subroot = p.calculate_root_for_spec(Option::Some(spec));
            verify_existence(spec, p, keys[proofs_len - 1 - i], @subvalue);
        } else {
            panic!("{}", ICS23Errors::INVALID_PROOF_TYPE);
        }
        subvalue = subroot.into();
        i += 1;
    }
    assert(root == subroot, ICS23Errors::INVALID_MERKLE_PROOF);
}

pub fn verify_non_membership(
    specs: Array<ProofSpec>, proofs: @Array<Proof>, root: RootBytes, keys: Array<KeyBytes>,
) {
    let proofs_len = proofs.len();
    assert(proofs_len > 0, ICS23Errors::MISSING_MERKLE_PROOF);
    assert(root != [0; 8], ICS23Errors::ZERO_MERKLE_ROOT);
    assert(proofs_len == specs.len(), ICS23Errors::MISMATCHED_NUM_OF_PROOFS);
    assert(proofs_len == keys.len(), ICS23Errors::MISMATCHED_NUM_OF_PROOFS);
    let mut subroot = [0; 8];

    // verify the absence of key in lowest subtree
    let proof = proofs[0];
    let spec = specs[0];
    let key = keys[proofs_len - 1];
    if let Proof::NonExist(p) = proof {
        subroot = p.calculate_root_for_spec(Option::Some(spec));
        verify_non_existence(spec, p, key.clone());
        verify_membership(specs.clone(), proofs, root, keys.clone(), subroot.into(), 1)
    } else {
        panic!("{}", ICS23Errors::INVALID_PROOF_TYPE);
    }
}

pub fn verify_existence(
    spec: @ProofSpec, proof: @ExistenceProof, key: @KeyBytes, value: @ValueBytes,
) {
    check_existence_spec(spec, proof);
    assert(proof.key == key, ICS23Errors::MISMATCHED_KEY);
    assert(proof.value == value, ICS23Errors::MISMATCHED_VALUE);
}

pub fn verify_non_existence(spec: @ProofSpec, proof: @NonExistenceProof, key: KeyBytes) {
    if let Option::Some(left) = proof.left {
        verify_existence(spec, left, left.key, left.value);
        assert(
            spec.key_for_comparison(key.clone()) > spec.key_for_comparison(left.key.clone()),
            ICS23Errors::INVALID_LEFT_KEY_ORDER,
        )
    }

    if let Option::Some(right) = proof.right {
        verify_existence(spec, right, right.key, right.value);
        assert(
            spec.key_for_comparison(key) < spec.key_for_comparison(right.key.clone()),
            ICS23Errors::INVALID_RIGHT_KEY_ORDER,
        )
    }

    match (proof.left, proof.right) {
        (Option::Some(left), Option::None) => ensure_right_most(spec.inner_spec.clone(), left.path),
        (
            Option::None, Option::Some(right),
        ) => ensure_left_most(spec.inner_spec.clone(), right.path),
        (
            Option::Some(left), Option::Some(right),
        ) => ensure_left_neighbor(spec.inner_spec.clone(), left.path.clone(), right.path.clone()),
        (Option::None, Option::None) => panic!("{}", ICS23Errors::MISSING_EXISTENCE_PROOFS),
    }
}

fn check_existence_spec(spec: @ProofSpec, proof: @ExistenceProof) {
    let is_iavl = spec.is_iavl();

    if is_iavl {
        ensure_leaf_prefix(proof.leaf.prefix);
    }
    ensure_leaf(proof.leaf, spec.leaf_spec);

    let path = proof.path;
    let inner_len = path.len();

    if spec.min_depth != @0 {
        assert(@inner_len >= spec.min_depth, ICS23Errors::INVALID_INNER_OP_SIZE);
        assert(@inner_len <= spec.max_depth, ICS23Errors::INVALID_INNER_OP_SIZE);
    }

    let mut i = 0;
    for inner_op in @path {
        if is_iavl {
            ensure_inner_prefix(inner_op.prefix, i, inner_op.hash);
        }
        ensure_inner(inner_op, spec);
        i += 1;
    }
}

fn ensure_leaf_prefix(prefix: @Array<u8>) {
    let mut prefix = prefix.clone();
    let rem = ensure_iavl_prefix(ref prefix, 0);
    assert(rem == 0, ICS23Errors::INVALID_LEAF_PREFIX);
}

fn ensure_iavl_prefix(ref prefix: Array<u8>, min_height: u64) -> u32 {
    let (height, _) = decode_varint_from_u8_array(ref prefix);
    assert(height >= min_height, ICS23Errors::INVALID_IAVL_HEIGHT_PREFIX);

    // Checks if the size and version get successfully decoded to u64 from prefix.
    decode_varint_from_u8_array(ref prefix);
    decode_varint_from_u8_array(ref prefix);
    prefix.len()
}

fn ensure_leaf(leaf: @LeafOp, leaf_spec: @LeafOp) {
    assert(leaf.hash == leaf_spec.hash, ICS23Errors::INVALID_HASH_OP);
    assert(leaf.prehash_key == leaf_spec.prehash_key, ICS23Errors::INVALID_PREHASH_KEY);
    assert(leaf.prehash_value == leaf_spec.prehash_value, ICS23Errors::INVALID_PREHASH_VALUE);
    assert(leaf.length == leaf_spec.length, ICS23Errors::INVALID_LENGTH_OP);
    assert(has_prefix(leaf.prefix, leaf_spec.prefix), ICS23Errors::INVALID_LEAF_PREFIX);
}

fn ensure_inner_prefix(prefix: @Array<u8>, min_height: u64, hash_op: @HashOp) {
    let mut prefix = prefix.clone();
    let rem = ensure_iavl_prefix(ref prefix, min_height);
    assert(rem == 0 || rem == 1 || rem == 34, ICS23Errors::INVALID_INNER_PREFIX);
    assert(hash_op == @HashOp::Sha256, ICS23Errors::INVALID_HASH_OP);
}

fn ensure_inner(inner: @InnerOp, spec: @ProofSpec) {
    let inner_spec = spec.inner_spec.clone();
    let inner_p_len = inner.prefix.len();
    let max_left_child_bytes = (inner_spec.child_order.len() - 1) * inner_spec.child_size;

    assert(inner.hash == @inner_spec.hash, ICS23Errors::INVALID_HASH_OP);
    assert(!has_prefix(inner.prefix, spec.leaf_spec.prefix), ICS23Errors::INVALID_INNER_PREFIX);
    assert(inner_p_len >= inner_spec.min_prefix_length, ICS23Errors::INVALID_INNER_PREFIX_LEN);
    assert(
        inner_p_len <= inner_spec.max_prefix_length + max_left_child_bytes,
        ICS23Errors::INVALID_INNER_PREFIX_LEN,
    );
    assert(inner_spec.child_size > 0, ICS23Errors::ZERO_CHILD_SIZE);
    assert(
        inner_spec.min_prefix_length + inner_spec.child_size > inner_spec.max_prefix_length,
        ICS23Errors::INVALID_INNER_PREFIX_LEN,
    );
    assert(inner.suffix.len() % inner_spec.child_size == 0, ICS23Errors::INVALID_INNER_SUFFIX);
}

fn has_prefix(proof_prefix: @Array<u8>, spec_prefix: @Array<u8>) -> bool {
    let spec_prefix_len = spec_prefix.len();
    let proof_prefix_len = proof_prefix.len();

    if spec_prefix_len > proof_prefix_len {
        return false;
    }

    let mut proof_prefix_span = proof_prefix.span();
    let mut spec_prefix_span = spec_prefix.span();

    let mut has_prefix = true;

    while let Option::Some(s_prefix) = spec_prefix_span.pop_front() {
        if proof_prefix_span.pop_front().unwrap() != s_prefix {
            has_prefix = false;
            break;
        }
    }

    return has_prefix;
}

// Fails unless this is the left-most path in the tree, excluding placeholder (empty child) nodes.
fn ensure_left_most(inner_spec: InnerSpec, path: @Array<InnerOp>) {
    let pad = get_padding(inner_spec.clone(), 0);
    for step in path {
        assert(
            has_padding(step, @pad) || left_branches_are_empty(inner_spec.clone(), step),
            ICS23Errors::STEP_NOT_LEFT_MOST,
        );
    };
}

// Fails unless this is the right-most path in the tree, excluding placeholder (empty child) nodes.
fn ensure_right_most(inner_spec: InnerSpec, path: @Array<InnerOp>) {
    let pad = get_padding(inner_spec.clone(), inner_spec.child_order.len() - 1);
    for step in path {
        assert(
            has_padding(step, @pad) || right_branches_are_empty(@inner_spec, step),
            ICS23Errors::STEP_NOT_RIGHT_MOST,
        );
    };
}

fn ensure_left_neighbor(
    inner_spec: InnerSpec, left_path: Array<InnerOp>, right_path: Array<InnerOp>,
) {
    let mut left_path_span = left_path.span();
    let mut right_path_span = right_path.span();

    let mut top_left = left_path_span.pop_back().unwrap();
    let mut top_right = right_path_span.pop_back().unwrap();

    while top_left.prefix == top_right.prefix && top_left.suffix == top_right.suffix {
        top_left = left_path_span.pop_back().unwrap();
        top_right = right_path_span.pop_back().unwrap();
    }

    assert(
        is_left_step(inner_spec.clone(), top_left, top_right), ICS23Errors::INVALID_LEFT_NEIGHBOR,
    );
    ensure_right_most(inner_spec.clone(), @left_path_span.into());
    ensure_left_most(inner_spec, @right_path_span.into());
}

fn is_left_step(inner_spec: InnerSpec, left_op: @InnerOp, right_op: @InnerOp) -> bool {
    let left_idx = order_from_padding(inner_spec.clone(), left_op);
    let right_idx = order_from_padding(inner_spec, right_op);
    left_idx + 1 == right_idx
}

#[derive(Clone, Drop, Debug, Default, PartialEq)]
pub struct Padding {
    pub min_prefix: u32,
    pub max_prefix: u32,
    pub suffix: u32,
}

fn get_padding(inner_spec: InnerSpec, branch: u32) -> Padding {
    let mut padding = Option::None;
    let child_order_len = inner_spec.child_order.len();
    for o in inner_spec.child_order {
        if o == branch {
            let prefix = o * inner_spec.child_size;
            let suffix = inner_spec.child_size * (child_order_len - 1 - o);
            padding =
                Option::Some(
                    Padding {
                        min_prefix: prefix + inner_spec.min_prefix_length,
                        max_prefix: prefix + inner_spec.max_prefix_length,
                        suffix,
                    },
                );
            break;
        }
    }
    assert(padding.is_some(), ICS23Errors::MISSING_BRANCH);
    padding.unwrap()
}

fn has_padding(inner_op: @InnerOp, pad: @Padding) -> bool {
    inner_op.prefix.len() >= *pad.min_prefix
        && inner_op.prefix.len() <= *pad.max_prefix
        && inner_op.suffix.len() == *pad.suffix
}

fn order_from_padding(inner_spec: InnerSpec, inner_op: @InnerOp) -> u32 {
    let mut order = Option::None;
    let len = inner_spec.child_order.len();
    for branch in 0..len {
        let padding = get_padding(inner_spec.clone(), branch);
        if has_padding(inner_op, @padding) {
            order = Option::Some(branch);
            break;
        }
    }
    assert(order.is_some(), ICS23Errors::MISMATCHED_PADDING);
    order.unwrap()
}

fn left_branches_are_empty(inner_spec: InnerSpec, inner_op: @InnerOp) -> bool {
    let inner_spec_child_order = inner_spec.child_order.clone();
    let child_size = inner_spec.child_size.clone();
    let inner_spec_empty_child = inner_spec.empty_child.clone();
    let left_branches = order_from_padding(inner_spec, inner_op);
    if left_branches.is_zero() {
        return false;
    }

    let actual_prefix = inner_op.prefix.len().checked_sub(left_branches * child_size);
    if actual_prefix.is_none() {
        return false;
    }

    for lb in 0..left_branches {
        for o in @inner_spec_child_order {
            if o == @lb {
                let from = actual_prefix.unwrap() + child_size * o.clone();
                let mut expected_prefix = ArrayTrait::new();
                for i in from..from + child_size {
                    expected_prefix.append(*inner_op.prefix[i]);
                }
                if inner_spec_empty_child != expected_prefix {
                    return false;
                }
            }
        };
    }
    return true;
}

fn right_branches_are_empty(inner_spec: @InnerSpec, inner_op: @InnerOp) -> bool {
    let right_branches = order_from_padding(inner_spec.clone(), inner_op);
    if right_branches.is_zero() {
        return false;
    }

    let child_size = inner_spec.child_size.clone();
    if inner_op.suffix.len() != child_size {
        return false;
    }

    let mut are_empty = true;
    for rb in 0..right_branches {
        for o in inner_spec.child_order.clone() {
            if o == rb {
                let from = child_size * o;
                let mut expected_suffix = ArrayTrait::new();
                for i in from..from + child_size {
                    expected_suffix.append(*inner_op.suffix[i]);
                }
                if inner_spec.empty_child != @expected_suffix {
                    are_empty = false;
                    break;
                }
            }
        };
    }
    are_empty
}

