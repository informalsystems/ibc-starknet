use std::sync::OnceLock;

use attestator::{Ed25519, attest};
use rocket::serde::json::Json as Codec;
use rocket::{get, launch, post, routes};
use starknet_crypto::{Felt, get_public_key};

static PRIVATE_KEY: OnceLock<Felt> = OnceLock::new();

#[post("/attest", data = "<data>")]
fn attest_api(data: Codec<Vec<Ed25519>>) -> Option<Codec<Vec<(Felt, Felt)>>> {
    let private_key = PRIVATE_KEY.get_or_init(|| {
        Felt::from_hex(
            &std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY environment variable not set"),
        )
        .expect("Invalid PRIVATE_KEY format")
    });

    let challenges = data.into_inner();

    challenges
        .into_iter()
        .map(|challenge| attest(private_key, &challenge))
        .collect::<Option<Vec<_>>>()
        .map(Codec)
}

#[get("/public_key")]
fn public_key_api() -> Codec<Felt> {
    let private_key = PRIVATE_KEY.get_or_init(|| {
        Felt::from_hex(
            &std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY environment variable not set"),
        )
        .expect("Invalid PRIVATE_KEY format")
    });

    Codec(get_public_key(private_key))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![attest_api, public_key_api])
}
