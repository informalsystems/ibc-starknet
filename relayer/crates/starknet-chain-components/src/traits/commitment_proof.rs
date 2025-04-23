use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use starknet::core::types::Felt;

use crate::traits::types::commitment::HasMerkleProofType;
use crate::traits::types::storage_proof::HasStorageProofType;

#[cgp_component {
    name: StarknetMerkleProofVerifierComponent,
    provider: StarknetMerkleProofVerifier,
}]
pub trait CanVerifyStarknetMerkleProof: HasMerkleProofType + HasErrorType {
    fn verify_starknet_merkle_proof(
        proof: &Self::MerkleProof,
        root: Felt,
        path: Felt,
        value: Felt,
    ) -> Result<(), Self::Error>;
}

#[cgp_component {
    name: StarknetStorageProofVerifierComponent,
    provider: StarknetStorageProofVerifier,
}]
pub trait CanVerifyStarknetStorageProof:
    HasStorageProofType + HasAddressType + HasErrorType
{
    fn verify_starknet_storage_proof(
        proof: &Self::StorageProof,
        contract_address: &Self::Address,
        path: Felt,
        value: Felt,
    ) -> Result<(), Self::Error>;
}
