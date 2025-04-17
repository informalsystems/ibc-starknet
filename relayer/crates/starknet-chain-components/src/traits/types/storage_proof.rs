use cgp::prelude::*;

#[cgp_type]
pub trait HasStorageProofType {
    type StorageProof: Async;
}

#[cgp_type]
pub trait HasStorageKeyType {
    type StorageKey: Async;
}
