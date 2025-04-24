use cgp::prelude::*;

/**
   A Starknet Merkle proof is a part of the larger storage proof,
   which contains multiple related Merkle proofs.
*/
#[cgp_type]
pub trait HasMerkleProofType {
    type MerkleProof;
}
