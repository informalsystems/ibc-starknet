use indexmap::IndexMap;
use starknet::core::crypto::pedersen_hash;
use starknet::core::types::{
    BinaryNode, ContractLeafData, ContractsProof, EdgeNode, Felt, GlobalRoots, MerkleNode,
    StorageProof,
};
use starknet_storage_verifier::verifier::{
    verify_starknet_merkle_proof, verify_starknet_storage_proof,
};

#[test]
fn test_verify_starknet_storage_proof() {
    let global_roots = GlobalRoots {
        contracts_tree_root: Felt::from_hex(
            "0x368991d64cd97e90a9da1fd9f3d676875d5d29b7136a6ecf77ddc35704f4c27",
        )
        .unwrap(),
        classes_tree_root: Felt::from_hex(
            "0x50c234027c744bb8baf77f2229f0433804e8fb9ceb30ad21fde94698832edd1",
        )
        .unwrap(),
        block_hash: Felt::from_hex(
            "0x4a538abd420c852e990133d802ecaaa4b4bc2c76371b595a3e837135c63fbb0",
        )
        .unwrap(),
    };
    let mut nodes = IndexMap::new();
    nodes.insert(
        Felt::from_hex("0x2ed240ee95949cd56bb6d1feb5974abd931b844fd090f6738c1166ab129f256")
            .unwrap(),
        MerkleNode::EdgeNode(EdgeNode {
            path: Felt::from_hex("0x0").unwrap(),
            length: 1,
            child: Felt::from_hex(
                "0x69caa1781f8cd2bd601d17d668e4268a609430ada713cbfd97c4f1a7b79a37e",
            )
            .unwrap(),
        }),
    );
    nodes.insert(
        Felt::from_hex("0x3fb08aa8d7ae250618a5eded665e72102624aca7988f30c1abf93478c918624")
            .unwrap(),
        MerkleNode::EdgeNode(EdgeNode {
            path: Felt::from_hex("0x17d0ad6ddbc7e97208e2639fc5bbf9856b4ede9a66a5995aec87b0d45837c")
                .unwrap(),
            length: 244,
            child: Felt::from_hex(
                "0x5cea18c46bb31c421e40f94efd51a83ec4882ae72473bb41edd2652137b938e",
            )
            .unwrap(),
        }),
    );
    nodes.insert(
        Felt::from_hex("0x368991d64cd97e90a9da1fd9f3d676875d5d29b7136a6ecf77ddc35704f4c27")
            .unwrap(),
        MerkleNode::BinaryNode(BinaryNode {
            left: Felt::from_hex(
                "0x136287afa5c7e9d96deba14d1080672eef35240cc4940076d378e84ef6b7c26",
            )
            .unwrap(),
            right: Felt::from_hex(
                "0xbe9a5b8558021942adf479733d2e345c5147b18d6c0aee40e5ee9fb85bc32",
            )
            .unwrap(),
        }),
    );
    nodes.insert(
        Felt::from_hex("0xe1081c844581366e4e04cd3523b382a40865746a8cd035e41c9806327852a0").unwrap(),
        MerkleNode::BinaryNode(BinaryNode {
            left: Felt::from_hex(
                "0x5dc3360ed7d92cfc8acfc3d5eb35ce9402a2e6e0754ed82487f78630ee63e1d",
            )
            .unwrap(),
            right: Felt::from_hex(
                "0x54cb87a02896d50c9822961eb21c41326a3730e03beebb4273e9f53ecb1aa10",
            )
            .unwrap(),
        }),
    );
    nodes.insert(
        Felt::from_hex("0x3d72d82b1e3e3eeda9fe96221dc8a2131f22e243382d0f16883568f927589ef")
            .unwrap(),
        MerkleNode::BinaryNode(BinaryNode {
            left: Felt::from_hex(
                "0xe1081c844581366e4e04cd3523b382a40865746a8cd035e41c9806327852a0",
            )
            .unwrap(),
            right: Felt::from_hex(
                "0x7f5c3d3f9a27468b74d8d9352d6104a12e51257bb9bd2b775bcef738eba3d88",
            )
            .unwrap(),
        }),
    );
    nodes.insert(
        Felt::from_hex("0xbe9a5b8558021942adf479733d2e345c5147b18d6c0aee40e5ee9fb85bc32").unwrap(),
        MerkleNode::BinaryNode(BinaryNode {
            left: Felt::from_hex(
                "0x3d72d82b1e3e3eeda9fe96221dc8a2131f22e243382d0f16883568f927589ef",
            )
            .unwrap(),
            right: Felt::from_hex(
                "0x50b7568824156bac7669bda45c4db7c579f6b799bd3fd3142332ecd083cceb6",
            )
            .unwrap(),
        }),
    );
    nodes.insert(
        Felt::from_hex("0x5dc3360ed7d92cfc8acfc3d5eb35ce9402a2e6e0754ed82487f78630ee63e1d")
            .unwrap(),
        MerkleNode::BinaryNode(BinaryNode {
            left: Felt::from_hex(
                "0x2ed240ee95949cd56bb6d1feb5974abd931b844fd090f6738c1166ab129f256",
            )
            .unwrap(),
            right: Felt::from_hex(
                "0x4e50fb911906759c79313749ede30a1888a29e85e297f128cd9df6ee3a4cabb",
            )
            .unwrap(),
        }),
    );
    nodes.insert(
        Felt::from_hex("0x69caa1781f8cd2bd601d17d668e4268a609430ada713cbfd97c4f1a7b79a37e")
            .unwrap(),
        MerkleNode::BinaryNode(BinaryNode {
            left: Felt::from_hex(
                "0x3fb08aa8d7ae250618a5eded665e72102624aca7988f30c1abf93478c918624",
            )
            .unwrap(),
            right: Felt::from_hex(
                "0x3781d4c7ddb1f323275a2640bb37924c979225308808987b3b16b7dd7893f1a",
            )
            .unwrap(),
        }),
    );

    let contracts_proof = ContractsProof {
        contract_leaves_data: vec![ContractLeafData {
            class_hash: Felt::from_hex(
                "0x45ba727abaff9ae3a4311d7a30196e09d1f30aeeb3a8e157277793740d20f61",
            )
            .unwrap(),
            nonce: Felt::from_hex("0x0").unwrap(),
            storage_root: Some(
                Felt::from_hex("0x4592da9795f9fd7a042eb0cb0d4dae7b6894bd90ccb3e6ff360185db24301f7")
                    .unwrap(),
            ),
        }],
        nodes,
    };

    let mut contracts_storage_proof = IndexMap::new();
    contracts_storage_proof.insert(
        Felt::from_hex("0x4592da9795f9fd7a042eb0cb0d4dae7b6894bd90ccb3e6ff360185db24301f7")
            .unwrap(),
        MerkleNode::BinaryNode(BinaryNode {
            left: Felt::from_hex(
                "0x4a73c78eb32a0614903b17bbc06bd8a7a28ca707981aa36bbe6cd18c2cbb92b",
            )
            .unwrap(),
            right: Felt::from_hex(
                "0x4352b8ed6f017cb4cb7084f64af3f1858db935ed9bf734ba5078e774f8a9097",
            )
            .unwrap(),
        }),
    );
    contracts_storage_proof.insert(
        Felt::from_hex("0x4a73c78eb32a0614903b17bbc06bd8a7a28ca707981aa36bbe6cd18c2cbb92b")
            .unwrap(),
        MerkleNode::EdgeNode(EdgeNode {
            path: Felt::from_hex("0x1").unwrap(),
            length: 250,
            child: Felt::from_hex("0x9911").unwrap(),
        }),
    );

    let storage_proof = StorageProof {
        classes_proof: Default::default(),
        global_roots,
        contracts_proof,
        contracts_storage_proofs: vec![contracts_storage_proof],
    };
    let key: Felt = Felt::from_hex("0x0001").unwrap();
    let value: Felt = Felt::from_hex("0x9911").unwrap();
    let contract_address: Felt =
        Felt::from_hex("0x4017d0ad6ddbc7e97208e2639fc5bbf9856b4ede9a66a5995aec87b0d45837c")
            .unwrap();

    let verification_result =
        verify_starknet_storage_proof(&storage_proof, &contract_address, key, value);

    assert!(
        verification_result.is_ok(),
        "Storage proof verification failed"
    );
}

