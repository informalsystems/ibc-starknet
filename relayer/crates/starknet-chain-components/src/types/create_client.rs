use hermes_core::relayer_components::chain::traits::{
    CreateClientMessageOptionsTypeComponent, ProvideCreateClientMessageOptionsType,
};
use hermes_prelude::*;

#[derive(Clone, Debug)]
pub struct CreateWasmStarknetMessageOptions {
    pub crypto_cw_address: WasmAddress,
}

#[derive(Clone, Debug)]
pub enum WasmAddress {
    ContractAddress(String),
    ContractAddressPath(String),
}

impl WasmAddress {
    pub fn get_contract_address(&self) -> String {
        match self {
            Self::ContractAddress(address) => address.clone(),
            Self::ContractAddressPath(path) => {
                let cw_address_file = std::fs::read_to_string(path).expect("failed to read file");
                cw_address_file
                    .trim()
                    .split_once('=')
                    .map(|(_, v)| v.trim().to_string())
                    .expect("failed extract wasm contract address")
            }
        }
    }
}

pub struct ProvideCreateWasmStarknetMessageOptionsType;

#[cgp_provider(CreateClientMessageOptionsTypeComponent)]
impl<Chain, Counterparty> ProvideCreateClientMessageOptionsType<Chain, Counterparty>
    for ProvideCreateWasmStarknetMessageOptionsType
where
    Chain: Async,
{
    type CreateClientMessageOptions = CreateWasmStarknetMessageOptions;
}
