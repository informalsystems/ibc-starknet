use cgp::prelude::*;

use crate::traits::types::commitment::{
    HasCommitmentPathType, HasCommitmentValueType, HasMerkleProofType,
};

#[cgp_component {
    provider: MerkleProofVerifier,
}]
pub trait CanVerifyMerkleProof:
    HasMerkleProofType + HasCommitmentPathType + HasCommitmentValueType + HasErrorType
{
    fn verify_merkle_proof(
        proof: &Self::MerkleProof,
        path: &Self::CommitmentPath,
        value: Option<&Self::CommitmentValue>,
    ) -> Result<(), Self::Error>;
}
