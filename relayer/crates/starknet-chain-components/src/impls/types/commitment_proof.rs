use cgp::prelude::*;
use hermes_core::chain_components::traits::{
    CommitmentProofBytesGetter, CommitmentProofBytesGetterComponent, CommitmentProofHeightGetter,
    CommitmentProofHeightGetterComponent, CommitmentProofTypeProviderComponent,
    HasCommitmentProofType, HasHeightType,
};

use crate::types::commitment_proof::StarknetCommitmentProof;

pub struct UseStarknetCommitmentProof;

delegate_components! {
    UseStarknetCommitmentProof {
        CommitmentProofTypeProviderComponent:
            UseType<StarknetCommitmentProof>,
    }
}

#[cgp_provider(CommitmentProofHeightGetterComponent)]
impl<Chain> CommitmentProofHeightGetter<Chain> for UseStarknetCommitmentProof
where
    Chain: HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasHeightType<Height = u64>,
{
    fn commitment_proof_height(proof: &StarknetCommitmentProof) -> &u64 {
        &proof.proof_height
    }
}

#[cgp_provider(CommitmentProofBytesGetterComponent)]
impl<Chain> CommitmentProofBytesGetter<Chain> for UseStarknetCommitmentProof
where
    Chain: HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>,
{
    fn commitment_proof_bytes(proof: &StarknetCommitmentProof) -> &[u8] {
        &proof.proof_bytes
    }
}
