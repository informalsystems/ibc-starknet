use std::sync::OnceLock;

use attestator::Ed25519;
use rocket::serde::json::Json as Codec;
use rocket::{get, launch, post, routes};
use starknet_crypto::{Felt, get_public_key};

static KEY: OnceLock<(Felt, Felt)> = OnceLock::new();

fn key() -> &'static (Felt, Felt) {
    KEY.get_or_init(|| {
        let private_key = Felt::from_hex(
            &std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY environment variable not set"),
        )
        .expect("Invalid PRIVATE_KEY format");
        let public_key = get_public_key(&private_key);
        (private_key, public_key)
    })
}

#[post("/attest", data = "<data>")]
fn attest_api(data: Codec<Vec<Ed25519>>) -> Option<Codec<(Felt, Vec<(Felt, Felt)>)>> {
    let (private_key, public_key) = key();

    let challenges = data.into_inner();

    challenges
        .into_iter()
        .map(|challenge| challenge.attest(private_key))
        .collect::<Option<Vec<_>>>()
        .map(|attestations| (*public_key, attestations))
        .map(Codec)
}

#[get("/public_key")]
fn public_key_api() -> Codec<Felt> {
    let (_, public_key) = key();

    Codec(*public_key)
}

#[launch]
fn rocket() -> _ {
    {
        let (_, _) = key(); // Ensure the private key is initialized at startup
    }
    rocket::build().mount("/", routes![attest_api, public_key_api])
}
