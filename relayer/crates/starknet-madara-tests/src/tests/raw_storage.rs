use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_error::Error;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::traits::contract::call::CanCallContract;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::traits::contract::invoke::CanInvokeContract;
use hermes_starknet_chain_components::traits::types::blob::HasBlobType;
use hermes_starknet_chain_components::traits::types::method::HasSelectorType;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use starknet::core::types::Felt;
use starknet::macros::{felt, selector};
use tracing::info;

use crate::contexts::MadaraChainDriver;
use crate::impls::init_madara_bootstrap;

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

        let key = felt!("0x123");
        let value = felt!("0x456");

        chain.set(&contract_address, key, value).await?;

        let value2 = chain.get(&contract_address, key).await?;

        assert_eq!(value2, value);

        Ok(())
    })
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
