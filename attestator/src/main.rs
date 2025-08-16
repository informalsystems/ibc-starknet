use std::sync::OnceLock;

use attestator::{Ed25519, attest};
#[cfg(feature = "json")]
use rocket::serde::json::Json as Codec;
#[cfg(not(feature = "json"))]
use rocket::serde::msgpack::MsgPack as Codec;
use rocket::{get, launch, post, routes};
use starknet_crypto::{Felt, get_public_key};

static PRIVATE_KEY: OnceLock<Felt> = OnceLock::new();

#[post("/attest", data = "<data>")]
fn attest_api(data: Codec<Vec<Ed25519>>) -> Option<Codec<Vec<u8>>> {
    let private_key = PRIVATE_KEY.get_or_init(|| {
        Felt::from_hex(
            &std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY environment variable not set"),
        )
        .expect("Invalid PRIVATE_KEY format")
    });

    let challenges = data.into_inner();

    let result = attest(private_key, &challenges);

    result.map(|(r, s)| Codec([r.to_bytes_be(), s.to_bytes_be()].concat().to_vec()))
}

#[get("/public_key")]
fn public_key_api() -> Option<Codec<Vec<u8>>> {
    let private_key = PRIVATE_KEY.get_or_init(|| {
        Felt::from_hex(
            &std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY environment variable not set"),
        )
        .expect("Invalid PRIVATE_KEY format")
    });

    let public_key = get_public_key(private_key);

    Some(Codec(public_key.to_bytes_be().to_vec()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![attest_api, public_key_api])
}
