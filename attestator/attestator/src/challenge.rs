use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use starknet_core::crypto::ecdsa_sign;
use starknet_core::types::Felt;
use starknet_crypto::poseidon_hash_many;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ed25519 {
    pub message: Vec<u8>,
    // length 64
    pub signature: Vec<u8>,
    // length 32
    pub public_key: Vec<u8>,
}

impl Ed25519 {
    pub fn verify(&self) -> bool {
        let verifying_key =
            VerifyingKey::from_bytes(&self.public_key.clone().try_into().unwrap()).unwrap();
        let signature = Signature::from_bytes(&self.signature.clone().try_into().unwrap());
        verifying_key.verify(&self.message, &signature).is_ok()
    }

    pub fn cairo_serialize(&self) -> Vec<Felt> {
        let mut serialized = Vec::new();

        serialized.push(Felt::from(self.message.len()));
        serialized.extend(self.message.iter().map(|&byte| Felt::from(byte)));
        serialized.extend(self.signature.iter().map(|&byte| Felt::from(byte)));
        serialized.extend(self.public_key.iter().map(|&byte| Felt::from(byte)));

        serialized
    }

    pub fn signed_message(&self) -> Felt {
        poseidon_hash_many(&self.cairo_serialize())
    }

    pub fn attest(&self, private_key: &Felt) -> Option<(Felt, Felt)> {
        if self.verify() {
            ecdsa_sign(private_key, &self.signed_message())
                .map(|signature| (signature.r, signature.s))
                .ok()
        } else {
            None
        }
    }
}
