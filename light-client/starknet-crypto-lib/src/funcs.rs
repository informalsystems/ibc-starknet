use starknet_core::types::Felt;

pub trait StarknetCryptoFunctions {
    type Error: core::fmt::Debug;

    fn starknet_keccak(&self, input: &[u8]) -> Felt;

    fn pedersen_hash(&self, x: &Felt, y: &Felt) -> Felt;

    fn poseidon_hash_many(&self, inputs: &[Felt]) -> Felt;

    fn verify(
        &self,
        public_key: &Felt,
        message: &Felt,
        r: &Felt,
        s: &Felt,
    ) -> Result<bool, Self::Error>;
}

pub struct StarknetCryptoEmpty;

impl StarknetCryptoFunctions for StarknetCryptoEmpty {
    type Error = ();

    fn starknet_keccak(&self, input: &[u8]) -> Felt {
        Felt::ZERO // Placeholder implementation
    }

    fn pedersen_hash(&self, x: &Felt, y: &Felt) -> Felt {
        Felt::ZERO // Placeholder implementation
    }

    fn poseidon_hash_many(&self, inputs: &[Felt]) -> Felt {
        Felt::ZERO // Placeholder implementation
    }

    fn verify(&self, public_key: &Felt, message: &Felt, r: &Felt, s: &Felt) -> Result<bool, ()> {
        Ok(true) // Placeholder implementation
    }
}

pub struct StarknetCryptoLib;

impl StarknetCryptoFunctions for StarknetCryptoLib {
    type Error = starknet_crypto::VerifyError;

    fn starknet_keccak(&self, input: &[u8]) -> Felt {
        starknet_core::utils::starknet_keccak(input)
    }

    fn pedersen_hash(&self, x: &Felt, y: &Felt) -> Felt {
        starknet_crypto::pedersen_hash(x, y)
    }

    fn poseidon_hash_many(&self, inputs: &[Felt]) -> Felt {
        starknet_crypto::poseidon_hash_many(inputs)
    }

    fn verify(
        &self,
        public_key: &Felt,
        message: &Felt,
        r: &Felt,
        s: &Felt,
    ) -> Result<bool, starknet_crypto::VerifyError> {
        starknet_crypto::verify(public_key, message, r, s)
    }
}
