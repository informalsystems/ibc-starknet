use starknet_core::types::StorageProof;
use starknet_crypto::{pedersen_hash, Felt};
use starknet_crypto_lib::StarknetCryptoLib;
use starknet_macros::felt;
use starknet_storage_verifier::validate::validate_storage_proof;
use starknet_storage_verifier::verifier::{
    verify_starknet_contract_proof, verify_starknet_global_contract_root,
    verify_starknet_merkle_proof, verify_starknet_storage_proof,
};
use starknet_storage_verifier::StorageError;

#[test]
fn test_verify_starknet_storage_proof() -> Result<(), Box<dyn core::error::Error>> {
    let storage_proof = serde_json::from_reader(std::fs::File::open(
        "tests/fixtures/storage_proof_success.json",
    )?)?;

    let key: Felt = felt!("0x0001");
    let value: Felt = felt!("0x9911");
    let contract_address: Felt =
        felt!("0x4017d0ad6ddbc7e97208e2639fc5bbf9856b4ede9a66a5995aec87b0d45837c");

    let state_root = felt!("0x2bba45af2d71e57b1f82f1668bc53184762e6212c22e69f9949e3a607022fd2");

    validate_storage_proof::<StarknetCryptoLib>(&storage_proof)?;
    let global_contract_trie_root =
        verify_starknet_global_contract_root::<StarknetCryptoLib>(&storage_proof, state_root)?;
    let contract_root = verify_starknet_contract_proof::<StarknetCryptoLib>(
        &storage_proof,
        global_contract_trie_root,
        contract_address,
    )?;

    Ok(verify_starknet_storage_proof(
        &storage_proof,
        contract_root,
        key,
        value,
    )?)
}

#[test]
fn test_verify_starknet_merkle_proof() -> Result<(), Box<dyn core::error::Error>> {
    let nodes = serde_json::from_reader(std::fs::File::open("tests/fixtures/merkle_proof.json")?)?;

    let root = felt!("0x368991d64cd97e90a9da1fd9f3d676875d5d29b7136a6ecf77ddc35704f4c27");
    let path = felt!("0x4017d0ad6ddbc7e97208e2639fc5bbf9856b4ede9a66a5995aec87b0d45837c");
    let value = pedersen_hash(
        &pedersen_hash(
            &pedersen_hash(
                &felt!("0x45ba727abaff9ae3a4311d7a30196e09d1f30aeeb3a8e157277793740d20f61"),
                &felt!("0x4592da9795f9fd7a042eb0cb0d4dae7b6894bd90ccb3e6ff360185db24301f7"),
            ),
            &Felt::ZERO,
        ),
        &Felt::ZERO,
    );

    Ok(verify_starknet_merkle_proof(&nodes, root, path, value)?)
}

#[test]
fn test_verify_starknet_storage_proof_failure() -> Result<(), Box<dyn core::error::Error>> {
    let storage_proof = serde_json::from_reader(std::fs::File::open(
        "tests/fixtures/storage_proof_failure.json",
    )?)?;

    let key: Felt = felt!("0x0001");
    let value: Felt = felt!("0x9911");
    let contract_address: Felt =
        felt!("0x4017d0ad6ddbc7e97208e2639fc5bbf9856b4ede9a66a5995aec87b0d45837c");

    let state_root = felt!("0x2bba45af2d71e57b1f82f1668bc53184762e6212c22e69f9949e3a607022fd2");

    assert!(matches!(
        validate_storage_proof::<StarknetCryptoLib>(&storage_proof),
        Err(StorageError::MismatchBinaryHash)
    ));

    Ok(())
}

#[test]
fn test_verify_starknet_merkle_proof_failure() -> Result<(), Box<dyn core::error::Error>> {
    let nodes = serde_json::from_reader(std::fs::File::open("tests/fixtures/merkle_proof.json")?)?;

    // root changed to invalid value
    let root = felt!("0xaaaaaaaa4cd97e90a9da1fd9f3d676875d5d29b7136a6ecf77ddc35704f4c27");
    let path = felt!("0x4017d0ad6ddbc7e97208e2639fc5bbf9856b4ede9a66a5995aec87b0d45837c");
    let value = pedersen_hash(
        &pedersen_hash(
            &pedersen_hash(
                &felt!("0x45ba727abaff9ae3a4311d7a30196e09d1f30aeeb3a8e157277793740d20f61"),
                &felt!("0x4592da9795f9fd7a042eb0cb0d4dae7b6894bd90ccb3e6ff360185db24301f7"),
            ),
            &Felt::ZERO,
        ),
        &Felt::ZERO,
    );

    assert!(matches!(
        verify_starknet_merkle_proof(&nodes, root, path, value),
        Err(StorageError::MissingRootProofNode)
    ));

    Ok(())
}

#[test]
fn test_verify_non_membership_proof() -> Result<(), Box<dyn core::error::Error>> {
    let _storage_proof: StorageProof = serde_json::from_reader(std::fs::File::open(
        "tests/fixtures/storage_proof_non_membership.json",
    )?)?;

    Ok(())
}
