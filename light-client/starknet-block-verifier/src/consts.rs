use starknet_core::types::Felt;

pub const MAINNET_FEEDER_URL: &str = "https://alpha-mainnet.starknet.io";
pub const SEPOLIA_FEEDER_URL: &str = "https://alpha-sepolia.starknet.io";

// https://alpha-mainnet.starknet.io/feeder_gateway/get_public_key
pub const MAINNET_PUBLIC_KEY: Felt =
    Felt::from_hex_unchecked("0x48253ff2c3bed7af18bde0b611b083b39445959102d4947c51c4db6aa4f4e58");

// https://alpha-sepolia.starknet.io/feeder_gateway/get_public_key
pub const SEPOLIA_PUBLIC_KEY: Felt =
    Felt::from_hex_unchecked("0x1252b6bce1351844c677869c6327e80eae1535755b611c66b8f46e595b40eea");
