//! This test will assert that packet clearing works correctly.
//!
//! This test will be built step by step when each component
//! required for packet clearing is added.

use core::marker::PhantomData;
use core::time::Duration;

use hermes_core::chain_components::traits::{CanQueryChainHeight, CanQueryChainStatus};
use hermes_core::relayer_components::birelay::traits::CanAutoBiRelay;
use hermes_core::relayer_components::relay::traits::{
    CanAutoRelayWithHeights, CanRelayPacket, SourceTarget,
};
use hermes_core::relayer_components::transaction::traits::CanSendMessagesWithSigner;
use hermes_core::runtime_components::traits::CanSleep;
use hermes_core::test_components::chain::traits::{
    CanAssertEventualAmount, CanBuildIbcTokenTransferMessages, CanIbcTransferToken, CanQueryBalance,
};
use hermes_cosmos::error::types::Error;
use hermes_cosmos::integration_tests::init::init_test_runtime;
use hermes_cosmos::relayer::contexts::CosmosChain;
use hermes_cosmos::test_components::chain::types::Amount;
use hermes_starknet_chain_components::impls::StarknetAddress;
use hermes_starknet_chain_components::traits::{
    CanQueryCosmosTokenAddressOnStarknet, CanQueryTokenBalance,
};
use hermes_starknet_chain_components::types::{Denom, PrefixedDenom, StarknetAmount, TracePrefix};
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use ibc::core::host::types::identifiers::PortId;
use ibc::primitives::Timestamp;
use tracing::info;

use crate::utils::init_starknet_test_driver;

