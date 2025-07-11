use starknet_core::types::Felt;
use sylvia::cw_std::{Addr, QuerierWrapper, StdError};
use sylvia::types::BoundQuerier;

use crate::contract::sv::Querier;
use crate::funcs::StarknetCryptoFunctions;

pub struct StarknetCryptoCw<'a> {
    lib_addr: String,
    querier: QuerierWrapper<'a>,
}

impl<'a> StarknetCryptoCw<'a> {
    pub fn new(lib_addr: String, querier: QuerierWrapper<'a>) -> Self {
        Self { lib_addr, querier }
    }
}

impl StarknetCryptoFunctions for StarknetCryptoCw<'_> {
    type Error = StdError;

    fn starknet_keccak(&self, input: &[u8]) -> Felt {
        let felt_hex = BoundQuerier::borrowed(&Addr::unchecked(&self.lib_addr), &self.querier)
            .starknet_keccak(input.into())
            .expect("failed Starknet keccak hash");

        Felt::from_hex(&felt_hex).expect("failed to convert hex to Felt)
    }

    fn pedersen_hash(&self, x: &Felt, y: &Felt) -> Felt {
        let felt_hex = BoundQuerier::borrowed(&Addr::unchecked(&self.lib_addr), &self.querier)
            .pedersen_hash(x.to_fixed_hex_string(), y.to_fixed_hex_string())
            .expect("failed pedersen hash");

        Felt::from_hex(&felt_hex).expect("failed to convert hex to Felt)
    }

    fn poseidon_hash_many(&self, inputs: &[Felt]) -> Felt {
        let input_hex: Vec<String> = inputs
            .iter()
            .map(|felt| felt.to_fixed_hex_string())
            .collect();

        let felt_hex = BoundQuerier::borrowed(&Addr::unchecked(&self.lib_addr), &self.querier)
            .poseidon_hash_many(input_hex)
            .expect("failed poseidon hash");

        Felt::from_hex(&felt_hex).expect("failed to convert hex to Felt)
    }

    fn verify(
        &self,
        public_key: &Felt,
        message: &Felt,
        r: &Felt,
        s: &Felt,
    ) -> Result<bool, Self::Error> {
        BoundQuerier::borrowed(&Addr::unchecked(&self.lib_addr), &self.querier).verify(
            public_key.to_fixed_hex_string(),
            message.to_fixed_hex_string(),
            r.to_fixed_hex_string(),
            s.to_fixed_hex_string(),
        )
    }
}
