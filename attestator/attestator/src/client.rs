use starknet_crypto::Felt;
use thiserror::Error;

use crate::Ed25519;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No Public Key")]
    MissingPublicKey,
    #[error("Invalid Signature")]
    InvalidSignature,
}

pub struct AttestatorClient<'a>(pub &'a str);

impl AttestatorClient<'_> {
    pub fn get_attestation(
        &self,
        challenges: &[Ed25519],
    ) -> Result<(Felt, Vec<(Felt, Felt)>), Error> {
        ureq::post(&format!("{}/attest", self.0))
            .send_json(challenges)
            .and_then(|mut resp| resp.body_mut().read_json())
            .map_err(|_| Error::InvalidSignature)
    }

    pub fn get_public_key(&self) -> Result<Felt, Error> {
        ureq::get(&format!("{}/public_key", self.0))
            .call()
            .and_then(|mut resp| resp.body_mut().read_json())
            .map_err(|_| Error::MissingPublicKey)
    }
}

#[cfg(test)]
mod tests {
    use starknet_crypto::{Felt, get_public_key, verify};

    use super::*;

    #[test]
    #[ignore = "manual testing"]
    fn test_get_attestation() {
        let addr = "http://localhost:1234";

        // test 3 from https://datatracker.ietf.org/doc/html/rfc8032
        let challenge = serde_json::from_value(serde_json::json!({
            "message": "af82",
            "signature": "6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a",
            "public_key": "fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025",
        })).unwrap();

        let client = AttestatorClient(addr);

        let [(r, s)] = client
            .get_attestation(std::slice::from_ref(&challenge))
            .unwrap()
            .try_into()
            .unwrap();

        let message = challenge.signed_message();
        let private_key = Felt::from_hex("0x1234").unwrap();
        let public_key = get_public_key(&private_key);

        assert!(
            verify(&public_key, &message, &r, &s).unwrap(),
            "Signature verification failed"
        );
    }
}
