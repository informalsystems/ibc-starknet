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
    // Verify with a merkle proof that the given path contains the given value.
    // The root given is assumed to be trusted.
    fn verify_starknet_merkle_proof(
        proof: &Self::MerkleProof,
        root: Felt,
        path: Felt, // todo: allow multiple key/values to be verified at once
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
    /**
       Verify from a storage proof that a contract contains a given value at the specified path.

       This also verifies that a contract has a given state root, and that state root is provable
       from the global stateroot of the blockchain.
    */
    fn verify_starknet_storage_proof(
        proof: &Self::StorageProof,
        contract_address: &Self::Address,
        path: Felt, // todo: allow multiple key/values to be verified at once
        value: Felt,
    ) -> Result<(), Self::Error>;
}
