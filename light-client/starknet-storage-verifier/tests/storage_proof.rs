use indexmap::IndexMap;
use starknet_core::types::{
    BinaryNode, ContractLeafData, ContractsProof, EdgeNode, GlobalRoots, MerkleNode, StorageProof,
};
use starknet_crypto::{pedersen_hash, Felt};
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

    validate_storage_proof(&storage_proof)?;
    let global_contract_trie_root =
        verify_starknet_global_contract_root(&storage_proof, state_root)?;
    let contract_root = verify_starknet_contract_proof(
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
fn test_verify_starknet_merkle_proof() {
    let mut nodes = IndexMap::new();

    nodes.insert(
        felt!("0x2ed240ee95949cd56bb6d1feb5974abd931b844fd090f6738c1166ab129f256"),
        MerkleNode::EdgeNode(EdgeNode {
            path: Felt::ZERO,
            length: 1,
            child: felt!("0x69caa1781f8cd2bd601d17d668e4268a609430ada713cbfd97c4f1a7b79a37e"),
        }),
    );
    nodes.insert(
        felt!("0x3fb08aa8d7ae250618a5eded665e72102624aca7988f30c1abf93478c918624"),
        MerkleNode::EdgeNode(EdgeNode {
            path: felt!("0x17d0ad6ddbc7e97208e2639fc5bbf9856b4ede9a66a5995aec87b0d45837c"),
            length: 244,
            child: felt!("0x5cea18c46bb31c421e40f94efd51a83ec4882ae72473bb41edd2652137b938e"),
        }),
    );
    nodes.insert(
        felt!("0x368991d64cd97e90a9da1fd9f3d676875d5d29b7136a6ecf77ddc35704f4c27"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x136287afa5c7e9d96deba14d1080672eef35240cc4940076d378e84ef6b7c26"),
            right: felt!("0xbe9a5b8558021942adf479733d2e345c5147b18d6c0aee40e5ee9fb85bc32"),
        }),
    );
    nodes.insert(
        felt!("0xe1081c844581366e4e04cd3523b382a40865746a8cd035e41c9806327852a0"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x5dc3360ed7d92cfc8acfc3d5eb35ce9402a2e6e0754ed82487f78630ee63e1d"),
            right: felt!("0x54cb87a02896d50c9822961eb21c41326a3730e03beebb4273e9f53ecb1aa10"),
        }),
    );
    nodes.insert(
        felt!("0x3d72d82b1e3e3eeda9fe96221dc8a2131f22e243382d0f16883568f927589ef"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0xe1081c844581366e4e04cd3523b382a40865746a8cd035e41c9806327852a0"),
            right: felt!("0x7f5c3d3f9a27468b74d8d9352d6104a12e51257bb9bd2b775bcef738eba3d88"),
        }),
    );
    nodes.insert(
        felt!("0xbe9a5b8558021942adf479733d2e345c5147b18d6c0aee40e5ee9fb85bc32"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x3d72d82b1e3e3eeda9fe96221dc8a2131f22e243382d0f16883568f927589ef"),
            right: felt!("0x50b7568824156bac7669bda45c4db7c579f6b799bd3fd3142332ecd083cceb6"),
        }),
    );
    nodes.insert(
        felt!("0x5dc3360ed7d92cfc8acfc3d5eb35ce9402a2e6e0754ed82487f78630ee63e1d"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x2ed240ee95949cd56bb6d1feb5974abd931b844fd090f6738c1166ab129f256"),
            right: felt!("0x4e50fb911906759c79313749ede30a1888a29e85e297f128cd9df6ee3a4cabb"),
        }),
    );
    nodes.insert(
        felt!("0x69caa1781f8cd2bd601d17d668e4268a609430ada713cbfd97c4f1a7b79a37e"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x3fb08aa8d7ae250618a5eded665e72102624aca7988f30c1abf93478c918624"),
            right: felt!("0x3781d4c7ddb1f323275a2640bb37924c979225308808987b3b16b7dd7893f1a"),
        }),
    );

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

    let verification_result = verify_starknet_merkle_proof(&nodes, root, path, value);
    assert!(
        verification_result.is_ok(),
        "Merkle proof verification failed"
    );
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
        validate_storage_proof(&storage_proof),
        Err(StorageError::MismatchBinaryHash)
    ));

    Ok(())
}

