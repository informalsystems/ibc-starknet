use hermes_core::chain_components::traits::{CanQueryBlock, CanQueryChainStatus};
use hermes_core::test_components::bootstrap::traits::CanBootstrapChain;
use hermes_cosmos::error::types::Error;
use hermes_cosmos::integration_tests::init::init_test_runtime;
use starknet::core::crypto::{ecdsa_verify, Signature};
use starknet_block_verifier::Endpoint;
use tracing::info;

use crate::contexts::StarknetChainDriver;
use crate::utils::init_starknet_bootstrap;

#[test]
fn test_starknet_feeder_gateway_signature() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let starknet_bootstrap = init_starknet_bootstrap(&runtime).await?;

        let chain_driver: StarknetChainDriver =
            starknet_bootstrap.bootstrap_chain("starknet").await?;

        let chain = &chain_driver.chain;

        let chain_status = chain.query_chain_status().await?;

        info!("chain status: {chain_status}");

        let block = chain.query_block(&chain_status.height).await?;

        info!("block: {block}");

        let gateway_port = chain_driver.node_config.rpc_port + 1;

        // starknet feeder gateway endpoint
        let endpoint = Endpoint::new(&format!("http://0.0.0.0:{gateway_port}"));

        let public_key = endpoint.get_public_key(Some(block.height)).unwrap();

        info!("public key: {public_key:x}");

        let signature = endpoint.get_signature(Some(block.height)).unwrap();

        info!("signature: {signature:x?}");

        assert!(ecdsa_verify(
            &public_key,
            &signature.block_hash,
            &Signature {
                r: signature.signature[0],
                s: signature.signature[1],
            },
        )
        .unwrap());

        // can't call `get_block` yet as we are using `0.13.5` block header
        // and, starknet uses `0.13.2` block header

        // let block_header = endpoint.get_block_header(Some(block.height)).unwrap();
        // info!("block_header: {block_header:?}");
        // assert_eq!(block_header.block_number, block.height);
        // assert_eq!(block_header.starknet_version, "0.13.2");

        Ok(())
    })
}
