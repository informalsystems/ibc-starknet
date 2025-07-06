use starknet_core::types::Felt;

pub trait StarknetCryptoFunctions {
    type Error: core::fmt::Debug;

    fn starknet_keccak(input: &[u8]) -> Felt;

    fn pedersen_hash(x: &Felt, y: &Felt) -> Felt;

    fn poseidon_hash_many(inputs: &[Felt]) -> Felt;

    fn verify(public_key: &Felt, message: &Felt, r: &Felt, s: &Felt) -> Result<bool, Self::Error>;
}

pub struct StarknetCryptoEmpty;

impl StarknetCryptoFunctions for StarknetCryptoEmpty {
    type Error = ();

    fn starknet_keccak(input: &[u8]) -> Felt {
        Felt::ZERO // Placeholder implementation
    }

    fn pedersen_hash(x: &Felt, y: &Felt) -> Felt {
        Felt::ZERO // Placeholder implementation
    }

    fn poseidon_hash_many(inputs: &[Felt]) -> Felt {
        Felt::ZERO // Placeholder implementation
    }

    fn verify(_public_key: &Felt, _message: &Felt, _r: &Felt, _s: &Felt) -> Result<bool, ()> {
        Ok(true) // Placeholder implementation
    }
}

pub struct StarknetCryptoLib;

impl StarknetCryptoFunctions for StarknetCryptoLib {
    type Error = starknet_crypto::VerifyError;

    fn starknet_keccak(input: &[u8]) -> Felt {
        starknet_core::utils::starknet_keccak(input)
    }

    fn pedersen_hash(x: &Felt, y: &Felt) -> Felt {
        starknet_crypto::pedersen_hash(x, y)
    }

    fn poseidon_hash_many(inputs: &[Felt]) -> Felt {
        starknet_crypto::poseidon_hash_many(inputs)
    }

    fn verify(
        public_key: &Felt,
        message: &Felt,
        r: &Felt,
        s: &Felt,
    ) -> Result<bool, starknet_crypto::VerifyError> {
        starknet_crypto::verify(public_key, message, r, s)
    }
}
