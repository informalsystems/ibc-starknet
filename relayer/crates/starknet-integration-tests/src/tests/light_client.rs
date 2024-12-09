use core::marker::PhantomData;
use std::env::var;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use flate2::write::GzEncoder;
use flate2::Compression;
use hermes_cosmos_chain_components::traits::message::ToCosmosMessage;
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_chain_components::types::config::gas::eip_type::EipQueryType;
use hermes_cosmos_chain_components::types::messages::channel::open_ack::CosmosChannelOpenAckMessage;
use hermes_cosmos_chain_components::types::messages::channel::open_init::CosmosChannelOpenInitMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_ack::CosmosConnectionOpenAckMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_init::CosmosConnectionOpenInitMessage;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::contexts::encoding::CosmosEncoding;
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
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::relay::traits::update_client_message_builder::CanSendTargetUpdateClientMessage;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use ibc::core::channel::types::channel::State;
use ibc::core::client::types::Height;
use ibc::core::connection::types::version::Version;
use ibc::core::host::types::identifiers::ClientId;
use ibc_proto::ibc::core::channel::v1::{Channel, Counterparty};
use sha2::{Digest, Sha256};
use starknet::core::types::Felt;
use tracing::info;

use crate::contexts::bootstrap::StarknetBootstrap;
use crate::contexts::osmosis_bootstrap::OsmosisBootstrap;