#[test]
fn test_verify_starknet_merkle_proof_failure() {
    let mut nodes = IndexMap::new();

    nodes.insert(
        felt!("0x2ed240ee95949cd56bb6d1feb5974abd931b844fd090f6738c1166ab129f256"),
        MerkleNode::EdgeNode(EdgeNode {
            path: Felt::ZERO,
            length: 1,
            child: felt!("0x69caa1781f8cd2bd601d17d668e4268a609430ada713cbfd97c4f1a7b79a37e"),
        }),
    );
    nodes.insert(
        felt!("0x3fb08aa8d7ae250618a5eded665e72102624aca7988f30c1abf93478c918624"),
        MerkleNode::EdgeNode(EdgeNode {
            path: felt!("0x17d0ad6ddbc7e97208e2639fc5bbf9856b4ede9a66a5995aec87b0d45837c"),
            length: 244,
            child: felt!("0x5cea18c46bb31c421e40f94efd51a83ec4882ae72473bb41edd2652137b938e"),
        }),
    );
    nodes.insert(
        felt!("0x368991d64cd97e90a9da1fd9f3d676875d5d29b7136a6ecf77ddc35704f4c27"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x136287afa5c7e9d96deba14d1080672eef35240cc4940076d378e84ef6b7c26"),
            right: felt!("0xbe9a5b8558021942adf479733d2e345c5147b18d6c0aee40e5ee9fb85bc32"),
        }),
    );
    nodes.insert(
        felt!("0xe1081c844581366e4e04cd3523b382a40865746a8cd035e41c9806327852a0"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x5dc3360ed7d92cfc8acfc3d5eb35ce9402a2e6e0754ed82487f78630ee63e1d"),
            right: felt!("0x54cb87a02896d50c9822961eb21c41326a3730e03beebb4273e9f53ecb1aa10"),
        }),
    );
    nodes.insert(
        felt!("0x3d72d82b1e3e3eeda9fe96221dc8a2131f22e243382d0f16883568f927589ef"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0xe1081c844581366e4e04cd3523b382a40865746a8cd035e41c9806327852a0"),
            right: felt!("0x7f5c3d3f9a27468b74d8d9352d6104a12e51257bb9bd2b775bcef738eba3d88"),
        }),
    );
    nodes.insert(
        felt!("0xbe9a5b8558021942adf479733d2e345c5147b18d6c0aee40e5ee9fb85bc32"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x3d72d82b1e3e3eeda9fe96221dc8a2131f22e243382d0f16883568f927589ef"),
            right: felt!("0x50b7568824156bac7669bda45c4db7c579f6b799bd3fd3142332ecd083cceb6"),
        }),
    );
    nodes.insert(
        felt!("0x5dc3360ed7d92cfc8acfc3d5eb35ce9402a2e6e0754ed82487f78630ee63e1d"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x2ed240ee95949cd56bb6d1feb5974abd931b844fd090f6738c1166ab129f256"),
            right: felt!("0x4e50fb911906759c79313749ede30a1888a29e85e297f128cd9df6ee3a4cabb"),
        }),
    );
    nodes.insert(
        felt!("0x69caa1781f8cd2bd601d17d668e4268a609430ada713cbfd97c4f1a7b79a37e"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x3fb08aa8d7ae250618a5eded665e72102624aca7988f30c1abf93478c918624"),
            right: felt!("0x3781d4c7ddb1f323275a2640bb37924c979225308808987b3b16b7dd7893f1a"),
        }),
    );

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

    let verification_result = verify_starknet_merkle_proof(&nodes, root, path, value);
    assert!(
        verification_result.is_err(),
        "Merkle proof verification should fail"
    );
}

#[test]
fn test_verify_non_membership_proof() -> Result<(), Box<dyn core::error::Error>> {
    let _storage_proof: StorageProof = serde_json::from_reader(std::fs::File::open(
        "tests/fixtures/storage_proof_non_membership.json",
    )?)?;

    Ok(())
}
