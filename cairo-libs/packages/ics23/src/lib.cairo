mod errors;
mod ops;
mod specs;
mod types;
mod verify;

pub use errors::ICS23Errors;

pub use specs::{iavl_spec, smt_spec, tendermint_spec};
pub use types::{
    CommitmentProof, ExistenceProof, ExistenceProofImpl, ExistenceProofTrait, HashOp, InnerOp,
    InnerSpec, KeyBytes, LeafOp, LengthOp, MerkleProof, NonExistenceProof, NonExistenceProofImpl,
    Proof, ProofSpec, ProofSpecImpl, ProofSpecTrait, RootBytes, ValueBytes,
};
pub use verify::{verify_existence, verify_membership, verify_non_existence, verify_non_membership};


#[cfg(test)]
mod tests {
    mod data;
    mod decode;
    mod ops;
    mod verify;
}
