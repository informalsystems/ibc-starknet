use hermes_core::chain_components::traits::{CanQueryBlock, CanQueryChainStatus};
use hermes_core::test_components::bootstrap::traits::CanBootstrapChain;
use hermes_error::Error;
use starknet::core::crypto::{ecdsa_verify, Signature};
use starknet_block_verifier::Endpoint;
use tracing::info;

use crate::contexts::MadaraChainDriver;
use crate::impls::{init_madara_bootstrap, init_test_runtime};

#[test]
fn test_madara_feeder_gateway_signature() -> Result<(), Error> {
    let runtime = init_test_runtime();

    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let madara_bootstrap = init_madara_bootstrap(&runtime).await?;

        let chain_driver: MadaraChainDriver = madara_bootstrap.bootstrap_chain("madara").await?;

        let chain = &chain_driver.chain;

        let chain_status = chain.query_chain_status().await?;

        info!("chain status: {chain_status}");

        let block = chain.query_block(&chain_status.height).await?;

        info!("block: {block}");

        let gateway_port = chain_driver.node_config.rpc_port + 1;

        // madara feeder gateway endpoint
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

        Ok(())
    })
}
