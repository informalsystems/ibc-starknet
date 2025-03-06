use core::marker::PhantomData;
use core::time::Duration;
use std::env::var;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_components::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use hermes_chain_components::traits::queries::consensus_state::CanQueryConsensusStateWithLatestHeight;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_wasm_relayer::context::cosmos_bootstrap::CosmosWithWasmClientBootstrap;
use hermes_error::types::Error;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::relay::traits::update_client_message_builder::CanSendTargetUpdateClientMessage;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use ibc::core::client::types::Height as CosmosHeight;
use sha2::{Digest, Sha256};
use tracing::info;

use crate::utils::init_starknet_bootstrap;

#[test]
fn test_relay_update_clients() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let wasm_client_code_path = PathBuf::from(
            var("STARKNET_WASM_CLIENT_PATH")
                .expect("Wasm blob for Starknet light client is required"),
        );

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let starknet_bootstrap = init_starknet_bootstrap(&runtime).await?;

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

        let starknet_chain: &StarknetChain = &mut starknet_chain_driver.chain;

        let cosmos_chain_driver = cosmos_bootstrap.bootstrap_chain("cosmos").await?;

        let cosmos_chain = &cosmos_chain_driver.chain;

        let cairo_encoding = StarknetCairoEncoding;

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

        let starknet_to_cosmos_relay = StarknetToCosmosRelay::new(
            runtime.clone(),
            starknet_chain.clone(),
            cosmos_chain.clone(),
            starknet_client_id.clone(),
            cosmos_client_id.clone(),
        );

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
