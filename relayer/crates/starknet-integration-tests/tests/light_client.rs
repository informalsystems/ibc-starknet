#![recursion_limit = "256"]

use std::env::var;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use hermes_cosmos_chain_components::traits::message::ToCosmosMessage;
use hermes_cosmos_chain_components::types::messages::client::create::CosmosCreateClientMessage;
use hermes_cosmos_chain_components::types::messages::client::update::CosmosUpdateClientMessage;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_wasm_relayer::context::cosmos_bootstrap::CosmosWithWasmClientBootstrap;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientEvent;
use hermes_starknet_chain_components::types::client_state::{
    StarknetClientState, WasmStarknetClientState,
};
use hermes_starknet_chain_components::types::consensus_state::{
    StarknetConsensusState, WasmStarknetConsensusState,
};
use hermes_starknet_chain_context::contexts::encoding::protobuf::StarknetProtobufEncoding;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use ibc::clients::wasm_types::client_message::ClientMessage;
use ibc::core::client::types::Height;
use ibc::core::primitives::Timestamp;
use ibc_proto::google::protobuf::Any as IbcAny;
use prost::Message;
use prost_types::Any;
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

        let encoding = StarknetProtobufEncoding;

        let client_id = {
            let client_state = WasmStarknetClientState {
                wasm_code_hash: wasm_code_hash.into(),
                client_state: StarknetClientState {
                    latest_height: Height::new(0, 1)?,
                },
            };

            let consensus_state = WasmStarknetConsensusState {
                consensus_state: StarknetConsensusState {
                    root: vec![1, 2, 3].into(),
                    time: Timestamp::now(),
                }
            };

            let consensus_state_any = encoding.convert(&consensus_state)?;

            let create_client_message = CosmosCreateClientMessage {
                client_state: encoding.convert(&client_state)?,
                consensus_state: consensus_state_any,
            }
            .to_cosmos_message();

            let events = cosmos_chain.send_message(create_client_message).await?;

            println!("create client events: {:?}", events);

            let client_id = events
                .into_iter()
                .find_map(|event| {
                    <CosmosChain as HasCreateClientEvent<CosmosChain>>::try_extract_create_client_event(event)
                })
                .unwrap()
                .client_id;

            println!("created client id: {:?}", client_id);

            client_id
        };

        {
            let consensus_state = StarknetConsensusState {
                root: vec![4, 5, 6].into(),
                time: Timestamp::now(),
            };

            let consensus_state_any: Any = encoding.convert(&consensus_state)?;

            let wasm_client_header = ClientMessage {
                data: consensus_state_any.encode_to_vec(),
            };

            let update_client_message = CosmosUpdateClientMessage {
                client_id,
                header: IbcAny {
                    type_url: consensus_state_any.type_url,
                    value: consensus_state_any.value,
                },
            }
            .to_cosmos_message();

            let events = cosmos_chain.send_message(update_client_message).await?;

            println!("update client events: {:?}", events);
        }

        Ok(())
    })
}
