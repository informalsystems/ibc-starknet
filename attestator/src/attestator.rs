use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use rocket::serde::{Deserialize, Serialize};
use starknet_crypto::{Felt, sign};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Ed25519 {
    pub message: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
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
        let message = Felt::ZERO; // FIXME: hash the challenges
        let k = Felt::TWO; // FIXME: randomize this
        sign(private_key, &message, &k)
            .map(|signature| (signature.r, signature.s))
            .ok()
    } else {
        None
    }
}
