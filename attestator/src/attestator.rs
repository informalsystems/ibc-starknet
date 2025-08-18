use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use rocket::serde::{Deserialize, Serialize};
use starknet_core::crypto::ecdsa_sign;
use starknet_core::types::Felt;
use starknet_crypto::poseidon_hash_many;

use crate::u64_array_to_u8_array;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Ed25519 {
    pub message: Vec<u8>,
    pub signature: [u64; 8],
    pub public_key: [u64; 4],
}

pub fn serialize_challenge(challenge: &Ed25519) -> Vec<Felt> {
    let mut serialized = Vec::new();

    serialized.push(Felt::from(challenge.message.len()));
    serialized.extend(challenge.message.iter().map(|&byte| Felt::from(byte)));
    serialized.extend(challenge.signature.iter().map(|&byte| Felt::from(byte)));
    serialized.extend(challenge.public_key.iter().map(|&byte| Felt::from(byte)));

    serialized
}

impl Ed25519 {
    pub fn verify(&self) -> bool {
        let verifying_key =
            VerifyingKey::from_bytes(&u64_array_to_u8_array(&self.public_key)).unwrap();
        let signature = Signature::from_bytes(&u64_array_to_u8_array(&self.signature));
        verifying_key.verify(&self.message, &signature).is_ok()
    }
}

pub fn attest(private_key: &Felt, challenges: &Ed25519) -> Option<(Felt, Felt)> {
    if challenges.verify() {
        let message = poseidon_hash_many(&serialize_challenge(challenges));
        ecdsa_sign(private_key, &message)
            .map(|signature| (signature.r, signature.s))
            .ok()
    } else {
        None
    }
}
