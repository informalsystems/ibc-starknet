use std::time::SystemTime;

use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_encoding_components::traits::encoder::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_error::types::Error;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_integration_tests::contexts::bootstrap::StarknetBootstrap;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use starknet::core::types::contract::SierraClass;

#[test]
fn test_starknet_ics20_contract() {
    let runtime = init_test_runtime();

    runtime
        .runtime
        .clone()
        .block_on(async move {
            let chain_command_path = std::env::var("STARKNET_BIN")
                .unwrap_or("starknet-devnet".into())
                .into();

            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs();

            let bootstrap = StarknetBootstrap {
                runtime: runtime.clone(),
                chain_command_path,
                chain_store_dir: format!("./test-data/{timestamp}").into(),
            };

            let chain_driver = bootstrap.bootstrap_chain("starknet").await?;

            let chain = &chain_driver.chain;

            let erc20_class_hash = {
                let contract_path = std::env::var("ERC20_CONTRACT")?;

                let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

                let contract: SierraClass = serde_json::from_str(&contract_str)?;

                let class_hash = chain.declare_contract(&contract).await?;

                println!("declared ERC20 class: {:?}", class_hash);

                class_hash
            };

            let ics20_class_hash = {
                let contract_path = std::env::var("ICS20_CONTRACT")?;

                let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

                let contract: SierraClass = serde_json::from_str(&contract_str)?;

                let class_hash = chain.declare_contract(&contract).await?;

                println!("declared ICS20 class: {:?}", class_hash);

                class_hash
            };

            let _ics20_contract_address = {
                let calldata = chain.encoding().encode(&erc20_class_hash)?;

                let contract_address = chain
                    .deploy_contract(&ics20_class_hash, false, &calldata)
                    .await?;

                println!("deployed ICS20 contract to address: {:?}", contract_address);

                contract_address
            };

            <Result<(), Error>>::Ok(())
        })
        .unwrap();
}
