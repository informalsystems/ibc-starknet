use core::marker::PhantomData;
use core::time::Duration;
use std::env::var;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use flate2::write::GzEncoder;
use flate2::Compression;
use hermes_chain_components::traits::extract_data::CanExtractFromMessageResponse;
use hermes_cosmos_chain_components::impls::connection::connection_handshake_message::default_connection_version;
use hermes_cosmos_chain_components::traits::message::ToCosmosMessage;
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_chain_components::types::config::gas::eip_type::EipQueryType;
use hermes_cosmos_chain_components::types::events::connection::CosmosConnectionOpenInitEvent;
use hermes_cosmos_chain_components::types::messages::connection::open_init::CosmosConnectionOpenInitMessage;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::queries::chain_status::{
    CanQueryChainHeight, CanQueryChainStatus,
};
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::queries::consensus_state::CanQueryConsensusState;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::relay::traits::update_client_message_builder::CanSendTargetUpdateClientMessage;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_starknet_chain_components::impls::types::message::StarknetMessage;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_components::types::register::MsgRegisterClient;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use ibc::core::client::types::Height;
use sha2::{Digest, Sha256};
use starknet::accounts::Call;
use starknet::macros::{selector, short_string};
use tracing::info;

use crate::contexts::bootstrap::StarknetBootstrap;
use crate::contexts::osmosis_bootstrap::OsmosisBootstrap;

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

        let ibc_core_class_hash = {
            let contract_path = std::env::var("IBC_CORE_CONTRACT")?;

            let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

            let contract = serde_json::from_str(&contract_str)?;

            let class_hash = starknet_chain.declare_contract(&contract).await?;

            info!("declared IBC core class: {:?}", class_hash);

            class_hash
        };

        let ibc_core_address = starknet_chain
            .deploy_contract(&ibc_core_class_hash, false, &Vec::new())
            .await?;

        info!(
            "deployed IBC core contract to address: {:?}",
            ibc_core_address
        );

        let comet_client_class_hash = {
            let contract_path = std::env::var("COMET_CLIENT_CONTRACT")?;

            let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

            let contract = serde_json::from_str(&contract_str)?;

            let class_hash = starknet_chain.declare_contract(&contract).await?;

            info!("declared class: {:?}", class_hash);

            class_hash
        };

        let cairo_encoding = StarknetCairoEncoding;

        let comet_client_address = {
            let owner_call_data = cairo_encoding.encode(&ibc_core_address)?;
            let contract_address = starknet_chain
                .deploy_contract(&comet_client_class_hash, false, &owner_call_data)
                .await?;

            info!(
                "deployed Comet client contract to address: {:?}",
                contract_address
            );

            contract_address
        };

        let starknet_chain = {
            let mut fields = starknet_chain.fields.as_ref().clone();

            fields.ibc_core_contract_address = Some(ibc_core_address);
            fields.ibc_client_contract_address = Some(comet_client_address);

            StarknetChain {
                fields: Arc::new(fields),
            }
        };

        {
            // register comet client contract with ibc-core

            let register_client = MsgRegisterClient {
                client_type: short_string!("07-tendermint"),
                contract_address: comet_client_address,
            };

            let calldata = cairo_encoding.encode(&register_client)?;

            let call = Call {
                to: *ibc_core_address,
                selector: selector!("register_client"),
                calldata,
            };

            let message = StarknetMessage::new(call);

            let response = starknet_chain.send_message(message).await?;

            info!("IBC register client response: {:?}", response);
        }

        let cosmos_client_id = StarknetToCosmosRelay::create_client(
            DestinationTarget,
            cosmos_chain,
            &starknet_chain,
            &StarknetCreateClientPayloadOptions { wasm_code_hash },
            &(),
        )
        .await?;

        info!("created client id on Cosmos: {:?}", cosmos_client_id);

        let starknet_client_id = StarknetToCosmosRelay::create_client(
            SourceTarget,
            &starknet_chain,
            cosmos_chain,
            &Default::default(),
            &(),
        )
        .await?;

        info!("created client on Starknet: {:?}", starknet_client_id);

        let starknet_to_cosmos_relay = StarknetToCosmosRelay::new(
            runtime.clone(),
            starknet_chain.clone(),
            cosmos_chain.clone(),
            starknet_client_id.clone(),
            cosmos_client_id.clone(),
        );

        {
            let client_state = cosmos_chain
                .query_client_state(
                    PhantomData::<StarknetChain>,
                    &cosmos_client_id,
                    &cosmos_chain.query_chain_height().await?,
                )
                .await?;

            let client_height = client_state.client_state.latest_height;

            let consensus_state = cosmos_chain
                .query_consensus_state(
                    PhantomData::<StarknetChain>,
                    &cosmos_client_id,
                    &client_height.revision_height(),
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
            let client_state = starknet_chain
                .query_client_state(
                    PhantomData::<CosmosChain>,
                    &starknet_client_id,
                    &starknet_chain.query_chain_height().await?,
                )
                .await?;

            let consensus_state = starknet_chain
                .query_consensus_state(
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
                "initial Cosmos consensus state height {:?} and root: {:?} on Starknet",
                client_state.latest_height, consensus_state.root
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

            {
                let client_state = cosmos_chain
                    .query_client_state(
                        PhantomData::<StarknetChain>,
                        &cosmos_client_id,
                        &cosmos_chain.query_chain_height().await?,
                    )
                    .await?;

                let consensus_state = cosmos_chain
                    .query_consensus_state(
                        PhantomData::<StarknetChain>,
                        &cosmos_client_id,
                        &client_state.client_state.latest_height.revision_height(),
                        &cosmos_chain.query_chain_height().await?,
                    )
                    .await?;

                info!(
                    "after updating Starknet client state height {:?} and root: {:?} on Cosmos",
                    client_state.client_state.latest_height, consensus_state.consensus_state.root
                );
            }

            let consensus_state = cosmos_chain
                .query_consensus_state(
                    PhantomData::<StarknetChain>,
                    &cosmos_client_id,
                    &starknet_status.height,
                    &cosmos_chain.query_chain_height().await?,
                )
                .await?;

            assert_eq!(
                consensus_state.consensus_state.root.clone().into_vec(),
                starknet_status.block_hash.to_bytes_be()
            );

            info!(
                "updated Starknet consensus state to Cosmos to height {} and root: {:?}",
                starknet_status.height, consensus_state.consensus_state.root
            );
        }

        {
            runtime.sleep(Duration::from_secs(2)).await;

            let cosmos_status = cosmos_chain.query_chain_status().await?;

            info!(
                "updating Cosmos client to Starknet to height {}",
                cosmos_status.height,
            );

            // TODO(rano): how do I query cosmos block root

            starknet_to_cosmos_relay
                .send_target_update_client_messages(SourceTarget, &cosmos_status.height)
                .await?;

            {
                let client_state = starknet_chain
                    .query_client_state(
                        PhantomData::<CosmosChain>,
                        &starknet_client_id,
                        &starknet_chain.query_chain_height().await?,
                    )
                    .await?;

                let consensus_state = starknet_chain
                    .query_consensus_state(
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
                    "after updating Cosmos client state with height {:?} and root: {:?} on Starknet",
                    client_state.latest_height, consensus_state.root
                );
            }

            let consensus_state = starknet_chain
                .query_consensus_state(
                    PhantomData::<CosmosChain>,
                    &starknet_client_id,
                    &cosmos_status.height,
                    &starknet_chain.query_chain_height().await?,
                )
                .await?;

            // TODO(rano): add assert

            info!(
                "updated Cosmos consensus state to Starknet to height {} and root: {:?}",
                cosmos_status.height, consensus_state.root
            );
        }

        let _cosmos_connection_id = {
            let open_init_message = CosmosConnectionOpenInitMessage {
                client_id: cosmos_client_id.to_string(),
                counterparty_client_id: starknet_client_id.to_string(),
                counterparty_commitment_prefix: "ibc".into(),
                version: default_connection_version(),
                delay_period: Duration::from_secs(0),
            };

            let events = cosmos_chain
                .send_message(open_init_message.to_cosmos_message())
                .await?;

            let connection_id = cosmos_chain
                .try_extract_from_message_response(
                    PhantomData::<CosmosConnectionOpenInitEvent>,
                    &events,
                )
                .unwrap()
                .connection_id;

            info!("initialized connection on Cosmos: {connection_id}");

            connection_id
        };

        Ok(())
    })
}
