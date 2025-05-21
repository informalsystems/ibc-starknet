use serde::de::DeserializeOwned;
use starknet_crypto::Felt;

use crate::{Block, Signature};

pub const MAINNET_FEEDER_URL: &str = "https://alpha-mainnet.starknet.io";
pub const SEPOLIA_FEEDER_URL: &str = "https://alpha-sepolia.starknet.io";

pub const MAINNET_PUBLIC_KEY: Felt =
    Felt::from_hex_unchecked("0x48253ff2c3bed7af18bde0b611b083b39445959102d4947c51c4db6aa4f4e58");
pub const SEPOLIA_PUBLIC_KEY: Felt =
    Felt::from_hex_unchecked("0x1252b6bce1351844c677869c6327e80eae1535755b611c66b8f46e595b40eea");

// https://community.starknet.io/t/feeder-gateway-deprecation/100233#full-list-of-endpoints-3
const GET_BLOCK_PATH: &str = "get_block";
const GET_PUBLIC_KEY: &str = "get_public_key";
const GET_SIGNATURE: &str = "get_signature";

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
        let mut req = ureq::get(&format!("{}/feeder_gateway/{}", self.0, path));

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
