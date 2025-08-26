use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use serde_with::hex::Hex;
use serde_with::serde_as;
use starknet_core::crypto::ecdsa_sign;
use starknet_core::types::Felt;
use starknet_crypto::poseidon_hash_many;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ed25519 {
    #[serde_as(as = "Hex")]
    pub message: Vec<u8>,
    #[serde_as(as = "Hex")]
    pub signature: [u8; 64],
    #[serde_as(as = "Hex")]
    pub public_key: [u8; 32],
}

impl Ed25519 {
    pub fn verify(&self) -> Option<()> {
        let verifying_key = VerifyingKey::from_bytes(&self.public_key).ok()?;
        let signature = Signature::from_bytes(&self.signature);
        verifying_key.verify(&self.message, &signature).ok()
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
        self.verify()?;

        ecdsa_sign(private_key, &self.signed_message())
            .map(|signature| (signature.r, signature.s))
            .ok()
    }
}
