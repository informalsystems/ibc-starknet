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
            VerifyingKey::from_bytes(&self.public_key.clone().try_into().unwrap()).unwrap();
        let signature = Signature::from_bytes(&self.signature.clone().try_into().unwrap());
        verifying_key.verify(&self.message, &signature).is_ok()
    }
}

pub fn attest(private_key: &Felt, challenges: &[Ed25519]) -> Option<Vec<(Felt, Felt)>> {
    if challenges.iter().all(|challenge| challenge.verify()) {
        challenges
            .iter()
            .map(|challenge| poseidon_hash_many(&serialize_challenge(challenge)))
            .map(|message| {
                ecdsa_sign(private_key, &message).map(|signature| (signature.r, signature.s))
            })
            .collect::<Result<Vec<_>, _>>()
            .ok()
    } else {
        None
    }
}
