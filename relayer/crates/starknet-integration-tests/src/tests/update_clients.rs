use core::marker::PhantomData;
use core::time::Duration;
use std::env::var;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use cgp::prelude::*;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_components::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use hermes_chain_components::traits::queries::consensus_state::CanQueryConsensusStateWithLatestHeight;
use hermes_chain_components::traits::send_message::CanSendSingleMessage;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_wasm_relayer::context::cosmos_bootstrap::CosmosWithWasmClientBootstrap;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_error::types::Error;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::relay::traits::update_client_message_builder::CanSendTargetUpdateClientMessage;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use ibc::core::client::types::Height as CosmosHeight;
use sha2::{Digest, Sha256};
use starknet::accounts::Call;
use starknet::macros::{selector, short_string};
use tracing::info;

use crate::contexts::bootstrap::StarknetBootstrap;

#[test]
fn test_relay_update_clients() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let starknet_chain_command_path = std::env::var("STARKNET_BIN")
            .unwrap_or("starknet-devnet".into())
            .into();

        let wasm_client_code_path = PathBuf::from(
            var("STARKNET_WASM_CLIENT_PATH")
                .expect("Wasm blob for Starknet light client is required"),
        );

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let starknet_bootstrap = StarknetBootstrap {
            runtime: runtime.clone(),
            chain_command_path: starknet_chain_command_path,
            chain_store_dir: format!("./test-data/{timestamp}/starknet").into(),
        };

        let wasm_client_byte_code = tokio::fs::read(&wasm_client_code_path).await?;

        let wasm_code_hash: [u8; 32] = {
            let mut hasher = Sha256::new();
            hasher.update(&wasm_client_byte_code);
            hasher.finalize().into()
        };

        let cosmos_builder = CosmosBuilder::new_with_default(runtime.clone());

        let cosmos_bootstrap = Arc::new(CosmosWithWasmClientBootstrap {
            runtime: runtime.clone(),
            cosmos_builder,
            should_randomize_identifiers: true,
            chain_store_dir: format!("./test-data/{timestamp}/cosmos").into(),
            chain_command_path: "simd".into(),
            account_prefix: "cosmos".into(),
            staking_denom_prefix: "stake".into(),
            transfer_denom_prefix: "coin".into(),
            wasm_client_byte_code,
            governance_proposal_authority: "cosmos10d07y265gmmuvt4z0w9aw880jnsr700j6zn9kn".into(), // TODO: don't hard code this
            dynamic_gas: None,
        });

        let mut starknet_chain_driver = starknet_bootstrap.bootstrap_chain("starknet").await?;

        let starknet_chain = &mut starknet_chain_driver.chain;

        let cosmos_chain_driver = cosmos_bootstrap.bootstrap_chain("cosmos").await?;

        let cosmos_chain = &cosmos_chain_driver.chain;

        let ibc_core_class_hash = {
            let contract_path = std::env::var("IBC_CORE_CONTRACT")?;

            let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

            let contract = serde_json::from_str(&contract_str)?;

            let class_hash = starknet_chain.declare_contract(&contract).await?;

            info!("declared class for ibc-core: {:?}", class_hash);

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

            info!("declared class for cometbft: {:?}", class_hash);

            class_hash
        };

        let comet_client_address = starknet_chain
            .deploy_contract(&comet_client_class_hash, false, &Vec::new())
            .await?;

        info!(
            "deployed Comet client contract to address: {:?}",
            comet_client_address
        );

        let cairo_encoding = StarknetCairoEncoding;

        {
            // register comet client contract with ibc-core

            let calldata = cairo_encoding.encode(&product![
                short_string!("07-cometbft"),
                comet_client_address
            ])?;

            let call = Call {
                to: ibc_core_address,
                selector: selector!("register_client"),
                calldata,
            };

            let response = starknet_chain.send_message(call).await?;

            info!("IBC register client response: {:?}", response);
        }

        starknet_chain.ibc_core_contract_address = Some(ibc_core_address);
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

        let cosmos_client_id = StarknetToCosmosRelay::create_client(
            DestinationTarget,
            cosmos_chain,
            starknet_chain,
            &StarknetCreateClientPayloadOptions { wasm_code_hash },
            &(),
        )
        .await?;

        info!("created client on Cosmos: {:?}", cosmos_client_id);

        let starknet_to_cosmos_relay = StarknetToCosmosRelay {
            runtime: runtime.clone(),
            src_chain: starknet_chain.clone(),
            dst_chain: cosmos_chain.clone(),
            src_client_id: starknet_client_id.clone(),
            dst_client_id: cosmos_client_id.clone(),
        };

        {
            info!("test relaying UpdateClient from Cosmos to Starknet");

            {
                let client_state = starknet_chain
                    .query_client_state_with_latest_height(
                        PhantomData::<CosmosChain>,
                        &starknet_client_id,
                    )
                    .await?;

                info!("Cosmos client state on Starknet: {client_state:?}");

                let consensus_state = starknet_chain
                    .query_consensus_state_with_latest_height(
                        PhantomData::<CosmosChain>,
                        &starknet_client_id,
                        &CosmosHeight::new(
                            client_state.latest_height.revision_number,
                            client_state.latest_height.revision_height,
                        )?,
                    )
                    .await?;

                info!("Cosmos consensus state on Starknet: {consensus_state:?}");
            }

            runtime.sleep(Duration::from_secs(1)).await;

            let target_height = cosmos_chain.query_chain_height().await?;

            info!(
                "updating Cosmos client on Starknet to height {}",
                target_height
            );

            starknet_to_cosmos_relay
                .send_target_update_client_messages(SourceTarget, &target_height)
                .await?;

            info!("sent update client message from Cosmos to Starknet");

            {
                let client_state = starknet_chain
                    .query_client_state_with_latest_height(
                        PhantomData::<CosmosChain>,
                        &starknet_client_id,
                    )
                    .await?;

                info!("Cosmos client state on Starknet after UpdateClient: {client_state:?}");

                let consensus_state = starknet_chain
                    .query_consensus_state_with_latest_height(
                        PhantomData::<CosmosChain>,
                        &starknet_client_id,
                        &CosmosHeight::new(
                            client_state.latest_height.revision_number,
                            client_state.latest_height.revision_height,
                        )?,
                    )
                    .await?;

                info!("Cosmos consensus state on Starknet after UpdateClient: {consensus_state:?}");
            }
        }

        {
            info!("test relaying UpdateClient from Starknet to Cosmos");

            {
                let client_state = cosmos_chain
                    .query_client_state_with_latest_height(
                        PhantomData::<StarknetChain>,
                        &cosmos_client_id,
                    )
                    .await?;

                info!("Starknet client state on Cosmos: {client_state:?}");

                let consensus_state = cosmos_chain
                    .query_consensus_state_with_latest_height(
                        PhantomData::<StarknetChain>,
                        &cosmos_client_id,
                        &client_state.client_state.latest_height.revision_height(),
                    )
                    .await?;

                info!("Starknet consensus state on Cosmos: {consensus_state:?}");
            }

            let target_height = starknet_chain.query_chain_height().await?;

            info!(
                "updating Starknet client on Cosmos to height {}",
                target_height
            );

            starknet_to_cosmos_relay
                .send_target_update_client_messages(DestinationTarget, &target_height)
                .await?;

            {
                let client_state = cosmos_chain
                    .query_client_state_with_latest_height(
                        PhantomData::<StarknetChain>,
                        &cosmos_client_id,
                    )
                    .await?;

                info!("Starknet client state on Cosmos after update: {client_state:?}");

                let consensus_state = cosmos_chain
                    .query_consensus_state_with_latest_height(
                        PhantomData::<StarknetChain>,
                        &cosmos_client_id,
                        &client_state.client_state.latest_height.revision_height(),
                    )
                    .await?;

                info!("Starknet consensus state on Cosmos after update: {consensus_state:?}");
            }
        }

        Ok(())
    })
}
