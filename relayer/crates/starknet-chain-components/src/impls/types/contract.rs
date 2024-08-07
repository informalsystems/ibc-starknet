use cgp_core::Async;
use starknet::core::types::contract::SierraClass;
use starknet::core::types::Felt;

use crate::traits::types::contract_class::{
    ProvideContractClassHashType, ProvideContractClassType,
};

pub struct ProvideStarknetContractTypes;

impl<Chain: Async> ProvideContractClassType<Chain> for ProvideStarknetContractTypes {
    type ContractClass = SierraClass;
}

impl<Chain: Async> ProvideContractClassHashType<Chain> for ProvideStarknetContractTypes {
    type ContractClassHash = Felt;
}
