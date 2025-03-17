//! This test will assert that packet clearing works correctly.
//!
//! This test will be built step by step when each component
//! required for packet clearing is added.

use core::marker::PhantomData;
use core::time::Duration;
use std::sync::Arc;
use std::time::SystemTime;

use cgp::prelude::*;
use hermes_chain_components::traits::packet::fields::{
    HasPacketTimeoutHeight, HasPacketTimeoutTimestamp,
};
use hermes_chain_components::traits::queries::chain_status::{
    CanQueryChainHeight, CanQueryChainStatus,
};
use hermes_chain_components::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use hermes_chain_components::traits::queries::packet_commitment::CanQueryPacketCommitment;
use hermes_chain_components::traits::types::chain_id::HasChainId;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_chain_components::types::config::gas::eip_type::EipQueryType;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::chain::types::amount::Amount;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_error::types::Error;
use hermes_relayer_components::birelay::traits::CanAutoBiRelay;
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::traits::chains::{HasRelayChains, HasRelayPacketType};
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::packet_relayer::CanRelayPacket;
use hermes_relayer_components::relay::traits::packet_relayers::receive_packet::CanRelayReceivePacket;
use hermes_relayer_components::relay::traits::packet_relayers::timeout_unordered_packet::CanRelayTimeoutUnorderedPacket;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::send_messages_with_signer::CanSendMessagesWithSigner;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::impls::types::message::StarknetMessage;
use hermes_starknet_chain_components::traits::contract::call::CanCallContract;
use hermes_starknet_chain_components::traits::queries::token_balance::CanQueryTokenBalance;
use hermes_starknet_chain_components::types::amount::StarknetAmount;
use hermes_starknet_chain_components::types::cosmos::height::Height;
use hermes_starknet_chain_components::types::cosmos::timestamp::Timestamp;
use hermes_starknet_chain_components::types::messages::ibc::denom::{
    Denom, PrefixedDenom, TracePrefix,
};
use hermes_starknet_chain_components::types::messages::ibc::ibc_transfer::MsgTransfer;
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use hermes_starknet_chain_context::impls::error::SignError;
use hermes_starknet_relayer::contexts::cosmos_to_starknet_relay::CosmosToStarknetRelay;
use hermes_starknet_relayer::contexts::starknet_cosmos_birelay::StarknetCosmosBiRelay;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;
use ibc::core::connection::types::version::Version as IbcConnectionVersion;
use ibc::core::host::types::identifiers::{PortId as IbcPortId, Sequence};
use poseidon::Poseidon3Hasher;
use starknet::accounts::{Account, AccountError, ExecutionEncoding, SingleOwnerAccount};
use starknet::core::types::{Call, U256};
use starknet::macros::selector;
use starknet::providers::Provider;
use starknet::signers::{LocalWallet, SigningKey};
use tracing::info;

use crate::contexts::osmosis_bootstrap::OsmosisBootstrap;
use crate::utils::{init_starknet_bootstrap, load_wasm_client};

