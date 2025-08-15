use crate::Ed25519;

pub fn get_attestation(addr: &str, challenges: Vec<Ed25519>) -> Vec<u8> {
    ureq::post(&format!("{}/attest", addr))
        .send_json(&challenges)
        .unwrap()
        .body_mut()
        .read_json()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use starknet_crypto::{Felt, get_public_key, verify};

    use super::*;

    #[test]
    fn test_get_attestation() {
        let addr = "http://localhost:8000";
        let challenges = vec![];
        let signature: [u8; 64] = get_attestation(addr, challenges).try_into().unwrap();

        let r = Felt::from_bytes_be_slice(&signature[0..32]);
        let s = Felt::from_bytes_be_slice(&signature[32..64]);
        let private_key = Felt::from_hex("0x1234").unwrap();
        let public_key = get_public_key(&private_key);

        assert!(
            verify(&public_key, &Felt::ZERO, &r, &s).unwrap(),
            "Signature verification failed"
        );
    }
}
