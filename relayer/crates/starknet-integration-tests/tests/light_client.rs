#![recursion_limit = "256"]

use std::env::var;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use hermes_cosmos_chain_components::traits::message::ToCosmosMessage;
use hermes_cosmos_chain_components::types::messages::channel::open_ack::CosmosChannelOpenAckMessage;
use hermes_cosmos_chain_components::types::messages::channel::open_init::CosmosChannelOpenInitMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_ack::CosmosConnectionOpenAckMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_init::CosmosConnectionOpenInitMessage;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::contexts::encoding::CosmosEncoding;
use hermes_cosmos_wasm_relayer::context::cosmos_bootstrap::CosmosWithWasmClientBootstrap;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CanBuildCreateClientPayload;
use hermes_relayer_components::chain::traits::queries::chain_status::{
    CanQueryChainHeight, CanQueryChainStatus,
};
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::queries::consensus_state::CanQueryConsensusState;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_relayer_components::chain::traits::types::ibc_events::channel::HasChannelOpenInitEvent;
use hermes_relayer_components::chain::traits::types::ibc_events::connection::HasConnectionOpenInitEvent;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::DestinationTarget;
use hermes_relayer_components::relay::traits::update_client_message_builder::CanSendTargetUpdateClientMessage;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_integration_tests::contexts::bootstrap::StarknetBootstrap;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use ibc::core::channel::types::channel::State;
use ibc::core::connection::types::version::Version;
use ibc_proto::ibc::core::channel::v1::{Channel, Counterparty};
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer::config::types::TrustThreshold;
use ibc_relayer_types::core::ics04_channel::channel::Ordering;
use ibc_relayer_types::Height;
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

        let starknet_bootstrap = StarknetBootstrap {
            runtime: runtime.clone(),
            chain_command_path: "starknet-devnet".into(),
            chain_store_dir: store_dir,
        };

        let cosmos_chain_driver = cosmos_bootstrap.bootstrap_chain("cosmos").await?;

        let starknet_chain_driver = starknet_bootstrap.bootstrap_chain("starknet").await?;

        let cosmos_chain = cosmos_chain_driver.chain();

        let starknet_chain = &starknet_chain_driver.chain;

        let client_id = StarknetToCosmosRelay::create_client(
            DestinationTarget,
            cosmos_chain,
            starknet_chain,
            &StarknetCreateClientPayloadOptions { wasm_code_hash },
            &(),
        )
        .await?;

        println!("created client id: {:?}", client_id);

        let starknet_to_cosmos_relay = StarknetToCosmosRelay {
            runtime: runtime.clone(),
            src_chain: starknet_chain.clone(),
            dst_chain: cosmos_chain.clone(),
            src_client_id: client_id.clone(), // TODO: stub
            dst_client_id: client_id.clone(),
        };

        {
            let client_state =
                <CosmosChain as CanQueryClientState<StarknetChain>>::query_client_state(
                    cosmos_chain,
                    &client_id,
                    &cosmos_chain.query_chain_height().await?,
                )
                .await?;

            let client_height = client_state.client_state.latest_height.revision_height();

            let consensus_state =
                <CosmosChain as CanQueryConsensusState<StarknetChain>>::query_consensus_state(
                    cosmos_chain,
                    &client_id,
                    &client_height,
                    &cosmos_chain.query_chain_height().await?,
                )
                .await?;

            println!(
                "initial consensus state height {} and root: {:?}",
                client_height,
                consensus_state.consensus_state.root.into_vec()
            );
        }

        {
            runtime.sleep(Duration::from_secs(2)).await;

            let starknet_status = starknet_chain.query_chain_status().await?;

            println!(
                "updating Starknet client to Cosmos to height {} and root: {:?}",
                starknet_status.height,
                starknet_status.block_hash.to_bytes_be()
            );

            starknet_to_cosmos_relay
                .send_target_update_client_messages(DestinationTarget, &starknet_status.height)
                .await?;

            let consensus_state =
                <CosmosChain as CanQueryConsensusState<StarknetChain>>::query_consensus_state(
                    cosmos_chain,
                    &client_id,
                    &starknet_status.height,
                    &cosmos_chain.query_chain_height().await?,
                )
                .await?;

            assert_eq!(
                consensus_state.consensus_state.root.into_vec(),
                starknet_status.block_hash.to_bytes_be()
            );
        }

        let connection_id = {
            let open_init_message = CosmosConnectionOpenInitMessage {
                client_id: client_id.clone(),
                counterparty_client_id: client_id.clone(), // TODO: stub
                counterparty_commitment_prefix: "ibc".into(),
                version: Version::compatibles().pop().unwrap(),
                delay_period: Duration::from_secs(0),
            };

            let events = cosmos_chain.send_message(open_init_message.to_cosmos_message()).await?;

            let connection_id = events
                .into_iter()
                .find_map(<CosmosChain as HasConnectionOpenInitEvent<StarknetChain>>::try_extract_connection_open_init_event)
                .unwrap()
                .connection_id
            ;

            println!("initialized connection on Cosmos: {connection_id}");

            connection_id
        };

        {
            // Pretend that we have relayed ConnectionOpenTry to Starknet, and then send ConnectionOpenAck.

            let create_client_settings = Settings {
                max_clock_drift: Duration::from_secs(40),
                trusting_period: None,
                trust_threshold: TrustThreshold::ONE_THIRD,
            };

            let payload = <CosmosChain as CanBuildCreateClientPayload<CosmosChain>>::build_create_client_payload(cosmos_chain, &create_client_settings,
            ).await?;

            let client_state = CosmosEncoding.convert(&payload.client_state)?;

            runtime.sleep(Duration::from_secs(1)).await;

            let open_ack_message = CosmosConnectionOpenAckMessage {
                connection_id: connection_id.clone(),
                counterparty_connection_id: connection_id.clone(), // TODO: stub
                version: Version::compatibles().pop().unwrap(),
                client_state,
                update_height: Height::new(0, 1).unwrap(),
                proof_try: [0; 32].into(), // dummy proofs
                proof_client: [0; 32].into(),
                proof_consensus: [0; 32].into(),
                proof_consensus_height: payload.client_state.latest_height,
            };

            cosmos_chain.send_message(open_ack_message.to_cosmos_message()).await?;
        }

        let channel_id = {
            let channel = Channel {
                state: State::Init as i32,
                ordering: Ordering::Unordered as i32,
                counterparty: Some(Counterparty {
                    port_id: "11b7f9bfa43d3facae74efa5dfe0030df98273271278291d67c16a4e6cd5f7c".to_string(), // stub application contract on Starknet as port ID
                    channel_id: "".to_string(),
                }),
                connection_hops: vec![connection_id.to_string()],
                version: "ics20-1".into(),
                upgrade_sequence: 0,
            };

            let open_init_message = CosmosChannelOpenInitMessage {
                port_id: "transfer".into(),
                channel,
            };

            let events = cosmos_chain.send_message(open_init_message.to_cosmos_message()).await?;


            let channel_id = events
                .into_iter()
                .find_map(<CosmosChain as HasChannelOpenInitEvent<StarknetChain>>::try_extract_channel_open_init_event)
                .unwrap()
                .channel_id
            ;

            println!("initialized channel on Cosmos: {channel_id}");

            channel_id
        };

        {
            // Pretend that we have already done ChannelOpenTry on Starknet, and then continue with ChannelOpenAck

            let open_ack_message = CosmosChannelOpenAckMessage {
                port_id: "transfer".into(),
                channel_id: channel_id.to_string(),
                counterparty_channel_id: "63c350000c404581a3385ec7b4324008b2965dd8fc5af768b87329d25e57cfa".into(), // stub channel contract on Starknet as channel ID
                counterparty_version: "ics20-1".into(),
                update_height: Height::new(0, 1).unwrap(),
                proof_try: [0; 32].into(), // dummy proofs
            };

            cosmos_chain.send_message(open_ack_message.to_cosmos_message()).await?;
        }

        Ok(())
    })
}