#[test]
fn test_packet_clearing() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let wasm_client_code_path =
            std::env::var("STARKNET_WASM_CLIENT_PATH")
                .expect("Wasm blob for Starknet light client is required")
        ;

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let starknet_bootstrap = init_starknet_bootstrap(&runtime).await?;

        let (wasm_code_hash, wasm_client_byte_code) =
            load_wasm_client(&wasm_client_code_path).await?;

        let cosmos_builder = CosmosBuilder::new_with_default(runtime.clone());

        let cosmos_bootstrap = Arc::new(OsmosisBootstrap {
            runtime: runtime.clone(),
            cosmos_builder,
            should_randomize_identifiers: true,
            chain_store_dir: format!("./test-data/{timestamp}/osmosis").into(),
            chain_command_path: "osmosisd".into(),
            account_prefix: "osmo".into(),
            staking_denom_prefix: "stake".into(),
            transfer_denom_prefix: "coin".into(),
            wasm_client_byte_code,
            governance_proposal_authority: "osmo10d07y265gmmuvt4z0w9aw880jnsr700jjeq4qp".into(), // TODO: don't hard code this
            dynamic_gas: Some(DynamicGasConfig {
                multiplier: 1.1,
                max: 1.6,
                eip_query_type: EipQueryType::Osmosis,
                denom: "stake".to_owned(),
            }),
        });

        let mut starknet_chain_driver = starknet_bootstrap.bootstrap_chain("starknet").await?;

        let starknet_chain = &mut starknet_chain_driver.chain;

        info!(
            "started starknet chain at port {}",
            starknet_chain_driver.node_config.rpc_port
        );

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

        let client_state_on_starknet = starknet_chain
            .query_client_state_with_latest_height(PhantomData::<CosmosChain>, &starknet_client_id)
            .await?;

        info!("client state on Starknet: {:?}", client_state_on_starknet);

        assert_eq!(&client_state_on_starknet.chain_id, cosmos_chain.chain_id());

        let client_state_on_cosmos = cosmos_chain
            .query_client_state_with_latest_height(PhantomData::<StarknetChain>, &cosmos_client_id)
            .await?;

        info!("client state on Cosmos: {:?}", client_state_on_cosmos);

        assert_eq!(
            &client_state_on_cosmos.client_state.chain_id,
            starknet_chain.chain_id()
        );

        let starknet_to_cosmos_relay = StarknetToCosmosRelay::new(
            runtime.clone(),
            starknet_chain.clone(),
            cosmos_chain.clone(),
            starknet_client_id.clone(),
            cosmos_client_id.clone(),
        );

        let cosmos_to_starknet_relay = CosmosToStarknetRelay::new(
            runtime.clone(),
            cosmos_chain.clone(),
            starknet_chain.clone(),
            cosmos_client_id.clone(),
            starknet_client_id.clone(),
        );

        // connection handshake

        let conn_init_option = CosmosInitConnectionOptions {
            delay_period: Duration::from_secs(0),
            connection_version: IbcConnectionVersion::compatibles().first().unwrap().clone(),
        };

        let (starknet_connection_id, cosmos_connection_id) = starknet_to_cosmos_relay
            .bootstrap_connection(&conn_init_option)
            .await?;

        info!("starknet_connection_id: {:?}", starknet_connection_id);
        info!("cosmos_connection_id: {:?}", cosmos_connection_id);

        // channel handshake

        let ics20_port = IbcPortId::transfer();

        let init_channel_options = CosmosInitChannelOptions::new(starknet_connection_id);

        let (starknet_channel_id, cosmos_channel_id) = starknet_to_cosmos_relay
            .bootstrap_channel(
                &ics20_port.clone(),
                &ics20_port.clone(),
                &init_channel_options,
            )
            .await?;

        info!("starknet_channel_id: {:?}", starknet_channel_id);
        info!("cosmos_channel_id: {:?}", cosmos_channel_id);

        // First ics20 transfer to Cosmos

        let wallet_cosmos_a = &cosmos_chain_driver.user_wallet_a;
        let address_cosmos_a = &wallet_cosmos_a.address;
        let wallet_starknet_b = &starknet_chain_driver.user_wallet_b;
        let address_starknet_b = &wallet_starknet_b.account_address;
        let transfer_quantity = 1_000u128;
        let transfer_back_quantity = 310u128;
        let denom_cosmos = &cosmos_chain_driver.genesis_config.transfer_denom;

        let starknet_account_b = SingleOwnerAccount::new(
            starknet_chain.rpc_client.clone(),
            LocalWallet::from_signing_key(SigningKey::from_secret_scalar(
                wallet_starknet_b.signing_key,
            )),
            *wallet_starknet_b.account_address,
            starknet_chain.rpc_client.chain_id().await?,
            ExecutionEncoding::New,
        );

        let balance_cosmos_a_step_0 = cosmos_chain
            .query_balance(address_cosmos_a, denom_cosmos)
            .await?;

        info!(
            "cosmos balance before transfer: {}",
            balance_cosmos_a_step_0
        );

        let packet = <CosmosChain as CanIbcTransferToken<StarknetChain>>::ibc_transfer_token(
            cosmos_chain,
            &cosmos_channel_id,
            &IbcPortId::transfer(),
            wallet_cosmos_a,
            address_starknet_b,
            &Amount::new(transfer_quantity, denom_cosmos.clone()),
            &None,
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
                    port_id: IbcPortId::transfer().to_string(),
                    channel_id: starknet_channel_id.to_string(),
                }],
                base: Denom::Hosted(denom_cosmos.to_string()),
            };

            let mut denom_serialized = vec![];

            {
                // https://github.com/informalsystems/ibc-starknet/blob/06cb7587557e6f3bef323abe7b5d9c3ab35bd97a/cairo-contracts/packages/apps/src/transfer/types.cairo#L120-L130
                for trace_prefix in &ibc_prefixed_denom.trace_path {
                    denom_serialized.extend(cairo_encoding.encode(trace_prefix)?);
                }

                denom_serialized.extend(cairo_encoding.encode(&ibc_prefixed_denom.base)?);
            }

            // https://github.com/informalsystems/ibc-starknet/blob/06cb7587557e6f3bef323abe7b5d9c3ab35bd97a/cairo-contracts/packages/utils/src/utils.cairo#L35
            let ibc_prefixed_denom_key = Poseidon3Hasher::digest(&denom_serialized);

            let calldata = cairo_encoding.encode(&product![ibc_prefixed_denom_key])?;

            let output = starknet_chain
                .call_contract(
                    &ics20_contract_address,
                    &selector!("ibc_token_address"),
                    &calldata,
                    None,
                )
                .await?;

            cairo_encoding.decode(&output)?
        };

        let birelay = StarknetCosmosBiRelay {
            runtime: runtime.clone(),
            relay_a_to_b: starknet_to_cosmos_relay.clone(),
            relay_b_to_a: cosmos_to_starknet_relay.clone(),
        };

        birelay
            .auto_bi_relay(Some(Duration::from_secs(10)), Some(Duration::from_secs(0)))
            .await?;

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
            "starknet balance before transfer: {} from token_address: {ics20_token_address} and account_address {address_starknet_b}",
            balance_starknet_b_step_0.quantity
        );

        // ### SETUP DONE ###

        // ### SETUP PENDING PACKETS AND ACKS ###

        // Create Cosmos to Starknet transfer
        let packet = <CosmosChain as CanIbcTransferToken<StarknetChain>>::ibc_transfer_token(
            cosmos_chain,
            &cosmos_channel_id,
            &IbcPortId::transfer(),
            wallet_cosmos_a,
            address_starknet_b,
            &Amount::new(transfer_quantity, denom_cosmos.clone()),
            &None,
        )
        .await?;

        runtime.sleep(Duration::from_secs(2)).await;

        info!("Will relay only relay the recv from Cosmos to Starknet");

        relay_only_send(&cosmos_to_starknet_relay, &packet).await?;

        runtime.sleep(Duration::from_secs(2)).await;

        // approve ics20 contract to spend the tokens for `address_starknet_b`
        {
            let call_data = cairo_encoding.encode(&product![
                ics20_contract_address,
                U256::from(transfer_back_quantity*2)
            ])?;

            let call = Call {
                to: *ics20_token_address,
                selector: selector!("approve"),
                calldata: call_data,
            };

            let _ = starknet_chain
                .send_messages_with_signer(wallet_starknet_b, &[StarknetMessage::new(call)])
                .await?;
        }

        // Create Starknet to Cosmos transfer
        let amount_back = StarknetAmount {
            quantity: transfer_back_quantity.into(),
            token_address: balance_starknet_b_step_0.token_address,
        };

        let packet = <StarknetChain as CanIbcTransferToken<CosmosChain>>::ibc_transfer_token(
            starknet_chain,
            &starknet_channel_id,
            &IbcPortId::transfer(),
            wallet_starknet_b,
            address_cosmos_a,
            &amount_back,
            &None,
        )
        .await?;

        runtime.sleep(Duration::from_secs(2)).await;

        info!("Will relay only relay the recv from Starknet to Cosmos");

        relay_only_send(&starknet_to_cosmos_relay, &packet).await?;

        runtime.sleep(Duration::from_secs(2)).await;

        let _packet = <StarknetChain as CanIbcTransferToken<CosmosChain>>::ibc_transfer_token(
            starknet_chain,
            &starknet_channel_id,
            &IbcPortId::transfer(),
            wallet_starknet_b,
            address_cosmos_a,
            &amount_back,
            &None,
        )
        .await?;

        // Create a pending packet
        let _packet = <CosmosChain as CanIbcTransferToken<StarknetChain>>::ibc_transfer_token(
            cosmos_chain,
            &cosmos_channel_id,
            &IbcPortId::transfer(),
            wallet_cosmos_a,
            address_starknet_b,
            &Amount::new(transfer_quantity, denom_cosmos.clone()),
            &None,
        )
        .await?;

        let eventual_cosmos_amount = &Amount::new(
            balance_cosmos_a_step_0.quantity - (transfer_quantity * 2) + (transfer_back_quantity * 2),
            denom_cosmos.clone(),
        );

        // Assert the following packets have not yet been cleared:
        //   * Cosmos to Starknet sequences 2 & 3
        //   * Starknet to Cosmos sequences 1 & 2
        let cosmos_latest_height = cosmos_chain.query_chain_height().await?;
        let starknet_latest_height = starknet_chain.query_chain_height().await?;

        let (cosmos_commitment2, _) =
            <CosmosChain as CanQueryPacketCommitment<StarknetChain>>::query_packet_commitment(
                cosmos_chain,
                &cosmos_channel_id,
                &IbcPortId::transfer(),
                &Sequence::from(2),
                &cosmos_latest_height,
            )
            .await?;

        let (cosmos_commitment3, _) =
            <CosmosChain as CanQueryPacketCommitment<StarknetChain>>::query_packet_commitment(
                cosmos_chain,
                &cosmos_channel_id,
                &IbcPortId::transfer(),
                &Sequence::from(3),
                &cosmos_latest_height,
            )
            .await?;

        let (starknet_commitment1, _) =
            <StarknetChain as CanQueryPacketCommitment<CosmosChain>>::query_packet_commitment(
                starknet_chain,
                &starknet_channel_id,
                &IbcPortId::transfer(),
                &Sequence::from(1),
                &starknet_latest_height,
            )
            .await?;

        let (starknet_commitment2, _) =
            <StarknetChain as CanQueryPacketCommitment<CosmosChain>>::query_packet_commitment(
                starknet_chain,
                &starknet_channel_id,
                &IbcPortId::transfer(),
                &Sequence::from(2),
                &starknet_latest_height,
            )
            .await?;

        assert!(
            cosmos_commitment2.is_some(),
            "cosmos_commitment2 expected to be Some"
        );
        assert!(
            cosmos_commitment3.is_some(),
            "cosmos_commitment3 expected to be Some"
        );
        assert!(
            starknet_commitment1.is_some(),
            "starknet_commitment1 expected to be Some"
        );
        assert!(
            starknet_commitment2.is_some(),
            "starknet_commitment2 expected to be Some"
        );

        birelay
            .auto_bi_relay(Some(Duration::from_secs(30)), Some(Duration::from_secs(0)))
            .await?;

        cosmos_chain
            .assert_eventual_amount(address_cosmos_a, eventual_cosmos_amount)
            .await?;

        let balance_starknet_b_step_2 = starknet_chain
            .query_token_balance(&ics20_token_address, address_starknet_b)
            .await?;

        assert_eq!(
            balance_starknet_b_step_0.quantity + (transfer_quantity * 2).into()
                - (transfer_back_quantity * 2).into(),
            balance_starknet_b_step_2.quantity
        );

        // Assert all packets have been cleared after auto relaying
        let cosmos_latest_height = cosmos_chain.query_chain_height().await?;
        let starknet_latest_height = starknet_chain.query_chain_height().await?;

        let (cosmos_commitment1, _) =
            <CosmosChain as CanQueryPacketCommitment<StarknetChain>>::query_packet_commitment(
                cosmos_chain,
                &cosmos_channel_id,
                &IbcPortId::transfer(),
                &Sequence::from(1),
                &cosmos_latest_height,
            )
            .await?;

        let (cosmos_commitment2, _) =
            <CosmosChain as CanQueryPacketCommitment<StarknetChain>>::query_packet_commitment(
                cosmos_chain,
                &cosmos_channel_id,
                &IbcPortId::transfer(),
                &Sequence::from(2),
                &cosmos_latest_height,
            )
            .await?;

        let (cosmos_commitment3, _) =
            <CosmosChain as CanQueryPacketCommitment<StarknetChain>>::query_packet_commitment(
                cosmos_chain,
                &cosmos_channel_id,
                &IbcPortId::transfer(),
                &Sequence::from(3),
                &cosmos_latest_height,
            )
            .await?;

        let (starknet_commitment1, _) =
            <StarknetChain as CanQueryPacketCommitment<CosmosChain>>::query_packet_commitment(
                starknet_chain,
                &starknet_channel_id,
                &IbcPortId::transfer(),
                &Sequence::from(1),
                &starknet_latest_height,
            )
            .await?;

        let (starknet_commitment2, _) =
            <StarknetChain as CanQueryPacketCommitment<CosmosChain>>::query_packet_commitment(
                starknet_chain,
                &starknet_channel_id,
                &IbcPortId::transfer(),
                &Sequence::from(2),
                &starknet_latest_height,
            )
            .await?;

        assert!(
            cosmos_commitment1.is_none(),
            "cosmos_commitment1 expected to be None"
        );
        assert!(
            cosmos_commitment2.is_none(),
            "cosmos_commitment2 expected to be None"
        );
        assert!(
            cosmos_commitment3.is_none(),
            "cosmos_commitment3 expected to be None"
        );
        assert!(
            starknet_commitment1.is_none(),
            "starknet_commitment1 expected to be None"
        );
        assert!(
            starknet_commitment2.is_none(),
            "starknet_commitment2 expected to be None"
        );

        Ok(())
    })
}

