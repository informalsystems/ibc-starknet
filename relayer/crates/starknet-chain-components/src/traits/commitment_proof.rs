use cgp::prelude::*;
use starknet::core::types::Felt;

use crate::traits::types::commitment::HasMerkleProofType;

#[cgp_component {
    name: StarknetMerkleProofVerifierComponent,
    provider: StarknetMerkleProofVerifier,
}]
pub trait CanVerifyStarknetMerkleProof: HasMerkleProofType + HasErrorType {
    fn verify_starknet_merkle_proof(
        proof: &Self::MerkleProof,
        path: Felt,
        value: Felt,
    ) -> Result<(), Self::Error>;
}
