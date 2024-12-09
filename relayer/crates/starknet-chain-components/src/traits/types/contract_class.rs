use cgp::prelude::*;

#[cgp_component {
  name: ContractClassTypeComponent,
  provider: ProvideContractClassType,
  context: Chain,
}]
pub trait HasContractClassType: Async {
    type ContractClass: Async;
}

#[cgp_component {
  name: ContractClassHashTypeComponent,
  provider: ProvideContractClassHashType,
  context: Chain,
}]
pub trait HasContractClassHashType: Async {
    type ContractClassHash: Async;
}
