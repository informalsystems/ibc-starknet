use cgp::prelude::*;

#[cgp_type]
pub trait HasMerkleProofType {
    type MerkleProof;
}
