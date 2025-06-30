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

#[test]
fn test_verify_starknet_storage_proof() {
    let global_roots = GlobalRoots {
        contracts_tree_root: felt!(
            "0x368991d64cd97e90a9da1fd9f3d676875d5d29b7136a6ecf77ddc35704f4c27"
        ),
        classes_tree_root: felt!(
            "0x50c234027c744bb8baf77f2229f0433804e8fb9ceb30ad21fde94698832edd1"
        ),
        block_hash: felt!("0x4a538abd420c852e990133d802ecaaa4b4bc2c76371b595a3e837135c63fbb0"),
    };
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

    let contracts_proof = ContractsProof {
        contract_leaves_data: vec![ContractLeafData {
            class_hash: felt!("0x45ba727abaff9ae3a4311d7a30196e09d1f30aeeb3a8e157277793740d20f61"),
            nonce: Felt::ZERO,
            storage_root: Some(felt!(
                "0x4592da9795f9fd7a042eb0cb0d4dae7b6894bd90ccb3e6ff360185db24301f7"
            )),
        }],
        nodes,
    };

    let mut contracts_storage_proof = IndexMap::new();
    contracts_storage_proof.insert(
        felt!("0x4592da9795f9fd7a042eb0cb0d4dae7b6894bd90ccb3e6ff360185db24301f7"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x4a73c78eb32a0614903b17bbc06bd8a7a28ca707981aa36bbe6cd18c2cbb92b"),
            right: felt!("0x4352b8ed6f017cb4cb7084f64af3f1858db935ed9bf734ba5078e774f8a9097"),
        }),
    );
    contracts_storage_proof.insert(
        felt!("0x4a73c78eb32a0614903b17bbc06bd8a7a28ca707981aa36bbe6cd18c2cbb92b"),
        MerkleNode::EdgeNode(EdgeNode {
            path: felt!("0x1"),
            length: 250,
            child: felt!("0x9911"),
        }),
    );

    let storage_proof = StorageProof {
        classes_proof: Default::default(),
        global_roots,
        contracts_proof,
        contracts_storage_proofs: vec![contracts_storage_proof],
    };
    let key: Felt = felt!("0x0001");
    let value: Felt = felt!("0x9911");
    let contract_address: Felt =
        felt!("0x4017d0ad6ddbc7e97208e2639fc5bbf9856b4ede9a66a5995aec87b0d45837c");

    let state_root = felt!("0x2bba45af2d71e57b1f82f1668bc53184762e6212c22e69f9949e3a607022fd2");

    let verification_result = validate_storage_proof(&storage_proof)
        .and_then(|_| verify_starknet_global_contract_root(&storage_proof, state_root))
        .and_then(|global_contract_trie_root| {
            verify_starknet_contract_proof(
                &storage_proof,
                global_contract_trie_root,
                contract_address,
            )
        })
        .and_then(|contract_root| {
            verify_starknet_storage_proof(&storage_proof, contract_root, key, value)
        });

    assert!(
        verification_result.is_ok(),
        "Storage proof verification failed"
    );
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
fn test_verify_starknet_storage_proof_failure() {
    let global_roots = GlobalRoots {
        contracts_tree_root: felt!(
            "0x368991d64cd97e90a9da1fd9f3d676875d5d29b7136a6ecf77ddc35704f4c27"
        ),
        classes_tree_root: felt!(
            "0x50c234027c744bb8baf77f2229f0433804e8fb9ceb30ad21fde94698832edd1"
        ),
        block_hash: felt!("0x4a538abd420c852e990133d802ecaaa4b4bc2c76371b595a3e837135c63fbb0"),
    };
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
    // left value modified to invalid value
    nodes.insert(
        felt!("0x69caa1781f8cd2bd601d17d668e4268a609430ada713cbfd97c4f1a7b79a37e"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0xaaaaaaaad7ae250618a5eded665e72102624aca7988f30c1abf93478c918624"),
            right: felt!("0x3781d4c7ddb1f323275a2640bb37924c979225308808987b3b16b7dd7893f1a"),
        }),
    );

    let contracts_proof = ContractsProof {
        contract_leaves_data: vec![ContractLeafData {
            class_hash: felt!("0x45ba727abaff9ae3a4311d7a30196e09d1f30aeeb3a8e157277793740d20f61"),
            nonce: Felt::ZERO,
            storage_root: Some(felt!(
                "0x4592da9795f9fd7a042eb0cb0d4dae7b6894bd90ccb3e6ff360185db24301f7"
            )),
        }],
        nodes,
    };

    let mut contracts_storage_proof = IndexMap::new();
    contracts_storage_proof.insert(
        felt!("0x4592da9795f9fd7a042eb0cb0d4dae7b6894bd90ccb3e6ff360185db24301f7"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x4a73c78eb32a0614903b17bbc06bd8a7a28ca707981aa36bbe6cd18c2cbb92b"),
            right: felt!("0x4352b8ed6f017cb4cb7084f64af3f1858db935ed9bf734ba5078e774f8a9097"),
        }),
    );
    contracts_storage_proof.insert(
        felt!("0x4a73c78eb32a0614903b17bbc06bd8a7a28ca707981aa36bbe6cd18c2cbb92b"),
        MerkleNode::EdgeNode(EdgeNode {
            path: felt!("0x1"),
            length: 250,
            child: felt!("0x9911"),
        }),
    );

    let storage_proof = StorageProof {
        classes_proof: Default::default(),
        global_roots,
        contracts_proof,
        contracts_storage_proofs: vec![contracts_storage_proof],
    };
    let key: Felt = felt!("0x0001");
    let value: Felt = felt!("0x9911");
    let contract_address: Felt =
        felt!("0x4017d0ad6ddbc7e97208e2639fc5bbf9856b4ede9a66a5995aec87b0d45837c");

    let state_root = felt!("0x2bba45af2d71e57b1f82f1668bc53184762e6212c22e69f9949e3a607022fd2");

    let verification_result = validate_storage_proof(&storage_proof)
        .and_then(|_| verify_starknet_global_contract_root(&storage_proof, state_root))
        .and_then(|global_contract_trie_root| {
            verify_starknet_contract_proof(
                &storage_proof,
                global_contract_trie_root,
                contract_address,
            )
        })
        .and_then(|contract_root| {
            verify_starknet_storage_proof(&storage_proof, contract_root, key, value)
        });

    assert!(
        verification_result.is_err(),
        "Storage proof verification should fail"
    );
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
fn test_verify_non_membership_proof() {
    let mut nodes = IndexMap::new();
    nodes.insert(
        felt!("0x2589ca72a0f01f1ad5ac876238719b1ee94abe8e407867195dfaed7dc41bbc3"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x4fdbb3d94c8a14fd15c4f8357ad687c3f66b9e383e2d449b96156fddc3afa53"),
            right: felt!("0x432184dacf68be5e30df2b1a7947d04659b12ae3995c932cf51da6c96fccfb6"),
        }),
    );
    nodes.insert(
        felt!("0x63c8eced35e747f35b916362eb541fe376cb4818e35e986c2517e6249861f44"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x2589ca72a0f01f1ad5ac876238719b1ee94abe8e407867195dfaed7dc41bbc3"),
            right: felt!("0x26cee3d85f7b6322d53384031cca5afb65a5bbb9f5c37144b54da54eaecb6b9"),
        }),
    );
    nodes.insert(
        felt!("0x6d971efa1db2635c654a280da2a29e39313be22b5b1091eff0daae0254db2ae"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x63c8eced35e747f35b916362eb541fe376cb4818e35e986c2517e6249861f44"),
            right: felt!("0x798b233b7d074f422b5a12e362c00cb2a8982fdeb876f30964047c6eab7478d"),
        }),
    );
    nodes.insert(
        felt!("0x61dcb4883a2e913e8723163340c4126ea556701945e8f57ca6acd085ff4bb0f"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x6d971efa1db2635c654a280da2a29e39313be22b5b1091eff0daae0254db2ae"),
            right: felt!("0x29903970e26f0c3f499e3eb6da2c0b38e61163b2cebd4d38d5bd58af2a441d9"),
        }),
    );
    // Path is non-existent which is for non-membership proofs
    nodes.insert(
        felt!("0x4fdbb3d94c8a14fd15c4f8357ad687c3f66b9e383e2d449b96156fddc3afa53"),
        MerkleNode::EdgeNode(EdgeNode {
            path: felt!("0x39637e05c5b79b90b9be67963e322d4a1b457e8ef6b1ace779578aaae83a65"),
            length: 246,
            child: felt!("0xc5b536c9ec25995e1e97b68b693070f247b4b5fd908152e0a708a7b6a9526e"),
        }),
    );
    nodes.insert(
        felt!("0x3c97b8b422189134b22bb46583b7de17000ace1fe266f41967d2340d0775f75"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x61dcb4883a2e913e8723163340c4126ea556701945e8f57ca6acd085ff4bb0f"),
            right: felt!("0x6fd7bd2b2fc20304e55412daaad68182275615480473332f01e7a4b283ecc93"),
        }),
    );

    let contracts_proof = ContractsProof {
        contract_leaves_data: vec![ContractLeafData {
            class_hash: felt!("0x120e105241f6157aac9149848bca548501d2b66080e71650e11353043a1a61d"),
            nonce: Felt::ZERO,
            storage_root: Some(felt!(
                "0x42db0df05b5d299e7fbc5255f0e20a982530dafd765f62421b66c2763dd0951"
            )),
        }],
        nodes,
    };

    let mut contracts_storage_proof = IndexMap::new();
    contracts_storage_proof.insert(
        felt!("0x6c7cfea149d909e6bdf1fc70f8554926ea150f7648d5fe6cfa62765f7f42a0b"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x8dbd526ba172fe5444415165421685366f6671dc15946ef2449ca3bb901ef4"),
            right: felt!("0x3f3d762322920223ca7471bdc3cd08c5798e27987a62b2a863da9a2f3922acc"),
        }),
    );
    contracts_storage_proof.insert(
        felt!("0x42db0df05b5d299e7fbc5255f0e20a982530dafd765f62421b66c2763dd0951"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x6c7cfea149d909e6bdf1fc70f8554926ea150f7648d5fe6cfa62765f7f42a0b"),
            right: felt!("0x14e50547efee6faf3be6a163b8278921cfa65afc0dc19488fc8e9946f1d8eb"),
        }),
    );
    contracts_storage_proof.insert(
        felt!("0x6a09a0787540b13f6faa7b12858213612d8ccd4d56357b48904300b8144d9b0"),
        MerkleNode::EdgeNode(EdgeNode {
            path: felt!("0xb6ce5410fca59d078ee9b2a4371a9d684c530d697c64fbef0ae6d5e8f0ac72"),
            length: 248,
            child: felt!("0x5"),
        }),
    );
    contracts_storage_proof.insert(
        felt!("0x8dbd526ba172fe5444415165421685366f6671dc15946ef2449ca3bb901ef4"),
        MerkleNode::BinaryNode(BinaryNode {
            left: felt!("0x6a09a0787540b13f6faa7b12858213612d8ccd4d56357b48904300b8144d9b0"),
            right: felt!("0x2f4239acc80e5078cb35ad091ffae15bb4573b227dfaf82e417f04c445e451a"),
        }),
    );

    let global_roots = GlobalRoots {
        contracts_tree_root: felt!(
            "0x3c97b8b422189134b22bb46583b7de17000ace1fe266f41967d2340d0775f75"
        ),
        classes_tree_root: felt!(
            "0x35870e72a1cefa2c1715584a3a5f74f543535b55b06c5c3636cc13e2b6a8b68"
        ),
        block_hash: felt!("0xc047a553ef8aa3297c0c1386fa4ef548251e25b199b13c0f19156225b61ea7"),
    };

    let storage_proof = StorageProof {
        classes_proof: Default::default(),
        global_roots,
        contracts_proof,
        contracts_storage_proofs: vec![contracts_storage_proof],
    };
}
