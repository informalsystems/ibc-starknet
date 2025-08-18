use starknet_crypto::Felt;

use crate::Ed25519;

pub fn get_attestation(addr: &str, challenges: Vec<Ed25519>) -> Vec<(Felt, Felt)> {
    ureq::post(&format!("{}/attest", addr))
        .send_json(&challenges)
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
