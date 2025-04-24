use cgp::prelude::*;

/**
    A storage proof contains multiple Merkle proofs that are relative to the global state root.

    The verification of a storage proof is consist of multiple verification of Merkle proofs.

    Note: The storage proof type _may_ be the same as the `CommitmentProof` abstract type. But we
    are defining a Starknet-specific abstract type here for now for clarity. Feel free to remove
    this type if it is feasible to use `CommitmentProof` instead.
*/
#[cgp_type]
pub trait HasStorageProofType {
    type StorageProof: Async;
}

#[cgp_type]
pub trait HasStorageKeyType {
    type StorageKey: Async;
}