#[test]
fn test_relay_timeout_packet() -> Result<(), Error> {
    // ### SETUP START ###
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let test_driver = init_starknet_test_driver(&runtime).await?;

        let starknet_chain_driver = &test_driver.starknet_chain_driver;
        let cosmos_chain_driver = &test_driver.cosmos_chain_driver;

        let starknet_chain = &starknet_chain_driver.chain;
        let cosmos_chain = &cosmos_chain_driver.chain;

        let wallet_cosmos_a = &cosmos_chain_driver.user_wallet_a;
        let address_cosmos_a = &wallet_cosmos_a.address;

        let denom_cosmos = &cosmos_chain_driver.genesis_config.transfer_denom;

        let wallet_starknet_b = &starknet_chain_driver.user_wallet_a;
        let address_starknet_b = &wallet_starknet_b.account_address;

        let starknet_channel_id = &test_driver.channel_id_a;
        let cosmos_channel_id = &test_driver.channel_id_b;

        let cairo_encoding = StarknetCairoEncoding;

        let ics20_contract_address = starknet_chain.ibc_ics20_contract_address.get().unwrap();

        let birelay = &test_driver.relay_driver_a_b.birelay;

        let starknet_to_cosmos_relay = &birelay.relay_a_to_b;
        let cosmos_to_starknet_relay = &birelay.relay_b_to_a;

        let transfer_quantity = 1_000u128;
        let transfer_back_quantity = 310u128;

        let balance_cosmos_a_step_0 = cosmos_chain
            .query_balance(address_cosmos_a, denom_cosmos)
            .await?;

        info!(
            "cosmos balance before transfer: {}",
            balance_cosmos_a_step_0
        );

        let packet = cosmos_chain
            .ibc_transfer_token(
                PhantomData::<StarknetChain>,
                cosmos_channel_id,
                &PortId::transfer(),
                wallet_cosmos_a,
                address_starknet_b,
                &Amount::new(transfer_quantity, denom_cosmos.clone()),
                &None,
                &starknet_chain.query_chain_status().await?,
            )
            .await?;

        runtime.sleep(Duration::from_secs(2)).await;

        let balance_cosmos_a_step_1_pre = cosmos_chain
            .query_balance(address_cosmos_a, denom_cosmos)
            .await?;

        // Assert tokens have been escrowed from the wallet
        assert_eq!(
            balance_cosmos_a_step_0.quantity - transfer_quantity,
            balance_cosmos_a_step_1_pre.quantity
        );

        cosmos_to_starknet_relay.relay_packet(&packet).await?;

        runtime.sleep(Duration::from_secs(2)).await;

        let balance_cosmos_a_step_1 = cosmos_chain
            .query_balance(address_cosmos_a, denom_cosmos)
            .await?;

        info!("cosmos balance after transfer: {}", balance_cosmos_a_step_1);

        assert_eq!(
            balance_cosmos_a_step_0.quantity - transfer_quantity,
            balance_cosmos_a_step_1.quantity
        );

        let ics20_contract_address = *starknet_chain.ibc_ics20_contract_address.get().unwrap();

        let ics20_token_address: StarknetAddress = {
            let ibc_prefixed_denom = PrefixedDenom {
                trace_path: vec![TracePrefix {
                    port_id: PortId::transfer().to_string(),
                    channel_id: starknet_channel_id.to_string(),
                }],
                base: Denom::Hosted(denom_cosmos.to_string()),
            };

            starknet_chain
                .query_cosmos_token_address_on_starknet(&ibc_prefixed_denom)
                .await?
                .unwrap()
        };

        info!("ics20 token address: {:?}", ics20_token_address);

        let balance_cosmos_a_step_0 = cosmos_chain
            .query_balance(address_cosmos_a, denom_cosmos)
            .await?;

        let balance_starknet_b_step_0 = starknet_chain
            .query_token_balance(&ics20_token_address, address_starknet_b)
            .await?;

        info!(
            "cosmos balance before transfer: {}",
            balance_cosmos_a_step_0
        );

        info!(
            "starknet balance before transfer: {}",
            balance_starknet_b_step_0.quantity
        );

        birelay
            .auto_bi_relay(Some(Duration::from_secs(10)), Some(Duration::from_secs(0)))
            .await?;

        // ### SETUP DONE ###

        let balance_cosmos_a_step_0 = cosmos_chain
            .query_balance(address_cosmos_a, denom_cosmos)
            .await?;

        let balance_starknet_b_step_0 = starknet_chain
            .query_token_balance(&ics20_token_address, address_starknet_b)
            .await?;

        info!("send IBC transfer from Cosmos to Starknet");

        let height_a1 = starknet_chain.query_chain_height().await?;

        let height_b1 = cosmos_chain.query_chain_height().await?;

        // build packets with fast timeout
        let timeout = (Timestamp::now() + Duration::from_secs(5))?;

        {
            // Create Cosmos to Starknet transfer
            let messages = cosmos_chain
                .build_ibc_token_transfer_messages(
                    PhantomData::<StarknetChain>,
                    cosmos_channel_id,
                    &PortId::transfer(),
                    address_starknet_b,
                    &Amount::new(transfer_quantity, denom_cosmos.clone()),
                    &None,
                    None,
                    Some(&timeout),
                )
                .await?;

            cosmos_chain
                .send_messages_with_signer(&wallet_cosmos_a.keypair, &messages)
                .await?;
        }

        {
            let messages = starknet_chain
                .build_ibc_token_transfer_messages(
                    PhantomData::<CosmosChain>,
                    starknet_channel_id,
                    &PortId::transfer(),
                    address_cosmos_a,
                    &StarknetAmount::new(transfer_quantity.into(), ics20_token_address),
                    &None,
                    None,
                    Some(&timeout),
                )
                .await?;

            starknet_chain
                .send_messages_with_signer(wallet_starknet_b, &messages)
                .await?;
        }

        let balance_cosmos_a_step_1 = cosmos_chain
            .query_balance(address_cosmos_a, denom_cosmos)
            .await?;

        let balance_starknet_b_step_1 = starknet_chain
            .query_token_balance(&ics20_token_address, address_starknet_b)
            .await?;

        info!("assert amount has been escrowed from the wallets");

        assert_eq!(
            balance_cosmos_a_step_0.quantity - transfer_quantity,
            balance_cosmos_a_step_1.quantity
        );

        assert_eq!(
            balance_starknet_b_step_0.quantity - transfer_quantity.into(),
            balance_starknet_b_step_1.quantity
        );

        info!("wait for packet to timeout");

        runtime.sleep(Duration::from_secs(6)).await;

        info!("will relay timeout packets");

        let height_a2 = starknet_chain.query_chain_height().await?;

        let height_b2 = cosmos_chain.query_chain_height().await?;

        starknet_to_cosmos_relay
            .auto_relay_with_heights(SourceTarget, &height_a1, Some(&height_a2))
            .await?;

        cosmos_to_starknet_relay
            .auto_relay_with_heights(SourceTarget, &height_b1, Some(&height_b2))
            .await?;

        cosmos_chain
            .assert_eventual_amount(address_cosmos_a, &balance_cosmos_a_step_0)
            .await?;

        starknet_chain
            .assert_eventual_amount(address_starknet_b, &balance_starknet_b_step_0)
            .await?;

        Ok(())
    })
}
