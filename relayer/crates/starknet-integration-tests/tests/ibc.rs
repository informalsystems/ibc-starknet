use std::time::SystemTime;

use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_encoding_components::traits::encoder::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::send_message::CanSendMessages;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_starknet_chain_components::impls::messages::deploy_erc20::DeployErc20TokenMessage;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::traits::messages::transfer::CanBuildTransferTokenMessage;
use hermes_starknet_chain_components::traits::queries::token_balance::CanQueryTokenBalance;
use hermes_starknet_chain_components::types::amount::StarknetAmount;
use hermes_starknet_integration_tests::contexts::bootstrap::StarknetBootstrap;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use starknet::core::types::contract::SierraClass;

// Note: the test needs to be run with starknet-devnet-rs with the seed 0:
//
// $ starknet-devnet --seed 0
#[test]
fn test_starknet_chain_client() {
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

            {
                // Test deployment of ERC20 contract

                let contract_path = std::env::var("ERC20_CONTRACT")?;

                let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

                let contract: SierraClass = serde_json::from_str(&contract_str)?;

                let class_hash = chain.declare_contract(&contract).await?;

                println!("declared class: {:?}", class_hash);

                let relayer_address = chain_driver.relayer_wallet.account_address;

                let deploy_message = DeployErc20TokenMessage {
                    name: "token".into(),
                    symbol: "token".into(),
                    fixed_supply: 100u32.into(),
                    recipient: relayer_address,
                    owner: relayer_address,
                };

                let calldata = chain.encoding().encode(&deploy_message)?;

                let token_address = chain.deploy_contract(&class_hash, false, &calldata).await?;

                println!("deployed contract to address: {:?}", token_address);
            }

            <Result<(), Error>>::Ok(())
        })
        .unwrap();
}
