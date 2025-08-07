use hermes_core::chain_components::traits::{CanQueryBlock, CanQueryChainStatus};
use hermes_core::test_components::bootstrap::traits::CanBootstrapChain;
use hermes_cosmos::error::types::Error;
use hermes_cosmos::integration_tests::init::init_test_runtime;
use hermes_starknet_chain_components::traits::HasFeederGatewayUrl;
use starknet::core::crypto::{ecdsa_verify, Signature};
use starknet_block_verifier::Endpoint as FeederGatewayEndpoint;
use starknet_crypto_lib::StarknetCryptoLib;
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

        let endpoint_url = chain.feeder_gateway_url();
        let endpoint = FeederGatewayEndpoint::new(endpoint_url.as_str());

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

        let block_header = endpoint.get_block_header(Some(block.height)).unwrap();
        let block_signature = endpoint.get_signature(Some(block.height)).unwrap();
        let public_key = endpoint.get_public_key(Some(block.height)).unwrap();

        assert_eq!(block_header.block_number, block.height);
        assert_eq!(block_header.starknet_version, "0.14.0");
        assert!(block_header
            .verify_signature(&StarknetCryptoLib, &block_signature, &public_key)
            .unwrap());

        Ok(())
    })
}
