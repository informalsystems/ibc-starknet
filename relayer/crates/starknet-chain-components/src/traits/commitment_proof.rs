use cgp::prelude::*;
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;

use crate::traits::types::commitment::{HasCommitmentPathType, HasCommitmentValueType};

#[cgp_component {
    provider: CommitmentProofVerifier,
}]
pub trait CanVerifyCommitment:
    HasCommitmentProofType + HasCommitmentPathType + HasCommitmentValueType + HasErrorType
{
    fn verify_commitment(
        proof: &Self::CommitmentProof,
        path: &Self::CommitmentPath,
        value: Option<&Self::CommitmentValue>,
    ) -> Result<(), Self::Error>;
}
