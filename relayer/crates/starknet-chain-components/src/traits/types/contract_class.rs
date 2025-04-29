use hermes_prelude::*;

#[cgp_type]
pub trait HasContractClassType: Async {
    type ContractClass: Async;
}

#[cgp_type]
pub trait HasContractClassHashType: Async {
    type ContractClassHash: Async;
}
