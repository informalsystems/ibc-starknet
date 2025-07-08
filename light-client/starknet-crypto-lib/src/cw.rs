use cosmwasm_std::{from_json, to_json_vec, Addr, QuerierWrapper, StdError, Storage};
use starknet_core::types::Felt;
use sylvia::types::BoundQuerier;

use crate::contract::sv::Querier;
use crate::funcs::StarknetCryptoFunctions;

pub struct StarknetCryptoCw<'a> {
    lib_addr: Addr,
    querier: QuerierWrapper<'a>,
}

impl<'a> StarknetCryptoCw<'a> {
    const LIB_ADDR_VAR: &'static [u8] = b"STARKNET_CRYPTO_LIB_ADDR";

    pub fn new(querier: QuerierWrapper<'a>, storage: &'a dyn Storage) -> Self {
        Self {
            lib_addr: Self::get_lib_contract(storage).unwrap(),
            querier,
        }
    }

    pub fn get_lib_contract(storage: &'a dyn Storage) -> Result<Addr, StdError> {
        from_json(
            storage
                .get(Self::LIB_ADDR_VAR)
                .ok_or_else(|| StdError::not_found("STARKNET_CRYPTO_LIB_ADDR"))?,
        )
    }

    pub fn set_lib_contract(
        storage: &'a mut dyn Storage,
        contract_address: Addr,
    ) -> Result<(), StdError> {
        storage.set(Self::LIB_ADDR_VAR, &to_json_vec(&contract_address)?);
        Ok(())
    }
}

impl StarknetCryptoFunctions for StarknetCryptoCw<'_> {
    type Error = StdError;

    fn starknet_keccak(&self, input: &[u8]) -> Felt {
        let felt_hex = BoundQuerier::borrowed(&self.lib_addr, &self.querier)
            .starknet_keccak(input.into())
            .unwrap();

        Felt::from_hex(&felt_hex).unwrap()
    }

    fn pedersen_hash(&self, x: &Felt, y: &Felt) -> Felt {
        let felt_hex = BoundQuerier::borrowed(&self.lib_addr, &self.querier)
            .pedersen_hash(x.to_fixed_hex_string(), y.to_fixed_hex_string())
            .unwrap();

        Felt::from_hex(&felt_hex).unwrap()
    }

    fn poseidon_hash_many(&self, inputs: &[Felt]) -> Felt {
        let input_hex: Vec<String> = inputs
            .iter()
            .map(|felt| felt.to_fixed_hex_string())
            .collect();

        let felt_hex = BoundQuerier::borrowed(&self.lib_addr, &self.querier)
            .poseidon_hash_many(input_hex)
            .unwrap();

        Felt::from_hex(&felt_hex).unwrap()
    }

    fn verify(
        &self,
        public_key: &Felt,
        message: &Felt,
        r: &Felt,
        s: &Felt,
    ) -> Result<bool, Self::Error> {
        BoundQuerier::borrowed(&self.lib_addr, &self.querier).verify(
            public_key.to_fixed_hex_string(),
            message.to_fixed_hex_string(),
            r.to_fixed_hex_string(),
            s.to_fixed_hex_string(),
        )
    }
}