fn felt_to_trimmed_string(v: &Felt) -> String {
    String::from_utf8_lossy(&v.to_bytes_be())
        .trim_start_matches('\0')
        .to_string()
}

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

    let cosmos_builder = CosmosBuilder::new_with_default(runtime.clone());

    runtime.runtime.clone().block_on(async move {
        let wasm_client_byte_code = tokio::fs::read(&wasm_client_code_path).await?;

        let wasm_code_hash: [u8; 32] = {
            let mut hasher = Sha256::new();
            hasher.update(&wasm_client_byte_code);
            hasher.finalize().into()
        };

        let wasm_client_byte_code_gzip = {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&wasm_client_byte_code)?;
            encoder.finish()?
        };

        let cosmos_bootstrap = Arc::new(OsmosisBootstrap {
            runtime: runtime.clone(),
            cosmos_builder,
            should_randomize_identifiers: true,
            chain_store_dir: store_dir.join("chains"),
            chain_command_path: "osmosisd".into(),
            account_prefix: "osmo".into(),
            staking_denom_prefix: "stake".into(),
            transfer_denom_prefix: "coin".into(),
            wasm_client_byte_code: wasm_client_byte_code_gzip,
            governance_proposal_authority: "osmo10d07y265gmmuvt4z0w9aw880jnsr700jjeq4qp".into(), // TODO: don't hard code this
            dynamic_gas: Some(DynamicGasConfig {
                multiplier: 1.1,
                max: 1.6,
                eip_query_type: EipQueryType::Osmosis,
                denom: "stake".to_owned(),
            }),
        });

        let starknet_bootstrap = StarknetBootstrap {
            runtime: runtime.clone(),
            chain_command_path: "starknet-devnet".into(),
            chain_store_dir: store_dir,
        };

        let cosmos_chain_driver = cosmos_bootstrap.bootstrap_chain("cosmos").await?;

        let mut starknet_chain_driver = starknet_bootstrap.bootstrap_chain("starknet").await?;

        let cosmos_chain = cosmos_chain_driver.chain();

        let starknet_chain = &mut starknet_chain_driver.chain;

        let cosmos_client_id = StarknetToCosmosRelay::create_client(
            DestinationTarget,
            cosmos_chain,
            starknet_chain,
            &StarknetCreateClientPayloadOptions { wasm_code_hash },
            &(),
        )
        .await?;

        info!("created client id on Cosmos: {:?}", cosmos_client_id);

        let comet_client_class_hash = {
            let contract_path = std::env::var("COMET_CLIENT_CONTRACT")?;

            let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

            let contract = serde_json::from_str(&contract_str)?;

            let class_hash = starknet_chain.declare_contract(&contract).await?;

            info!("declared class: {:?}", class_hash);

            class_hash
        };

        let comet_client_address = starknet_chain
            .deploy_contract(&comet_client_class_hash, false, &Vec::new())
            .await?;

        info!(
            "deployed Comet client contract to address: {:?}",
            comet_client_address
        );

        starknet_chain.ibc_client_contract_address = Some(comet_client_address);

        let starknet_client_id = StarknetToCosmosRelay::create_client(
            SourceTarget,
            starknet_chain,
            cosmos_chain,
            &Default::default(),
            &(),
        )
        .await?;

        info!("created client on Starknet: {:?}", starknet_client_id);

        let starknet_to_cosmos_relay = StarknetToCosmosRelay {
            runtime: runtime.clone(),
            src_chain: starknet_chain.clone(),
            dst_chain: cosmos_chain.clone(),
            // TODO: stub
            src_client_id: starknet_client_id.clone(),
            dst_client_id: cosmos_client_id.clone(),
        };

        {
            let client_state =
                cosmos_chain.query_client_state(
                    PhantomData::<StarknetChain>,
                    &cosmos_client_id,
                    &cosmos_chain.query_chain_height().await?,
                )
                .await?;

            let client_height = client_state.client_state.latest_height.revision_height();

            let consensus_state =
                cosmos_chain.query_consensus_state(
                    PhantomData::<StarknetChain>,
                    &cosmos_client_id,
                    &client_height,
                    &cosmos_chain.query_chain_height().await?,
                )
                .await?;

            info!(
                "initial Starknet consensus state height {} and root: {:?} on Cosmos",
                client_height,
                consensus_state.consensus_state.root.into_vec()
            );
        }

        {
            let client_state =
                starknet_chain.query_client_state(
                    PhantomData::<CosmosChain>,
                    &starknet_client_id,
                    &starknet_chain.query_chain_height().await?,
                )
                .await?;

            let consensus_state =
                starknet_chain.query_consensus_state(
                    PhantomData::<CosmosChain>,
                    &starknet_client_id,
                    &Height::new(
                        client_state.latest_height.revision_number,
                        client_state.latest_height.revision_height,
                    )?,
                    &starknet_chain.query_chain_height().await?,
                )
                .await?;

            info!(
                "initial Cosmos consensus state height {} and root: {:?} on Starknet",
                client_state.latest_height.revision_height,
                consensus_state.root
            );
        }

        {
            runtime.sleep(Duration::from_secs(2)).await;

            let starknet_status = starknet_chain.query_chain_status().await?;

            info!(
                "updating Starknet client to Cosmos to height {} and root: {:?}",
                starknet_status.height,
                starknet_status.block_hash.to_bytes_be()
            );

            starknet_to_cosmos_relay
                .send_target_update_client_messages(DestinationTarget, &starknet_status.height)
                .await?;

            let consensus_state =
                cosmos_chain.query_consensus_state(
                    PhantomData::<StarknetChain>,
                    &cosmos_client_id,
                    &starknet_status.height,
                    &cosmos_chain.query_chain_height().await?,
                )
                .await?;

            assert_eq!(
                consensus_state.consensus_state.root.into_vec(),
                starknet_status.block_hash.to_bytes_be()
            );
        }

        {
            runtime.sleep(Duration::from_secs(2)).await;

            let cosmos_status= cosmos_chain.query_chain_status().await?;

            info!(
                "updating Cosmos client to Starknet to height {}",
                cosmos_status.height,
            );

            // TODO(rano): how do I query cosmos block root

            starknet_to_cosmos_relay
                .send_target_update_client_messages(SourceTarget, &cosmos_status.height)
                .await?;

            let consensus_state =
                starknet_chain.query_consensus_state(
                    PhantomData::<CosmosChain>,
                    &starknet_client_id,
                    &cosmos_status.height,
                    &starknet_chain.query_chain_height().await?,
                )
                .await?;


            // TODO(rano): add assert

            info!(
                "updated Cosmos client to Starknet to height {} and root: {:?}",
                cosmos_status.height,
                consensus_state.root
            );
        }

        let cosmos_connection_id = {
            let open_init_message = CosmosConnectionOpenInitMessage {
                client_id: cosmos_client_id.clone(),
                counterparty_client_id: ClientId::new(&felt_to_trimmed_string(&starknet_client_id.client_type), starknet_client_id.sequence)?,
                counterparty_commitment_prefix: "ibc".into(),
                version: Version::compatibles().pop().unwrap(),
                delay_period: Duration::from_secs(0),
            };

            let events = cosmos_chain.send_message(open_init_message.to_cosmos_message()).await?;

            let connection_id = <CosmosChain as HasConnectionOpenInitEvent<StarknetChain>>::try_extract_connection_open_init_event(&events)
                .unwrap()
                .connection_id
            ;

            info!("initialized connection on Cosmos: {connection_id}");

            connection_id
        };

        {
            // Pretend that we have relayed ConnectionOpenTry to Starknet, and then send ConnectionOpenAck.

            let payload = <CosmosChain as CanBuildCreateClientPayload<CosmosChain>>::build_create_client_payload(cosmos_chain, &Default::default(),
            ).await?;

            let client_state = CosmosEncoding.convert(&payload.client_state)?;

            runtime.sleep(Duration::from_secs(1)).await;

            let open_ack_message = CosmosConnectionOpenAckMessage {
                connection_id: cosmos_connection_id.clone(),
                counterparty_connection_id: cosmos_connection_id.clone(), // TODO: stub
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
                ordering: 1,
                counterparty: Some(Counterparty {
                    port_id: "11b7f9bfa43d3facae74efa5dfe0030df98273271278291d67c16a4e6cd5f7c".to_string(), // stub application contract on Starknet as port ID
                    channel_id: "".to_string(),
                }),
                connection_hops: vec![cosmos_connection_id.to_string()],
                version: "ics20-1".into(),
                upgrade_sequence: 0,
            };

            let open_init_message = CosmosChannelOpenInitMessage {
                port_id: "transfer".into(),
                channel,
            };

            let events = cosmos_chain.send_message(open_init_message.to_cosmos_message()).await?;

            let channel_id = <CosmosChain as HasChannelOpenInitEvent<StarknetChain>>::try_extract_channel_open_init_event(&events)
                .unwrap()
                .channel_id
            ;

            info!("initialized channel on Cosmos: {channel_id}");

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
