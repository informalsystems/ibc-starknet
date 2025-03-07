use cgp::prelude::*;
use starknet::core::types::contract::SierraClass;
use starknet::core::types::Felt;

use crate::traits::types::contract_class::{
    ContractClassHashTypeProvider, ContractClassHashTypeProviderComponent,
    ContractClassTypeProvider, ContractClassTypeProviderComponent,
};

pub struct ProvideStarknetContractTypes;

#[cgp_provider(ContractClassTypeProviderComponent)]
impl<Chain: Async> ContractClassTypeProvider<Chain> for ProvideStarknetContractTypes {
    type ContractClass = SierraClass;
}

#[cgp_provider(ContractClassHashTypeProviderComponent)]
impl<Chain: Async> ContractClassHashTypeProvider<Chain> for ProvideStarknetContractTypes {
    type ContractClassHash = Felt;
}
