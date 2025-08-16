use rand::Rng;
use starknet_crypto::Felt;

pub fn random_felt() -> Felt {
    let mut rng = rand::rng();
    let random_bytes: [u8; 32] = rng.random();
    Felt::from_bytes_be_slice(&random_bytes)
}
