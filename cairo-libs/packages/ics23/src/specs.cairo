use ics23::{HashOp, InnerSpec, LeafOp, LengthOp, ProofSpec};

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L198
pub fn iavl_spec() -> ProofSpec {
    let leaf_spec = LeafOp {
        hash: HashOp::Sha256,
        prehash_key: HashOp::NoOp,
        prehash_value: HashOp::Sha256,
        length: LengthOp::VarProto,
        prefix: array![0],
    };
    let inner_spec = InnerSpec {
        child_order: array![0, 1],
        min_prefix_length: 4,
        max_prefix_length: 12,
        child_size: 33,
        empty_child: array![],
        hash: HashOp::Sha256,
    };
    ProofSpec {
        leaf_spec, inner_spec, min_depth: 0, max_depth: 0, prehash_key_before_comparison: false,
    }
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L332
pub fn tendermint_spec() -> ProofSpec {
    let leaf_spec = LeafOp {
        hash: HashOp::Sha256,
        prehash_key: HashOp::NoOp,
        prehash_value: HashOp::Sha256,
        length: LengthOp::VarProto,
        prefix: array![0],
    };
    let inner_spec = InnerSpec {
        child_order: array![0, 1],
        min_prefix_length: 1,
        max_prefix_length: 1,
        child_size: 32,
        empty_child: array![],
        hash: HashOp::Sha256,
    };
    ProofSpec {
        leaf_spec, inner_spec, min_depth: 0, max_depth: 0, prehash_key_before_comparison: false,
    }
}

// https://github.com/cosmos/ics23/blob/a324422529b8c00ead00b4dcee825867c494cddd/rust/src/api.rs#L357
pub fn smt_spec() -> ProofSpec {
    let leaf_spec = LeafOp {
        hash: HashOp::Sha256,
        prehash_key: HashOp::Sha256,
        prehash_value: HashOp::Sha256,
        length: LengthOp::NoPrefix,
        prefix: array![0],
    };
    let inner_spec = InnerSpec {
        child_order: array![0, 1],
        min_prefix_length: 1,
        max_prefix_length: 1,
        child_size: 32,
        empty_child: [0; 32].span().into(),
        hash: HashOp::Sha256,
    };
    ProofSpec {
        leaf_spec, inner_spec, min_depth: 0, max_depth: 0, prehash_key_before_comparison: true,
    }
}
