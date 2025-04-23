use core::fmt::Display;

use cgp::prelude::*;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_error::Error;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::traits::commitment_proof::CanVerifyStarknetStorageProof;
use hermes_starknet_chain_components::traits::contract::call::CanCallContract;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::traits::contract::invoke::CanInvokeContract;
use hermes_starknet_chain_components::traits::queries::storage_proof::CanQueryStorageProof;
use hermes_starknet_chain_components::traits::types::blob::HasBlobType;
use hermes_starknet_chain_components::traits::types::contract_class::HasContractClassHashType;
use hermes_starknet_chain_components::traits::types::method::HasSelectorType;
use hermes_starknet_chain_components::traits::types::storage_proof::{
    HasStorageKeyType, HasStorageProofType,
};
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

        chain
            .test_proof_entries(
                &class_hash,
                &[
                    (felt!("0x0001"), felt!("0x9911")),
                    (Felt::ELEMENT_UPPER_BOUND - 1, felt!("0x9922")),
                    (felt!("0x100"), Felt::ZERO),
                ],
            )
            .await?;

        chain
            .test_proof_entries(
                &class_hash,
                &[
                    (felt!("0x0001"), felt!("0x9991")),
                    (felt!("0x0002"), felt!("0x9992")),
                    (felt!("0x0003"), Felt::ZERO),
                    (felt!("0x0004"), Felt::ZERO),
                    (felt!("0x0005"), Felt::ZERO),
                    (felt!("0x0006"), Felt::ZERO),
                    (felt!("0x0007"), felt!("0x9997")),
                    (felt!("0x0008"), felt!("0x9998")),
                ],
            )
            .await?;

        Ok(())
    })
}

#[async_trait]
pub trait CanTestProofEntries: HasContractClassHashType + HasAsyncErrorType {
    async fn test_proof_entries(
        &self,
        class_hash: &Self::ContractClassHash,
        entries: &[(Felt, Felt)],
    ) -> Result<(), Self::Error>;
}

impl<Chain> CanTestProofEntries for Chain
where
    Chain: CanDeployContract<Blob = Vec<Felt>> + CanBulkSetRawStorage + CanVerifyMerkleProofs,
    Chain::Address: Display,
{
    async fn test_proof_entries(
        &self,
        class_hash: &Self::ContractClassHash,
        entries: &[(Felt, Felt)],
    ) -> Result<(), Self::Error> {
        let contract_address = self
            .deploy_contract(&class_hash, false, &Vec::new())
            .await?;

        info!(
            "deployed raw storage contract to address: {}",
            contract_address
        );

        self.bulk_set(&contract_address, entries).await?;

        self.verify_merkle_proofs(&contract_address, entries)
            .await?;

        Ok(())
    }
}

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
    Chain: HasAddressType<Address = StarknetAddress>
        + HasStorageKeyType<StorageKey = Felt>
        + HasStorageProofType<StorageProof = StorageProof>
        + CanQueryStorageProof
        + CanQueryChainHeight
        + CanVerifyStarknetStorageProof
        + CanRaiseAsyncError<serde_json::Error>,
{
    async fn verify_merkle_proofs(
        &self,
        contract_address: &StarknetAddress,
        entries: &[(Felt, Felt)],
    ) -> Result<(), Self::Error> {
        let height = self.query_chain_height().await?;

        for (key, value) in entries {
            let storage_proof = self
                .query_storage_proof(&height, contract_address, &[*key])
                .await?;

            let storage_proof_str =
                serde_json::to_string_pretty(&storage_proof).map_err(Chain::raise_error)?;

            println!("storage proof for {key}: {storage_proof_str}");

            Chain::verify_starknet_storage_proof(&storage_proof, contract_address, *key, *value)?;
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
