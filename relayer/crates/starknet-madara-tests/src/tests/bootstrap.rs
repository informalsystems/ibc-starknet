use hermes_chain_components::traits::queries::block::CanQueryBlock;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainStatus;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_error::Error;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
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

        Ok(())
    })
}
