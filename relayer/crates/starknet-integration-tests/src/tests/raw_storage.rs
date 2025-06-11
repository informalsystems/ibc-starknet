use core::fmt::Display;

use hermes_core::chain_components::traits::CanQueryChainHeight;
use hermes_core::chain_type_components::traits::HasAddressType;
use hermes_core::runtime_components::traits::CanReadFileAsString;
use hermes_core::test_components::bootstrap::traits::CanBootstrapChain;
use hermes_cosmos::error::types::Error;
use hermes_cosmos::integration_tests::init::init_test_runtime;
use hermes_prelude::*;
use hermes_starknet_chain_components::impls::StarknetAddress;
use hermes_starknet_chain_components::traits::{
    CanCallContract, CanDeclareContract, CanDeployContract, CanInvokeContract,
    CanQueryStorageProof, CanVerifyStarknetStorageProof, HasBlobType, HasContractClassHashType,
    HasSelectorType, HasStorageKeyType, HasStorageProofType,
};
use starknet::core::types::Felt;
use starknet::macros::{felt, selector};
use starknet_v14::core::types::StorageProof;
use tracing::info;

use crate::contexts::StarknetChainDriver;
use crate::utils::init_starknet_bootstrap;

#[test]
fn test_starknet_raw_storage() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let starknet_bootstrap = init_starknet_bootstrap(&runtime).await?;

        let chain_driver: StarknetChainDriver =
            starknet_bootstrap.bootstrap_chain("starknet").await?;

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
    Chain: CanDeployContract<Blob = Vec<Felt>> + CanUseRawStorage + CanVerifyMerkleProofs,
    Chain::Address: Display,
{
    async fn test_proof_entries(
        &self,
        class_hash: &Self::ContractClassHash,
        entries: &[(Felt, Felt)],
    ) -> Result<(), Self::Error> {
        let contract_address = self.deploy_contract(class_hash, false, &Vec::new()).await?;

        info!(
            "deployed raw storage contract to address: {}",
            contract_address
        );

        self.set(&contract_address, entries).await?;

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
pub trait CanUseRawStorage: HasAddressType + HasAsyncErrorType {
    async fn set(
        &self,
        contract: &Self::Address,
        entries: &[(Felt, Felt)],
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
        entries: &[(Felt, Felt)],
    ) -> Result<(), Self::Error> {
        let length = Felt::from(entries.len());
        let mut calldata = vec![length];

        for (key, value) in entries {
            calldata.push(*key);
            calldata.push(*value);
        }

        self.invoke_contract(contract, &selector!("set"), &calldata)
            .await?;

        Ok(())
    }
}
