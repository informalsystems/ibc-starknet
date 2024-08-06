use cgp_core::prelude::*;

#[derive_component(ContractClassTypeComponent, ProvideContractClassType<Chain>)]
pub trait HasContractClassType: Async {
    type ContractClass: Async;
}

#[derive_component(ContractClassHashTypeComponent, ProvideContractClassHashType<Chain>)]
pub trait HasContractClassHashType: Async {
    type ContractClassHash: Async;
}
