use cgp::prelude::*;
use hermes_chain_components::traits::queries::block::CanQueryBlock;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainStatus;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_error::Error;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use starknet_v14::core::types::U256;
use tracing::info;

use crate::contexts::MadaraChainDriver;
use crate::impls::init_madara_bootstrap;

#[test]
#[ignore]
fn test_madara_bootstrap() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let madara_bootstrap = init_madara_bootstrap(&runtime).await?;

        let chain_driver: MadaraChainDriver = madara_bootstrap.bootstrap_chain("madara").await?;

        let chain = &chain_driver.chain;

        let chain_status = chain.query_chain_status().await?;

        info!("chain status: {chain_status}");

        let block = chain.query_block(&chain_status.height).await?;

        info!("block: {block}");

        let erc20_class_hash = {
            let contract_path = std::env::var("ERC20_CONTRACT")?;

            let contract_str: String = runtime.read_file_as_string(&contract_path.into()).await?;

            let contract = serde_json::from_str(&contract_str)?;

            let class_hash = chain.declare_contract(&contract).await?;

            info!("declared class: {:?}", class_hash);

            class_hash
        };

        let initial_supply = 1000u32;

        {
            let relayer_address = chain_driver.relayer_wallet.account_address;

            let calldata = StarknetCairoEncoding.encode(&product![
                "token".to_owned(),
                "token".to_owned(),
                U256::from(initial_supply),
                relayer_address,
            ])?;

            let token_address = chain
                .deploy_contract(&erc20_class_hash, false, &calldata)
                .await?;

            info!("deployed ERC20 contract to address: {:?}", token_address);
        };

        Ok(())
    })
}
