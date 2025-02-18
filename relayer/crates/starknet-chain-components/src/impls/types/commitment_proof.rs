use cgp::prelude::*;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::proof::{
    CommitmentProofBytesGetter, CommitmentProofHeightGetter, HasCommitmentProofType,
    ProvideCommitmentProofType,
};
use hermes_cosmos_chain_components::components::client::{
    CommitmentProofBytesGetterComponent, CommitmentProofHeightGetterComponent,
    CommitmentProofTypeComponent,
};

use crate::types::commitment_proof::StarknetCommitmentProof;

pub struct UseStarknetCommitmentProof;

#[cgp_provider(CommitmentProofTypeComponent)]
impl<Chain: Async> ProvideCommitmentProofType<Chain> for UseStarknetCommitmentProof {
    type CommitmentProof = StarknetCommitmentProof;
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