#[test]
fn test_relay_timeout_packet() -> Result<(), Error> {
    // ### SETUP START ###
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let wasm_client_code_path = std::env::var("STARKNET_WASM_CLIENT_PATH")
            .expect("Wasm blob for Starknet light client is required");

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let starknet_bootstrap = init_starknet_bootstrap(&runtime).await?;

        let (wasm_code_hash, wasm_client_byte_code) =
            load_wasm_client(&wasm_client_code_path).await?;

        let cosmos_builder = CosmosBuilder::new_with_default(runtime.clone());

        let cosmos_bootstrap = Arc::new(OsmosisBootstrap {
            runtime: runtime.clone(),
            cosmos_builder,
            should_randomize_identifiers: true,
            chain_store_dir: format!("./test-data/{timestamp}/osmosis").into(),
            chain_command_path: "osmosisd".into(),
            account_prefix: "osmo".into(),
            staking_denom_prefix: "stake".into(),
            transfer_denom_prefix: "coin".into(),
            wasm_client_byte_code,
            governance_proposal_authority: "osmo10d07y265gmmuvt4z0w9aw880jnsr700jjeq4qp".into(), // TODO: don't hard code this
            dynamic_gas: Some(DynamicGasConfig {
                multiplier: 1.1,
                max: 1.6,
                eip_query_type: EipQueryType::Osmosis,
                denom: "stake".to_owned(),
            }),
        });

        let mut starknet_chain_driver = starknet_bootstrap.bootstrap_chain("starknet").await?;

        let starknet_chain = &mut starknet_chain_driver.chain;

        info!(
            "started starknet chain at port {}",
            starknet_chain_driver.node_config.rpc_port
        );

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

        let client_state_on_starknet = starknet_chain
            .query_client_state_with_latest_height(PhantomData::<CosmosChain>, &starknet_client_id)
            .await?;

        info!("client state on Starknet: {:?}", client_state_on_starknet);

        assert_eq!(&client_state_on_starknet.chain_id, cosmos_chain.chain_id());

        let client_state_on_cosmos = cosmos_chain
            .query_client_state_with_latest_height(PhantomData::<StarknetChain>, &cosmos_client_id)
            .await?;

        info!("client state on Cosmos: {:?}", client_state_on_cosmos);

        assert_eq!(
            &client_state_on_cosmos.client_state.chain_id,
            starknet_chain.chain_id()
        );

        let starknet_to_cosmos_relay = StarknetToCosmosRelay::new(
            runtime.clone(),
            starknet_chain.clone(),
            cosmos_chain.clone(),
            starknet_client_id.clone(),
            cosmos_client_id.clone(),
        );

        let cosmos_to_starknet_relay = CosmosToStarknetRelay::new(
            runtime.clone(),
            cosmos_chain.clone(),
            starknet_chain.clone(),
            cosmos_client_id.clone(),
            starknet_client_id.clone(),
        );

        // connection handshake

        let conn_init_option = CosmosInitConnectionOptions {
            delay_period: Duration::from_secs(0),
            connection_version: IbcConnectionVersion::compatibles().first().unwrap().clone(),
        };

        let (starknet_connection_id, cosmos_connection_id) = starknet_to_cosmos_relay
            .bootstrap_connection(&conn_init_option)
            .await?;

        info!("starknet_connection_id: {:?}", starknet_connection_id);
        info!("cosmos_connection_id: {:?}", cosmos_connection_id);

        // channel handshake

        let ics20_port = IbcPortId::transfer();

        let init_channel_options = CosmosInitChannelOptions::new(starknet_connection_id);

        let (starknet_channel_id, cosmos_channel_id) = starknet_to_cosmos_relay
            .bootstrap_channel(
                &ics20_port.clone(),
                &ics20_port.clone(),
                &init_channel_options,
            )
            .await?;

        info!("starknet_channel_id: {:?}", starknet_channel_id);
        info!("cosmos_channel_id: {:?}", cosmos_channel_id);

        // First ics20 transfer to Cosmos

        let wallet_cosmos_a = &cosmos_chain_driver.user_wallet_a;
        let address_cosmos_a = &wallet_cosmos_a.address;
        let wallet_starknet_b = &starknet_chain_driver.user_wallet_b;
        let address_starknet_b = &wallet_starknet_b.account_address;
        let transfer_quantity = 1_000u128;
        let denom_cosmos = &cosmos_chain_driver.genesis_config.transfer_denom;

        let starknet_account_b = SingleOwnerAccount::new(
            starknet_chain.rpc_client.clone(),
            LocalWallet::from_signing_key(SigningKey::from_secret_scalar(
                wallet_starknet_b.signing_key,
            )),
            *wallet_starknet_b.account_address,
            starknet_chain.rpc_client.chain_id().await?,
            ExecutionEncoding::New,
        );

        let balance_cosmos_a_step_0 = cosmos_chain
            .query_balance(address_cosmos_a, denom_cosmos)
            .await?;

        info!(
            "cosmos balance before transfer: {}",
            balance_cosmos_a_step_0
        );

        let packet = <CosmosChain as CanIbcTransferToken<StarknetChain>>::ibc_transfer_token(
            cosmos_chain,
            &cosmos_channel_id,
            &IbcPortId::transfer(),
            wallet_cosmos_a,
            address_starknet_b,
            &Amount::new(transfer_quantity, denom_cosmos.clone()),
            &None,
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
                    port_id: IbcPortId::transfer().to_string(),
                    channel_id: starknet_channel_id.to_string(),
                }],
                base: Denom::Hosted(denom_cosmos.to_string()),
            };

            let mut denom_serialized = vec![];

            {
                // https://github.com/informalsystems/ibc-starknet/blob/06cb7587557e6f3bef323abe7b5d9c3ab35bd97a/cairo-contracts/packages/apps/src/transfer/types.cairo#L120-L130
                for trace_prefix in &ibc_prefixed_denom.trace_path {
                    denom_serialized.extend(cairo_encoding.encode(trace_prefix)?);
                }

                denom_serialized.extend(cairo_encoding.encode(&ibc_prefixed_denom.base)?);
            }

            // https://github.com/informalsystems/ibc-starknet/blob/06cb7587557e6f3bef323abe7b5d9c3ab35bd97a/cairo-contracts/packages/utils/src/utils.cairo#L35
            let ibc_prefixed_denom_key = Poseidon3Hasher::digest(&denom_serialized);

            let calldata = cairo_encoding.encode(&product![ibc_prefixed_denom_key])?;

            let output = starknet_chain
                .call_contract(
                    &ics20_contract_address,
                    &selector!("ibc_token_address"),
                    &calldata,
                    None,
                )
                .await?;

            cairo_encoding.decode(&output)?
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

        let birelay = StarknetCosmosBiRelay {
            runtime: runtime.clone(),
            relay_a_to_b: starknet_to_cosmos_relay,
            relay_b_to_a: cosmos_to_starknet_relay,
        };

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

        // Create Cosmos to Starknet transfer
        let packet = <CosmosChain as CanIbcTransferToken<StarknetChain>>::ibc_transfer_token(
            cosmos_chain,
            &cosmos_channel_id,
            &IbcPortId::transfer(),
            wallet_cosmos_a,
            address_starknet_b,
            &Amount::new(transfer_quantity, denom_cosmos.clone()),
            &None,
        )
        .await?;

        // approve ics20 contract to spend the tokens for `address_starknet_b`
        {
            let call_data = cairo_encoding.encode(&product![
                ics20_contract_address,
                U256::from(transfer_quantity)
            ])?;

            let call = Call {
                to: *ics20_token_address,
                selector: selector!("approve"),
                calldata: call_data,
            };

            let _ = starknet_chain
                .send_messages_with_signer(wallet_starknet_b, &[StarknetMessage::new(call)])
                .await?;
        }

        // Create Starknet to Cosmos transfer
        let starknet_ics20_send_message = {
            let current_starknet_time = starknet_chain.query_chain_status().await?.time;

            let denom = PrefixedDenom {
                trace_path: vec![TracePrefix {
                    port_id: ics20_port.to_string(),
                    channel_id: starknet_channel_id.to_string(),
                }],
                base: Denom::Hosted(denom_cosmos.to_string()),
            };

            MsgTransfer {
                port_id_on_a: ics20_port.clone(),
                chan_id_on_a: starknet_channel_id.clone(),
                denom,
                amount: transfer_quantity.into(),
                receiver: address_cosmos_a.clone(),
                memo: String::new(),
                timeout_height_on_b: Height {
                    revision_number: 0,
                    revision_height: 0,
                },
                timeout_timestamp_on_b: Timestamp::from_nanoseconds(
                    u64::try_from(current_starknet_time.unix_timestamp() + 90).unwrap()
                        * 1_000_000_000,
                ),
            }
        };

        // submit to ics20 contract
        {
            let call_data = cairo_encoding.encode(&starknet_ics20_send_message)?;

            let call = Call {
                to: *ics20_contract_address,
                selector: selector!("send_transfer"),
                calldata: call_data,
            };

            let _ = starknet_chain
                .send_messages_with_signer(wallet_starknet_b, &[StarknetMessage::new(call)])
                .await?;
        };

        runtime.sleep(Duration::from_secs(5)).await;

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

        runtime.sleep(Duration::from_secs(90)).await;

        info!("will relay timeout packets");

        birelay
            .auto_bi_relay(Some(Duration::from_secs(120)), Some(Duration::from_secs(0)))
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

