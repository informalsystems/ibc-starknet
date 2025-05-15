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
    ) -> Result<T, reqwest::Error> {
        let url_params = block_number.map(|num| ("blockNumber", num.to_string()));

        let url = reqwest::Url::parse_with_params(
            &format!("{}/feeder_gateway/{path}", self.0),
            url_params,
        )
        .unwrap();

        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;

        let result = response.json().await?;

        Ok(result)
    }

    pub async fn get_block(&self, block_number: Option<u64>) -> Result<Block, reqwest::Error> {
        self.get(GET_BLOCK_PATH, block_number).await
    }

    pub async fn get_public_key(&self, block_number: Option<u64>) -> Result<Felt, reqwest::Error> {
        self.get(GET_PUBLIC_KEY, block_number).await
    }

    pub async fn get_signature(
        &self,
        block_number: Option<u64>,
    ) -> Result<Signature, reqwest::Error> {
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
        let block = Endpoint::sepolia().get_block(Some(732297)).await.unwrap();
        assert_eq!(block.block_number, 732297);
        assert_eq!(
            block.block_hash,
            felt!("0x116702238e674269e8cf96de73d917e9bd79dec067cb528d57da863fe1d8468")
        );
        assert!(block.validate());
    }

    #[tokio::test]
    async fn test_mainnet_get_block() {
        let block = Endpoint::mainnet().get_block(Some(732297)).await.unwrap();
        assert_eq!(block.block_number, 732297);
        assert_eq!(
            block.block_hash,
            felt!("0x65b1882d075244cc8319b4a8155a69c8a98d8cc81c32f69ee2bc0f241a8f7e")
        );
        // mainnet is still on 0.13.2
        // currently, we are computing hash for >= 0.13.3
        // assert!(block.validate());
    }

    #[tokio::test]
    async fn test_sepolia_get_public_key() {
        let pub_key = Endpoint::sepolia()
            .get_public_key(Some(732297))
            .await
            .unwrap();
        assert_eq!(pub_key, SEPOLIA_PUBLIC_KEY);
    }

    #[tokio::test]
    async fn test_mainnet_get_public_key() {
        let pub_key = Endpoint::mainnet()
            .get_public_key(Some(732297))
            .await
            .unwrap();
        assert_eq!(pub_key, MAINNET_PUBLIC_KEY);
    }

    #[tokio::test]
    async fn test_sepolia_get_signature() {
        let signature = Endpoint::sepolia()
            .get_signature(Some(732297))
            .await
            .unwrap();

        assert_eq!(
            signature,
            Signature {
                block_hash: felt!(
                    "0x116702238e674269e8cf96de73d917e9bd79dec067cb528d57da863fe1d8468"
                ),
                signature: [
                    felt!("0x5b045b193b3d8a72d5ebc9145b6a7426816aca4969d471e6fc51383caf99c13"),
                    felt!("0x703e65a44b1780e6ae0f23f4dc4cabe7bab1ece109d551497843f51487f6773"),
                ],
            }
        );
    }

    #[tokio::test]
    async fn test_mainnet_get_signature() {
        let signature = Endpoint::mainnet()
            .get_signature(Some(732297))
            .await
            .unwrap();

        assert_eq!(
            signature,
            Signature {
                block_hash: felt!(
                    "0x65b1882d075244cc8319b4a8155a69c8a98d8cc81c32f69ee2bc0f241a8f7e"
                ),
                signature: [
                    felt!("0x760b20a5120b1d1164abe192128ab83a18f59c63e4b4eeb9a6eb56e426359e7"),
                    felt!("0x6d56d7f23af14c8dce749208a8240e4a766a2d51a0d08a6a73ecdd89fe70460"),
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
