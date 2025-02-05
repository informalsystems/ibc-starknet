use ics23::{ProofSpec, InnerSpec, LeafOp, HashOp, LengthOp};

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
        empty_child: "",
        hash: HashOp::Sha256,
    };
    ProofSpec {
        leaf_spec, inner_spec, min_depth: 0, max_depth: 0, prehash_key_before_comparison: false
    }
}

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
        empty_child: "",
        hash: HashOp::Sha256,
    };
    ProofSpec {
        leaf_spec, inner_spec, min_depth: 0, max_depth: 0, prehash_key_before_comparison: false
    }
}
