use starknet_block_verifier::*;
use starknet_crypto::Felt;

fn read_json<T: serde::de::DeserializeOwned>(path: &str) -> T {
    let file = std::fs::File::open(path).expect("Failed to open file");
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to read JSON")
}

#[rstest::fixture]
fn sepolia_block() -> Block {
    read_json("tests/fixtures/sepolia/785794/block.json")
}

#[rstest::fixture]
fn sepolia_signature() -> Signature {
    read_json("tests/fixtures/sepolia/785794/signature.json")
}

#[rstest::fixture]
fn sepolia_public_key() -> Felt {
    read_json("tests/fixtures/sepolia/785794/public_key.json")
}

#[rstest::fixture]
fn mainnet_block() -> Block {
    read_json("tests/fixtures/mainnet/1415244/block.json")
}

#[rstest::fixture]
fn mainnet_signature() -> Signature {
    read_json("tests/fixtures/mainnet/1415244/signature.json")
}

#[rstest::fixture]
fn mainnet_public_key() -> Felt {
    read_json("tests/fixtures/mainnet/1415244/public_key.json")
}

#[rstest::rstest]
fn test_sepolia(sepolia_block: Block, sepolia_signature: Signature, sepolia_public_key: Felt) {
    assert_eq!(sepolia_block.block_number, 785794);

    assert!(sepolia_block.validate::<StarknetCryptoLib>());

    assert_eq!(sepolia_block.block_hash, sepolia_signature.block_hash);

    assert_eq!(sepolia_public_key, SEPOLIA_PUBLIC_KEY);

    assert!(sepolia_block
        .verify_signature::<StarknetCryptoLib>(&sepolia_signature, &sepolia_public_key)
        .unwrap());
}

#[rstest::rstest]
fn test_mainnet(mainnet_block: Block, mainnet_signature: Signature, mainnet_public_key: Felt) {
    assert_eq!(mainnet_block.block_number, 1415244);

    assert!(mainnet_block.validate::<StarknetCryptoLib>());

    assert_eq!(mainnet_block.block_hash, mainnet_signature.block_hash);

    assert_eq!(mainnet_public_key, MAINNET_PUBLIC_KEY);

    assert!(mainnet_block
        .verify_signature::<StarknetCryptoLib>(&mainnet_signature, &mainnet_public_key)
        .unwrap());
}
