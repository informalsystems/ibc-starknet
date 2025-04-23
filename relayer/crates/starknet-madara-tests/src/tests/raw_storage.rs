use cgp::prelude::*;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_error::Error;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::traits::commitment_proof::CanVerifyStarknetMerkleProof;
use hermes_starknet_chain_components::traits::contract::call::CanCallContract;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::traits::contract::invoke::CanInvokeContract;
use hermes_starknet_chain_components::traits::queries::storage_proof::CanQueryStorageProof;
use hermes_starknet_chain_components::traits::types::blob::HasBlobType;
use hermes_starknet_chain_components::traits::types::commitment::{
    HasCommitmentPathType, HasCommitmentValueType, HasMerkleProofType,
};
use hermes_starknet_chain_components::traits::types::method::HasSelectorType;
use hermes_starknet_chain_components::traits::types::storage_proof::{
    HasStorageKeyType, HasStorageProofType,
};
use hermes_starknet_chain_components::types::merkle_proof::StarknetMerkleProof;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use starknet::core::types::{Felt, StorageProof};
use starknet::macros::{felt, selector};
use tracing::info;

use crate::contexts::MadaraChainDriver;
use crate::impls::{init_madara_bootstrap, init_test_runtime};

#[test]
#[ignore]
fn test_madara_raw_storage() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let madara_bootstrap = init_madara_bootstrap(&runtime).await?;

        let chain_driver: MadaraChainDriver = madara_bootstrap.bootstrap_chain("madara").await?;

        let chain = &chain_driver.chain;

        let class_hash = {
            let contract_path = std::env::var("RAW_STORAGE_CONTRACT")?;

            let contract_str: String = runtime.read_file_as_string(&contract_path.into()).await?;

            let contract = serde_json::from_str(&contract_str)?;

            let class_hash = chain.declare_contract(&contract).await?;

            info!("declared class: {:?}", class_hash);

            class_hash
        };

        let contract_address = {
            let contract_address = chain
                .deploy_contract(&class_hash, false, &Vec::new())
                .await?;

            info!(
                "deployed raw storage contract to address: {:?}",
                contract_address
            );

            contract_address
        };

        let key1 = felt!("0x0001");
        let key2 = Felt::ELEMENT_UPPER_BOUND - 1;
        let key3 = felt!("0x100");

        let key1_bits = key1.to_bits_le();
        let key2_bits = key2.to_bits_le();

        let value1 = felt!("0x9911");
        let value2 = felt!("0x9922");
        // let value3 = felt!("0x9933");

        chain
            .bulk_set(&contract_address, &[(key1, value1), (key2, value2)])
            .await?;

        chain
            .verify_merkle_proofs(
                &contract_address,
                &[(key1, value1), (key2, value2), (key3, Felt::ZERO)],
            )
            .await?;

        Ok(())
    })
}

// #[async_trait]
// pub trait CanTestProofEntries: HasAsyncErrorType {
//     async fn test_proof_entries(entries: )
// }

#[async_trait]
pub trait CanVerifyMerkleProofs: HasAddressType + HasAsyncErrorType {
    async fn verify_merkle_proofs(
        &self,
        contract: &Self::Address,
        entries: &[(Felt, Felt)],
    ) -> Result<(), Self::Error>;
}

impl<Chain> CanVerifyMerkleProofs for Chain
where
    Chain: HasAddressType
        + HasStorageKeyType<StorageKey = Felt>
        + HasStorageProofType<StorageProof = StorageProof>
        + HasMerkleProofType<MerkleProof = StarknetMerkleProof>
        + HasCommitmentPathType<CommitmentPath = Felt>
        + HasCommitmentValueType<CommitmentValue = Felt>
        + CanQueryStorageProof
        + CanQueryChainHeight
        + CanVerifyStarknetMerkleProof
        + CanRaiseAsyncError<serde_json::Error>,
{
    async fn verify_merkle_proofs(
        &self,
        contract: &Self::Address,
        entries: &[(Felt, Felt)],
    ) -> Result<(), Self::Error> {
        let height = self.query_chain_height().await?;

        for (key, value) in entries {
            let storage_proof = self.query_storage_proof(&height, contract, &[*key]).await?;

            let storage_proof_str =
                serde_json::to_string_pretty(&storage_proof.contracts_storage_proofs)
                    .map_err(Chain::raise_error)?;

            println!("storage proof for {key}: {storage_proof_str}");

            Chain::verify_starknet_merkle_proof(
                &StarknetMerkleProof {
                    root: storage_proof.contracts_proof.contract_leaves_data[0]
                        .storage_root
                        .unwrap(),
                    proof_nodes: storage_proof.contracts_storage_proofs[0].clone(),
                },
                *key,
                *value,
            )?;
        }

        Ok(())
    }
}

#[async_trait]
pub trait CanBulkSetRawStorage: HasAddressType + HasAsyncErrorType {
    async fn bulk_set(
        &self,
        contract: &Self::Address,
        entries: &[(Felt, Felt)],
    ) -> Result<(), Self::Error>;
}

impl<Chain> CanBulkSetRawStorage for Chain
where
    Chain: CanUseRawStorage,
{
    async fn bulk_set(
        &self,
        contract: &Self::Address,
        entries: &[(Felt, Felt)],
    ) -> Result<(), Self::Error> {
        for (key, value) in entries {
            self.set(contract, *key, *value).await?;
        }

        Ok(())
    }
}

#[async_trait]
pub trait CanUseRawStorage: HasAddressType + HasAsyncErrorType {
    async fn set(
        &self,
        contract: &Self::Address,
        path: Felt,
        value: Felt,
    ) -> Result<(), Self::Error>;

    async fn get(&self, contract: &Self::Address, path: Felt) -> Result<Felt, Self::Error>;
}

impl<Chain> CanUseRawStorage for Chain
where
    Chain: HasAddressType<Address = StarknetAddress>
        + HasSelectorType<Selector = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + CanCallContract
        + CanInvokeContract
        + CanRaiseAsyncError<String>,
{
    async fn get(&self, contract: &StarknetAddress, path: Felt) -> Result<Felt, Self::Error> {
        let result = self
            .call_contract(contract, &selector!("get"), &vec![path], None)
            .await?;

        let [value]: [Felt; 1] = result.try_into().unwrap();

        Ok(value)
    }

    async fn set(
        &self,
        contract: &StarknetAddress,
        path: Felt,
        value: Felt,
    ) -> Result<(), Self::Error> {
        self.invoke_contract(contract, &selector!("set"), &vec![path, value])
            .await?;

        Ok(())
    }
}
