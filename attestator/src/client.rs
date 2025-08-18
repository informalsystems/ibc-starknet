use crate::Ed25519;

pub fn get_attestation(addr: &str, challenges: Vec<Ed25519>) -> Vec<Vec<u8>> {
    ureq::post(&format!("{}/attest", addr))
        .send_json(&challenges)
        .unwrap()
        .body_mut()
        .read_json()
        .unwrap()
}
