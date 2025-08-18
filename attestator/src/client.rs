use starknet_crypto::Felt;

use crate::Ed25519;

pub fn get_attestation(addr: &str, challenges: &[Ed25519]) -> Vec<(Felt, Felt)> {
    ureq::post(&format!("{}/attest", addr))
        .send_json(challenges)
        .unwrap()
        .body_mut()
        .read_json()
        .unwrap()
}

pub fn get_public_key(addr: &str) -> Felt {
    ureq::get(&format!("{}/public_key", addr))
        .call()
        .unwrap()
        .body_mut()
        .read_json()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use starknet_crypto::{Felt, get_public_key, poseidon_hash_many, verify};

    use super::*;
    use crate::serialize_challenge;

    #[test]
    #[ignore = "manual testing"]
    fn test_get_attestation() {
        let addr = "http://localhost:8000";

        // test 3 from https://datatracker.ietf.org/doc/html/rfc8032
        let challenge = Ed25519 {
            message: vec![0xaf, 0x82],
            signature: [
                0x6291d657deec2402,
                0x4827e69c3abe01a3,
                0x0ce548a284743a44,
                0x5e3680d7db5ac3ac,
                0x18ff9b538d16f290,
                0xae67f760984dc659,
                0x4a7c15e9716ed28d,
                0xc027beceea1ec40a,
            ],
            public_key: [
                0xfc51cd8e6218a1a3,
                0x8da47ed00230f058,
                0x0816ed13ba3303ac,
                0x5deb911548908025,
            ],
        };

        let [(r, s)] = get_attestation(addr, std::slice::from_ref(&challenge))
            .try_into()
            .unwrap();

        let message = poseidon_hash_many(&serialize_challenge(&challenge));
        let private_key = Felt::from_hex("0x1234").unwrap();
        let public_key = get_public_key(&private_key);

        assert!(
            verify(&public_key, &message, &r, &s).unwrap(),
            "Signature verification failed"
        );
    }
}
