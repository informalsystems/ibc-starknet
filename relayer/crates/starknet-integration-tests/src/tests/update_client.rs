use core::marker::PhantomData;
use core::time::Duration;

use cgp::core::field::Index;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_components::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use hermes_chain_components::traits::queries::consensus_state::CanQueryConsensusStateWithLatestHeight;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::types::Error;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::relay::traits::update_client_message_builder::CanSendTargetUpdateClientMessage;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_test_components::setup::traits::chain::CanSetupChain;
use hermes_test_components::setup::traits::clients::CanSetupClients;
use ibc::core::client::types::Height as CosmosHeight;
use tracing::info;

use crate::contexts::setup::StarknetTestSetup;
use crate::utils::init_starknet_setup;

#[test]
fn test_relay_update_clients() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let setup = init_starknet_setup(&runtime).await?;

        let starknet_chain_driver = setup.setup_chain(PhantomData::<Index<0>>).await?;

        let cosmos_chain_driver = setup.setup_chain(PhantomData::<Index<1>>).await?;

        let starknet_chain = &starknet_chain_driver.chain;

        let cosmos_chain = &cosmos_chain_driver.chain;

        let (starknet_client_id, cosmos_client_id) = <StarknetTestSetup as CanSetupClients<
            Index<0>,
            Index<1>,
        >>::setup_clients(
            &setup, starknet_chain, cosmos_chain
        )
        .await?;

        let cairo_encoding = StarknetCairoEncoding;

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
