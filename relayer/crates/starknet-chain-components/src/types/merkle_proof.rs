use indexmap::IndexMap;
use starknet::core::types::{Felt, MerkleNode};

pub struct StarknetMerkleProof {
    pub root: Felt,
    pub proof_nodes: IndexMap<Felt, MerkleNode>,
}
