use std::sync::Arc;

use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_error::Error;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use starknet_v13::core::types::{BlockId, BlockTag};
use starknet_v13::providers::jsonrpc::HttpTransport;
use starknet_v13::providers::{JsonRpcClient, Provider};
use tracing::info;
use url::Url;

use crate::contexts::MadaraChainDriver;
use crate::impls::init_madara_bootstrap;

#[test]
fn test_madara_bootstrap() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let madara_bootstrap = init_madara_bootstrap(&runtime).await?;

        let chain_driver: MadaraChainDriver = madara_bootstrap.bootstrap_chain("madara").await?;

        let json_rpc_url = Url::parse(&format!(
            "http://{}:{}/",
            chain_driver.node_config.rpc_addr, chain_driver.node_config.rpc_port
        ))?;

        let rpc_client = Arc::new(JsonRpcClient::new(HttpTransport::new(json_rpc_url)));

        let block = rpc_client
            .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
            .await?;

        info!("madara latest block: {block:?}");

        Ok(())
    })
}
