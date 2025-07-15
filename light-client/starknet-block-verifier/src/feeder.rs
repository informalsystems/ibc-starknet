use alloc::string::{String, ToString};
use core::fmt::Write;

use serde::de::DeserializeOwned;
use starknet_core::types::Felt;

use crate::{Block, Signature, MAINNET_FEEDER_URL, SEPOLIA_FEEDER_URL};

// https://community.starknet.io/t/feeder-gateway-deprecation/100233#full-list-of-endpoints-3
const GET_BLOCK_PATH: &str = "get_block";
const GET_PUBLIC_KEY: &str = "get_public_key";
const GET_SIGNATURE: &str = "get_signature";

#[derive(Debug, Clone)]
pub struct Endpoint(pub String);

impl Endpoint {
    pub fn new(endpoint: &str) -> Self {
        Self(endpoint.to_string())
    }

    pub fn sepolia() -> Self {
        Self(SEPOLIA_FEEDER_URL.to_string())
    }

    pub fn mainnet() -> Self {
        Self(MAINNET_FEEDER_URL.to_string())
    }

    pub fn get<T: DeserializeOwned>(
        &self,
        path: &'static str,
        block_number: Option<u64>,
    ) -> Result<T, ureq::Error> {
        let mut text = String::new();
        write!(&mut text, "{}/feeder_gateway/{path}", self.0).expect("Failed to write to string");
        let mut req = ureq::get(&text);

        if let Some(block_number) = block_number {
            req = req.query("blockNumber", block_number.to_string());
        }

        let result = req.call()?.body_mut().read_json()?;

        Ok(result)
    }

    pub fn get_block_header(&self, block_number: Option<u64>) -> Result<Block, ureq::Error> {
        self.get(GET_BLOCK_PATH, block_number)
    }

    pub fn get_public_key(&self, block_number: Option<u64>) -> Result<Felt, ureq::Error> {
        self.get(GET_PUBLIC_KEY, block_number)
    }

    pub fn get_signature(&self, block_number: Option<u64>) -> Result<Signature, ureq::Error> {
        self.get(GET_SIGNATURE, block_number)
    }
}
