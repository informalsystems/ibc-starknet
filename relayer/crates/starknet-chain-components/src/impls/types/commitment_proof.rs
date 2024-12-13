use cgp::core::Async;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::proof::{
    CommitmentProofBytesGetter, CommitmentProofHeightGetter, HasCommitmentProofType,
    ProvideCommitmentProofType,
};

use crate::types::commitment_proof::StarknetCommitmentProof;

pub struct UseStarknetCommitmentProof;

impl<Chain: Async> ProvideCommitmentProofType<Chain> for UseStarknetCommitmentProof {
    type CommitmentProof = StarknetCommitmentProof;
}

impl<Chain> CommitmentProofHeightGetter<Chain> for UseStarknetCommitmentProof
where
    Chain: HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasHeightType<Height = u64>,
{
    fn commitment_proof_height(proof: &StarknetCommitmentProof) -> &u64 {
        &proof.proof_height
    }
}

impl<Chain> CommitmentProofBytesGetter<Chain> for UseStarknetCommitmentProof
where
    Chain: HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>,
{
    fn commitment_proof_bytes(proof: &StarknetCommitmentProof) -> &[u8] {
        &proof.proof_bytes
    }
}