#[test]
fn test_verify_starknet_merkle_proof() {
    let mut nodes = IndexMap::new();

    nodes.insert(
        Felt::from_hex("0x2ed240ee95949cd56bb6d1feb5974abd931b844fd090f6738c1166ab129f256")
            .unwrap(),
        MerkleNode::EdgeNode(EdgeNode {
            path: Felt::from_hex("0x0").unwrap(),
            length: 1,
            child: Felt::from_hex(
                "0x69caa1781f8cd2bd601d17d668e4268a609430ada713cbfd97c4f1a7b79a37e",
            )
            .unwrap(),
        }),
    );
    nodes.insert(
        Felt::from_hex("0x3fb08aa8d7ae250618a5eded665e72102624aca7988f30c1abf93478c918624")
            .unwrap(),
        MerkleNode::EdgeNode(EdgeNode {
            path: Felt::from_hex("0x17d0ad6ddbc7e97208e2639fc5bbf9856b4ede9a66a5995aec87b0d45837c")
                .unwrap(),
            length: 244,
            child: Felt::from_hex(
                "0x5cea18c46bb31c421e40f94efd51a83ec4882ae72473bb41edd2652137b938e",
            )
            .unwrap(),
        }),
    );
    nodes.insert(
        Felt::from_hex("0x368991d64cd97e90a9da1fd9f3d676875d5d29b7136a6ecf77ddc35704f4c27")
            .unwrap(),
        MerkleNode::BinaryNode(BinaryNode {
            left: Felt::from_hex(
                "0x136287afa5c7e9d96deba14d1080672eef35240cc4940076d378e84ef6b7c26",
            )
            .unwrap(),
            right: Felt::from_hex(
                "0xbe9a5b8558021942adf479733d2e345c5147b18d6c0aee40e5ee9fb85bc32",
            )
            .unwrap(),
        }),
    );
    nodes.insert(
        Felt::from_hex("0xe1081c844581366e4e04cd3523b382a40865746a8cd035e41c9806327852a0").unwrap(),
        MerkleNode::BinaryNode(BinaryNode {
            left: Felt::from_hex(
                "0x5dc3360ed7d92cfc8acfc3d5eb35ce9402a2e6e0754ed82487f78630ee63e1d",
            )
            .unwrap(),
            right: Felt::from_hex(
                "0x54cb87a02896d50c9822961eb21c41326a3730e03beebb4273e9f53ecb1aa10",
            )
            .unwrap(),
        }),
    );
    nodes.insert(
        Felt::from_hex("0x3d72d82b1e3e3eeda9fe96221dc8a2131f22e243382d0f16883568f927589ef")
            .unwrap(),
        MerkleNode::BinaryNode(BinaryNode {
            left: Felt::from_hex(
                "0xe1081c844581366e4e04cd3523b382a40865746a8cd035e41c9806327852a0",
            )
            .unwrap(),
            right: Felt::from_hex(
                "0x7f5c3d3f9a27468b74d8d9352d6104a12e51257bb9bd2b775bcef738eba3d88",
            )
            .unwrap(),
        }),
    );
    nodes.insert(
        Felt::from_hex("0xbe9a5b8558021942adf479733d2e345c5147b18d6c0aee40e5ee9fb85bc32").unwrap(),
        MerkleNode::BinaryNode(BinaryNode {
            left: Felt::from_hex(
                "0x3d72d82b1e3e3eeda9fe96221dc8a2131f22e243382d0f16883568f927589ef",
            )
            .unwrap(),
            right: Felt::from_hex(
                "0x50b7568824156bac7669bda45c4db7c579f6b799bd3fd3142332ecd083cceb6",
            )
            .unwrap(),
        }),
    );
    nodes.insert(
        Felt::from_hex("0x5dc3360ed7d92cfc8acfc3d5eb35ce9402a2e6e0754ed82487f78630ee63e1d")
            .unwrap(),
        MerkleNode::BinaryNode(BinaryNode {
            left: Felt::from_hex(
                "0x2ed240ee95949cd56bb6d1feb5974abd931b844fd090f6738c1166ab129f256",
            )
            .unwrap(),
            right: Felt::from_hex(
                "0x4e50fb911906759c79313749ede30a1888a29e85e297f128cd9df6ee3a4cabb",
            )
            .unwrap(),
        }),
    );
    nodes.insert(
        Felt::from_hex("0x69caa1781f8cd2bd601d17d668e4268a609430ada713cbfd97c4f1a7b79a37e")
            .unwrap(),
        MerkleNode::BinaryNode(BinaryNode {
            left: Felt::from_hex(
                "0x3fb08aa8d7ae250618a5eded665e72102624aca7988f30c1abf93478c918624",
            )
            .unwrap(),
            right: Felt::from_hex(
                "0x3781d4c7ddb1f323275a2640bb37924c979225308808987b3b16b7dd7893f1a",
            )
            .unwrap(),
        }),
    );

    let root = Felt::from_hex("0x368991d64cd97e90a9da1fd9f3d676875d5d29b7136a6ecf77ddc35704f4c27")
        .unwrap();
    let path = Felt::from_hex("0x4017d0ad6ddbc7e97208e2639fc5bbf9856b4ede9a66a5995aec87b0d45837c")
        .unwrap();
    let value = pedersen_hash(
        &pedersen_hash(
            &pedersen_hash(
                &Felt::from_hex(
                    "0x45ba727abaff9ae3a4311d7a30196e09d1f30aeeb3a8e157277793740d20f61",
                )
                .unwrap(),
                &Felt::from_hex(
                    "0x4592da9795f9fd7a042eb0cb0d4dae7b6894bd90ccb3e6ff360185db24301f7",
                )
                .unwrap(),
            ),
            &Felt::from_hex("0x0").unwrap(),
        ),
        &Felt::ZERO,
    );

    let verification_result = verify_starknet_merkle_proof(&nodes, root, path, value);
    assert!(
        verification_result.is_ok(),
        "Merkle proof verification failed"
    );
}
