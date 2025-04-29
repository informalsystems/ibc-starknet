use hermes_prelude::*;
use starknet::core::types::contract::SierraClass;
use starknet::core::types::Felt;

use crate::traits::{ContractClassHashTypeProviderComponent, ContractClassTypeProviderComponent};

pub struct UseStarknetContractTypes;

delegate_components! {
    UseStarknetContractTypes {
        ContractClassTypeProviderComponent:
            UseType<SierraClass>,
        ContractClassHashTypeProviderComponent:
            UseType<Felt>,
    }
}
