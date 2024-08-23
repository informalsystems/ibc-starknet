#![recursion_limit = "256"]

use std::env::var;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use hermes_cosmos_chain_components::traits::message::ToCosmosMessage;
use hermes_cosmos_chain_components::types::messages::client::create::CosmosCreateClientMessage;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_wasm_relayer::context::cosmos_bootstrap::CosmosWithWasmClientBootstrap;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_starknet_chain_components::types::client_state::{
    StarknetClientState, WasmStarknetClientState,
};
use hermes_starknet_chain_components::types::consensus_state::StarknetConsensusState;
use hermes_starknet_chain_context::contexts::protobuf_encoding::StarknetProtobufEncoding;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use ibc::core::client::types::Height;
use sha2::{Digest, Sha256};

#[test]
fn test_starknet_light_client() -> Result<(), Error> {
    let runtime = init_test_runtime();

    let store_postfix = format!(
        "{}-{}",
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),
        rand::random::<u64>()
    );

    let store_dir = std::env::current_dir()?.join(format!("test-data/{store_postfix}"));

    let wasm_client_code_path = PathBuf::from(
        var("STARKNET_WASM_CLIENT_PATH").expect("Wasm blob for Starknet light client is required"),
    );

    let cosmos_builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    runtime.runtime.clone().block_on(async move {
        let wasm_client_byte_code = tokio::fs::read(&wasm_client_code_path).await?;

        let wasm_code_hash: [u8; 32] = {
            let mut hasher = Sha256::new();
            hasher.update(&wasm_client_byte_code);
            hasher.finalize().into()
        };

        println!("Will Wasm Starknet client with code hash: {:?}", BASE64_STANDARD.encode(wasm_code_hash));

        let cosmos_bootstrap = Arc::new(CosmosWithWasmClientBootstrap {
            runtime: runtime.clone(),
            builder: cosmos_builder,
            should_randomize_identifiers: true,
            chain_store_dir: store_dir.join("chains"),
            chain_command_path: "simd".into(),
            account_prefix: "cosmos".into(),
            staking_denom: "stake".into(),
            transfer_denom: "coin".into(),
            wasm_client_byte_code,
            governance_proposal_authority: "cosmos10d07y265gmmuvt4z0w9aw880jnsr700j6zn9kn".into(), // TODO: don't hard code this
        });

        let cosmos_chain_driver = cosmos_bootstrap.bootstrap_chain("cosmos").await?;

        let cosmos_chain = cosmos_chain_driver.chain();

        let client_state = WasmStarknetClientState {
            wasm_code_hash: wasm_code_hash.into(),
            client_state: StarknetClientState {
                latest_height: Height::new(0, 1)?,
            },
        };

        let consensus_state = StarknetConsensusState {
            root: vec![1, 2, 3].into(),
        };

        let encoding = StarknetProtobufEncoding;

        let create_client_message = CosmosCreateClientMessage {
            client_state: encoding.convert(&client_state)?,
            consensus_state: encoding.convert(&consensus_state)?,
        }
        .to_cosmos_message();

        cosmos_chain.send_message(create_client_message).await?;

        Ok(())
    })
}