async fn relay_only_send<Relay, SrcChain, DstChain>(
    relay: &Relay,
    packet: &Relay::Packet,
) -> Result<(), Relay::Error>
where
    Relay: CanRelayReceivePacket
        + CanRelayTimeoutUnorderedPacket
        + HasRelayPacketType
        + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + CanRaiseAsyncError<SrcChain::Error>
        + CanRaiseAsyncError<DstChain::Error>,
    SrcChain: CanQueryChainStatus
        + HasPacketTimeoutHeight<DstChain>
        + HasPacketTimeoutTimestamp<DstChain>,
    DstChain: CanQueryChainStatus,
{
    let src_chain = relay.src_chain();
    let dst_chain = relay.dst_chain();

    let destination_status = dst_chain
        .query_chain_status()
        .await
        .map_err(Relay::raise_error)?;

    let destination_height = DstChain::chain_status_height(&destination_status);
    let destination_timestamp = DstChain::chain_status_time(&destination_status);

    let packet_timeout_height = SrcChain::packet_timeout_height(packet);
    let packet_timeout_timestamp = SrcChain::packet_timeout_timestamp(packet);

    let has_packet_timed_out = match (packet_timeout_height, packet_timeout_timestamp) {
        (Some(height), Some(timestamp)) => {
            destination_height > &height
                || DstChain::has_timed_out(destination_timestamp, &timestamp)
        }
        (Some(height), None) => destination_height > &height,
        (None, Some(timestamp)) => DstChain::has_timed_out(destination_timestamp, &timestamp),
        (None, None) => {
            // TODO: raise error?
            false
        }
    };

    if has_packet_timed_out {
        relay
            .relay_timeout_unordered_packet(destination_height, packet)
            .await?;
        Ok(())
    } else {
        let src_chain_status = src_chain
            .query_chain_status()
            .await
            .map_err(Relay::raise_error)?;

        let _write_ack = relay
            .relay_receive_packet(
                Relay::SrcChain::chain_status_height(&src_chain_status),
                packet,
            )
            .await?;
        Ok(())
    }
}
