use core::fmt::Debug;

use hermes_prelude::*;

#[cgp_type]
pub trait HasContractClassType: Async {
    type ContractClass: Async + Debug;
}

#[cgp_type]
pub trait HasContractClassHashType: Async {
    type ContractClassHash: Async;
}
