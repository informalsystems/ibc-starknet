use starknet_crypto::Felt;

pub trait StarknetCryptoFunctions {
    fn pedersen_hash(x: &Felt, y: &Felt) -> Felt;

    fn poseidon_hash_many(inputs: &[Felt]) -> Felt;

    fn verify(
        public_key: &Felt,
        message: &Felt,
        r: &Felt,
        s: &Felt,
    ) -> Result<bool, starknet_crypto::VerifyError>;
}

pub struct StarknetCryptoEmpty;

impl StarknetCryptoFunctions for StarknetCryptoEmpty {
    fn pedersen_hash(x: &Felt, y: &Felt) -> Felt {
        Felt::ZERO // Placeholder implementation
    }

    fn poseidon_hash_many(inputs: &[Felt]) -> Felt {
        Felt::ZERO // Placeholder implementation
    }

    fn verify(
        _public_key: &Felt,
        _message: &Felt,
        _r: &Felt,
        _s: &Felt,
    ) -> Result<bool, starknet_crypto::VerifyError> {
        Ok(true) // Placeholder implementation
    }
}

#[cfg(feature = "crypto")]
pub struct StarknetCryptoLib;

#[cfg(feature = "crypto")]
impl StarknetCryptoFunctions for StarknetCryptoLib {
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
