use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use rocket::serde::{Deserialize, Serialize};
use starknet_core::crypto::ecdsa_sign;
use starknet_core::types::Felt;
use starknet_crypto::poseidon_hash_many;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Ed25519 {
    pub message: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

pub fn serialize_challenges(challenges: &[Ed25519]) -> Vec<Felt> {
    let mut serialized = vec![Felt::from(challenges.len())];

    for challenge in challenges {
        serialized.extend(challenge.message.iter().map(|&byte| Felt::from(byte)));
        serialized.extend(challenge.signature.iter().map(|&byte| Felt::from(byte)));
        serialized.extend(challenge.public_key.iter().map(|&byte| Felt::from(byte)));
    }
    serialized
}

impl Ed25519 {
    pub fn verify(&self) -> bool {
        let verifying_key =
            VerifyingKey::from_bytes(&self.public_key.clone().try_into().unwrap()).unwrap();
        let signature = Signature::from_bytes(&self.signature.clone().try_into().unwrap());
        verifying_key.verify(&self.message, &signature).is_ok()
    }
}

pub fn attest(private_key: &Felt, challenges: &[Ed25519]) -> Option<(Felt, Felt)> {
    if challenges.iter().all(|challenge| challenge.verify()) {
        let message = poseidon_hash_many(&serialize_challenges(challenges));

        ecdsa_sign(private_key, &message)
            .map(|signature| (signature.r, signature.s))
            .ok()
    } else {
        None
    }
}
