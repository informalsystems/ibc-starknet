use cgp::prelude::*;
use starknet::core::types::contract::SierraClass;
use starknet::core::types::Felt;

use crate::traits::types::contract_class::{
    ContractClassHashTypeProviderComponent, ContractClassTypeProviderComponent,
};

pub struct UseStarknetContractTypes;

delegate_components! {
    UseStarknetContractTypes {
        ContractClassTypeProviderComponent:
            UseType<SierraClass>,
        ContractClassHashTypeProviderComponent:
            UseType<Felt>,
    }
}
