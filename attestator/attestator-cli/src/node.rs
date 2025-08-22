use std::sync::OnceLock;

use attestator::Ed25519;
use rocket::serde::json::Json as Codec;
use rocket::{get, launch, post, routes};
use starknet_crypto::{Felt, get_public_key};

static PRIVATE_KEY: OnceLock<Felt> = OnceLock::new();

fn private_key() -> &'static Felt {
    PRIVATE_KEY.get_or_init(|| {
        Felt::from_hex(
            &std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY environment variable not set"),
        )
        .expect("Invalid PRIVATE_KEY format")
    })
}

#[post("/attest", data = "<data>")]
fn attest_api(data: Codec<Vec<Ed25519>>) -> Option<Codec<Vec<(Felt, Felt)>>> {
    let private_key = private_key();

    let challenges = data.into_inner();

    challenges
        .into_iter()
        .map(|challenge| challenge.attest(private_key))
        .collect::<Option<Vec<_>>>()
        .map(Codec)
}

#[get("/public_key")]
fn public_key_api() -> Codec<Felt> {
    let private_key = private_key();

    Codec(get_public_key(private_key))
}

#[launch]
fn rocket() -> _ {
    {
        let _ = private_key(); // Ensure the private key is initialized at startup
    }
    rocket::build().mount("/", routes![attest_api, public_key_api])
}
