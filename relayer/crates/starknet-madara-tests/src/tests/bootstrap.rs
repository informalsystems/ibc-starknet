use hermes_chain_components::traits::queries::block::CanQueryBlock;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainStatus;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_error::Error;
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

        let starknet_chain = &chain_driver.chain;

        let chain_status = starknet_chain.query_chain_status().await?;

        info!("chain status: {chain_status}");

        let block = starknet_chain.query_block(&chain_status.height).await?;

        info!("block: {block}");

        Ok(())
    })
}
