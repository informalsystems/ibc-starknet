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

    pub async fn get<T: DeserializeOwned>(
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

    pub async fn get_block(&self, block_number: Option<u64>) -> Result<Block, ureq::Error> {
        self.get(GET_BLOCK_PATH, block_number).await
    }

    pub async fn get_public_key(&self, block_number: Option<u64>) -> Result<Felt, ureq::Error> {
        self.get(GET_PUBLIC_KEY, block_number).await
    }

    pub async fn get_signature(&self, block_number: Option<u64>) -> Result<Signature, ureq::Error> {
        self.get(GET_SIGNATURE, block_number).await
    }
}

#[cfg(test)]
mod tests {
    use starknet::macros::felt;
    use starknet_crypto::verify;

    use super::*;

    #[tokio::test]
    async fn test_sepolia_get_block() {
        let block = Endpoint::sepolia().get_block(Some(785794)).await.unwrap();
        assert_eq!(block.block_number, 785794);
        assert_eq!(
            block.block_hash,
            felt!("0x37b7814b6ca55e1a9f1f5d1069548110f783f8afc23c380ceadd21e7383b038")
        );
        assert!(block.validate());
    }

    #[tokio::test]
    async fn test_mainnet_get_block() {
        let block = Endpoint::mainnet().get_block(Some(1415244)).await.unwrap();
        assert_eq!(block.block_number, 1415244);
        assert_eq!(
            block.block_hash,
            felt!("0x1d690fa8011e5f0cf87052099772ad39188e40b9b42e2b4b274c2335387bc83")
        );
        assert!(block.validate());
    }

    #[tokio::test]
    async fn test_sepolia_get_public_key() {
        let pub_key = Endpoint::sepolia()
            .get_public_key(Some(785794))
            .await
            .unwrap();
        assert_eq!(pub_key, SEPOLIA_PUBLIC_KEY);
    }

    #[tokio::test]
    async fn test_mainnet_get_public_key() {
        let pub_key = Endpoint::mainnet()
            .get_public_key(Some(1415244))
            .await
            .unwrap();
        assert_eq!(pub_key, MAINNET_PUBLIC_KEY);
    }

    #[tokio::test]
    async fn test_sepolia_get_signature() {
        let signature = Endpoint::sepolia()
            .get_signature(Some(785794))
            .await
            .unwrap();

        assert_eq!(
            signature,
            Signature {
                block_hash: felt!(
                    "0x37b7814b6ca55e1a9f1f5d1069548110f783f8afc23c380ceadd21e7383b038"
                ),
                signature: [
                    felt!("0x7eeec392ab47f1bdf91b9084a471ff76a9a921eec59c36ff59e20a0c667ba6f"),
                    felt!("0x39836a94595614c1dce61bf88fcf7f5bdfb38ed618301fd1c94759e16831e9e"),
                ],
            }
        );
    }

    #[tokio::test]
    async fn test_mainnet_get_signature() {
        let signature = Endpoint::mainnet()
            .get_signature(Some(1415244))
            .await
            .unwrap();

        assert_eq!(
            signature,
            Signature {
                block_hash: felt!(
                    "0x1d690fa8011e5f0cf87052099772ad39188e40b9b42e2b4b274c2335387bc83"
                ),
                signature: [
                    felt!("0x1561afc7a7a36106bd7224a709ee54b1653152adce5f631c85e296ca6252109"),
                    felt!("0x58239310a16cfcd8bb6b6b89ace3f1a0b77a9fa854f1253faf7d6073a624fbd"),
                ],
            }
        );
    }

    #[tokio::test]
    async fn test_sepolia_signature() {
        let endpoint = Endpoint::sepolia();

        let public_key = endpoint.get_public_key(None).await.unwrap();

        let signature = endpoint.get_signature(None).await.unwrap();

        assert!(verify(
            &public_key,
            &signature.block_hash,
            &signature.signature[0],
            &signature.signature[1],
        )
        .unwrap());
    }

    #[tokio::test]
    async fn test_mainnet_signature() {
        let endpoint = Endpoint::mainnet();

        let public_key = endpoint.get_public_key(None).await.unwrap();

        let signature = endpoint.get_signature(None).await.unwrap();

        assert!(verify(
            &public_key,
            &signature.block_hash,
            &signature.signature[0],
            &signature.signature[1],
        )
        .unwrap());
    }
}
