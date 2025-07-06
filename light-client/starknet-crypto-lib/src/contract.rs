use starknet_core::types::Felt;
use sylvia::ctx::{InstantiateCtx, QueryCtx};
use sylvia::cw_std::{Binary, Response, StdError, StdResult};

use crate::funcs::{StarknetCryptoFunctions, StarknetCryptoLib};

pub struct StarknetLightClientLibraryContract {}

#[cfg_attr(not(feature = "library"), sylvia::entry_points)]
#[sylvia::contract]
impl StarknetLightClientLibraryContract {
    pub const fn new() -> Self {
        Self {}
    }

    #[sv::msg(instantiate)]
    pub fn instantiate(&self, ctx: InstantiateCtx<'_>) -> StdResult<Response> {
        Ok(Response::default())
    }

    #[sv::msg(query)]
    pub fn starknet_keccak(&self, ctx: QueryCtx<'_>, input: Binary) -> StdResult<String> {
        let hash = StarknetCryptoLib::starknet_keccak(&input);
        Ok(hash.to_fixed_hex_string())
    }

    #[sv::msg(query)]
    pub fn pedersen_hash(&self, ctx: QueryCtx<'_>, x: String, y: String) -> StdResult<String> {
        let x_felt = Felt::from_hex(&x).map_err(|e| StdError::generic_err(e.to_string()))?;
        let y_felt = Felt::from_hex(&y).map_err(|e| StdError::generic_err(e.to_string()))?;

        let hash = StarknetCryptoLib::pedersen_hash(&x_felt, &y_felt);
        Ok(hash.to_fixed_hex_string())
    }

    #[sv::msg(query)]
    pub fn poseidon_hash_many(&self, ctx: QueryCtx<'_>, felts: Vec<String>) -> StdResult<String> {
        let felts: Vec<Felt> = felts
            .into_iter()
            .map(|felt_str| {
                Felt::from_hex(&felt_str).map_err(|e| StdError::generic_err(e.to_string()))
            })
            .collect::<Result<Vec<Felt>, StdError>>()?;

        let hash = StarknetCryptoLib::poseidon_hash_many(&felts);

        Ok(hash.to_fixed_hex_string())
    }

    #[sv::msg(query)]
    pub fn verify(
        &self,
        ctx: QueryCtx<'_>,
        public_key: String,
        message: String,
        r: String,
        s: String,
    ) -> StdResult<bool> {
        let public_key_felt =
            Felt::from_hex(&public_key).map_err(|e| StdError::generic_err(e.to_string()))?;
        let message_felt =
            Felt::from_hex(&message).map_err(|e| StdError::generic_err(e.to_string()))?;
        let r_felt = Felt::from_hex(&r).map_err(|e| StdError::generic_err(e.to_string()))?;
        let s_felt = Felt::from_hex(&s).map_err(|e| StdError::generic_err(e.to_string()))?;

        StarknetCryptoLib::verify(&public_key_felt, &message_felt, &r_felt, &s_felt)
            .map_err(|e| StdError::generic_err(e.to_string()))
    }
}
