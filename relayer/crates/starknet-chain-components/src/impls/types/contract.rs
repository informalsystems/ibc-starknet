use cgp::prelude::*;
use starknet::core::types::contract::SierraClass;
use starknet::core::types::Felt;

use crate::components::chain::{ContractClassHashTypeComponent, ContractClassTypeComponent};
use crate::traits::types::contract_class::{
    ProvideContractClassHashType, ProvideContractClassType,
};

pub struct ProvideStarknetContractTypes;

#[cgp_provider(ContractClassTypeComponent)]
impl<Chain: Async> ProvideContractClassType<Chain> for ProvideStarknetContractTypes {
    type ContractClass = SierraClass;
}

#[cgp_provider(ContractClassHashTypeComponent)]
impl<Chain: Async> ProvideContractClassHashType<Chain> for ProvideStarknetContractTypes {
    type ContractClassHash = Felt;
}
