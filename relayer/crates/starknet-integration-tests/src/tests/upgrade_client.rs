use core::marker::PhantomData;
use core::time::Duration;

use cgp::core::field::Index;
use hermes_core::chain_components::traits::{CanQueryChainHeight, CanQueryChainStatus};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode};
use hermes_core::relayer_components::relay::traits::{
    CanSendTargetUpdateClientMessage, DestinationTarget, SourceTarget,
};
use hermes_core::relayer_components::transaction::impls::CanSendSingleMessageWithSigner;
use hermes_core::relayer_components::transaction::traits::HasDefaultSigner;
use hermes_core::runtime_components::traits::CanSleep;
use hermes_core::test_components::setup::traits::{CanSetupChain, CanSetupClients};
use hermes_cosmos::error::types::Error;
use hermes_cosmos::integration_tests::init::init_test_runtime;
use hermes_prelude::*;
use hermes_starknet_chain_components::impls::StarknetMessage;
use hermes_starknet_chain_components::traits::CanCallContract;
use hermes_starknet_chain_components::types::{
    CairoStarknetClientState, CairoStarknetConsensusState, Height,
};
use hermes_starknet_chain_context::contexts::StarknetCairoEncoding;
use hermes_starknet_relayer::contexts::StarknetToCosmosRelay;
use ibc::primitives::Timestamp;
use starknet::core::types::Felt;
use starknet::macros::selector;
use tracing::info;

use crate::contexts::StarknetTestSetup;
use crate::utils::init_starknet_setup;

#[test]
fn test_upgrade_clients() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let setup = init_starknet_setup(&runtime).await?;

        // different starknet sequencer private keys
        let starknet_sequencer_private_key_1 = Felt::ZERO;
        let starknet_sequencer_private_key_2 = Felt::ONE;

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
            None, // client refresh is not needed for this test
            None, // client refresh is not needed for this test
        );

        runtime.sleep(Duration::from_secs(1)).await;

        {
            let target_height = cosmos_chain.query_chain_height().await?;

            info!(
                "updating Cosmos client on Starknet to height {}",
                target_height
            );

            starknet_to_cosmos_relay
                .send_target_update_client_messages(SourceTarget, &target_height)
                .await?;

            info!("sent update client message from Cosmos to Starknet");
        }

        {
            let target_height = starknet_chain.query_chain_height().await?;

            info!(
                "updating Starknet client on Cosmos to height {}",
                target_height
            );

            starknet_to_cosmos_relay
                .send_target_update_client_messages(DestinationTarget, &target_height)
                .await?;
        }

        let starknet_current_block = starknet_chain.query_chain_status().await?;

        let starknet_final_height = starknet_current_block.height + 30; // 1 sec block time; 30 secs in future
        let starknet_final_timestamp = Timestamp::from_nanoseconds(
            starknet_current_block.time.unix_timestamp_nanos() as u64 + 30 * 1_000_000_000,
        );

        let ibc_core_contract_address = starknet_chain.ibc_core_contract_address.get().unwrap();

        {
            info!(
                "scheduling upgrade at Starknet with final height {}",
                starknet_final_height
            );

            // FIXME(rano): build correct upgraded states
            let upgraded_client_state = CairoStarknetClientState {
                latest_height: Height {
                    revision_number: 0,
                    revision_height: starknet_final_height,
                },
                final_height: starknet_final_height,
                chain_id: "dummy-chain".into(),
                sequencer_public_key: starknet_crypto::get_public_key(
                    &starknet_sequencer_private_key_2,
                ),
                ibc_contract_address: *ibc_core_contract_address,
            };

            let upgraded_consensus_state = CairoStarknetConsensusState {
                root: Felt::ZERO,
                time: starknet_final_timestamp,
            };

            let starknet_encoding = StarknetCairoEncoding;

            let calldata = starknet_encoding.encode(&product![
                starknet_final_height,
                upgraded_client_state,
                upgraded_consensus_state
            ])?;

            starknet_chain
                .send_message_with_signer(
                    starknet_chain.get_default_signer(),
                    StarknetMessage::new(
                        **ibc_core_contract_address,
                        selector!("schedule_upgrade"),
                        calldata,
                    ),
                )
                .await?;

            info!("sent schedule upgrade message to Starknet");

            {
                let output = starknet_chain
                    .call_contract(
                        ibc_core_contract_address,
                        &selector!("get_scheduled_upgrade"),
                        &vec![],
                        None,
                    )
                    .await?;

                let (_, _): (CairoStarknetClientState, CairoStarknetConsensusState) =
                    cairo_encoding.decode(&output)?;

                let output = starknet_chain
                    .call_contract(
                        ibc_core_contract_address,
                        &selector!("get_final_height"),
                        &vec![],
                        None,
                    )
                    .await?;

                let onchain_final_height: u64 = cairo_encoding.decode(&output)?;

                assert_eq!(onchain_final_height, starknet_final_height);
            }
        }

        {
            let target_height = starknet_chain.query_chain_height().await?;

            starknet_to_cosmos_relay
                .send_target_update_client_messages(DestinationTarget, &target_height)
                .await?;

            info!(
                "Starknet client update on Cosmos works on height {} (<= final height {})",
                target_height, starknet_final_height
            );
        }

        {
            info!("waiting for upgrade to complete on Starknet");

            // TODO(rano): can we wait by block height ?
            runtime.sleep(Duration::from_secs(30)).await;

            info!("upgrade completed on Starknet");

            let target_height = starknet_chain.query_chain_height().await?;

            assert!(target_height > starknet_final_height);

            info!(
                "expected error: {}",
                starknet_to_cosmos_relay
                    .send_target_update_client_messages(DestinationTarget, &target_height)
                    .await
                    .unwrap_err()
            );

            info!(
                "Starknet client update on Cosmos does not work on height {} (> final height {})",
                target_height, starknet_final_height
            );
        }

        // TODO(rano): rest of the steps

        {
            info!("restarting starknet chain with the new sequencer key");

            // restart starknet chain with starknet_sequencer_public_key_2
        }

        {
            // build upgrade client message for wasm light client

            // submit upgrade client message on Cosmos

            // unschedule_upgrade on starknet
        }

        {
            // try updating the client on Cosmos now
        }

        Ok(())
    })
}
