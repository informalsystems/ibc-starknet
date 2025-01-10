use alloc::sync::Arc;
use core::marker::PhantomData;
use core::time::Duration;
use std::path::PathBuf;
use std::time::SystemTime;

use cgp::prelude::*;
use hermes_chain_components::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::chain::impls::transfer::amount::derive_ibc_denom;
use hermes_cosmos_test_components::chain::types::amount::Amount;
use hermes_cosmos_test_components::chain::types::denom::Denom as IbcDenom;
use hermes_cosmos_wasm_relayer::context::cosmos_bootstrap::CosmosWithWasmClientBootstrap;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::packet_relayer::CanRelayPacket;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_starknet_chain_components::impls::encoding::events::CanFilterDecodeEvents;
use hermes_starknet_chain_components::impls::types::message::StarknetMessage;
use hermes_starknet_chain_components::traits::contract::call::CanCallContract;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::traits::queries::token_balance::CanQueryTokenBalance;
use hermes_starknet_chain_components::types::cosmos::height::Height;
use hermes_starknet_chain_components::types::cosmos::timestamp::Timestamp;
use hermes_starknet_chain_components::types::events::ics20::IbcTransferEvent;
use hermes_starknet_chain_components::types::events::packet::PacketRelayEvents;
use hermes_starknet_chain_components::types::messages::erc20::deploy::DeployErc20TokenMessage;
use hermes_starknet_chain_components::types::messages::ibc::channel::PortId;
use hermes_starknet_chain_components::types::messages::ibc::denom::{
    Denom, PrefixedDenom, TracePrefix,
};
use hermes_starknet_chain_components::types::messages::ibc::ibc_transfer::{
    MsgTransfer, Participant, TransferPacketData,
};
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_components::types::register::{MsgRegisterApp, MsgRegisterClient};
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use hermes_starknet_chain_context::contexts::encoding::event::StarknetEventEncoding;
use hermes_starknet_relayer::contexts::cosmos_to_starknet_relay::CosmosToStarknetRelay;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;
use ibc::apps::transfer::types::packet::PacketData;
use ibc::apps::transfer::types::PrefixedCoin;
use ibc::core::channel::types::packet::Packet as IbcPacket;
use ibc::core::channel::types::timeout::{TimeoutHeight, TimeoutTimestamp};
use ibc::core::client::types::Height as IbcHeight;
use ibc::core::connection::types::version::Version as IbcConnectionVersion;
use ibc::core::host::types::identifiers::{ConnectionId, PortId as IbcPortId};
use ibc::primitives::Timestamp as IbcTimestamp;
use sha2::{Digest, Sha256};
use starknet::accounts::Call;
use starknet::core::types::{Felt, U256};
use starknet::macros::{selector, short_string};
use tracing::info;

use crate::contexts::bootstrap::StarknetBootstrap;

#[test]
fn test_starknet_ics20_contract() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let chain_command_path = std::env::var("STARKNET_BIN")
            .unwrap_or("starknet-devnet".into())
            .into();

        let wasm_client_code_path = PathBuf::from(
            std::env::var("STARKNET_WASM_CLIENT_PATH")
                .expect("Wasm blob for Starknet light client is required"),
        );

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let starknet_bootstrap = StarknetBootstrap {
            runtime: runtime.clone(),
            chain_command_path,
            chain_store_dir: format!("./test-data/{timestamp}").into(),
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

        let erc20_class_hash = {
            let contract_path = std::env::var("ERC20_CONTRACT")?;

            let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

            let contract = serde_json::from_str(&contract_str)?;

            let class_hash = starknet_chain.declare_contract(&contract).await?;

            info!("declared ERC20 class: {:?}", class_hash);

            class_hash
        };

        let ics20_class_hash = {
            let contract_path = std::env::var("ICS20_CONTRACT")?;

            let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

            let contract = serde_json::from_str(&contract_str)?;

            let class_hash = starknet_chain.declare_contract(&contract).await?;

            info!("declared ICS20 class: {:?}", class_hash);

            class_hash
        };

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

        starknet_chain.ibc_core_contract_address = Some(ibc_core_address);
        starknet_chain.ibc_client_contract_address = Some(comet_client_address);

        let cairo_encoding = StarknetCairoEncoding;

        let event_encoding = StarknetEventEncoding {
            erc20_hashes: [erc20_class_hash].into(),
            ics20_hashes: [ics20_class_hash].into(),
            ibc_client_hashes: [comet_client_class_hash].into(),
            ibc_core_hashes: [ibc_core_class_hash].into(),
        };

        {
            // register comet client contract with ibc-core

            let register_client = MsgRegisterClient {
                client_type: short_string!("07-tendermint"),
                contract_address: comet_client_address,
            };

            let calldata = cairo_encoding.encode(&register_client)?;

            let call = Call {
                to: ibc_core_address,
                selector: selector!("register_client"),
                calldata,
            };

            let message = StarknetMessage::new(call);

            let response = starknet_chain.send_message(message).await?;

            info!("IBC register client response: {:?}", response);
        }

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

        let client_state_on_cosmos = cosmos_chain
            .query_client_state_with_latest_height(PhantomData::<StarknetChain>, &cosmos_client_id)
            .await?;

        info!("client state on Cosmos: {:?}", client_state_on_cosmos);

        let ics20_contract_address = {
            let owner_call_data = cairo_encoding.encode(&ibc_core_address)?;
            let erc20_call_data = cairo_encoding.encode(&erc20_class_hash)?;

            let contract_address = starknet_chain
                .deploy_contract(
                    &ics20_class_hash,
                    false,
                    &[owner_call_data, erc20_call_data].concat(),
                )
                .await?;

            info!("deployed ICS20 contract to address: {:?}", contract_address);

            contract_address
        };

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

        {
            // register the ICS20 contract with the IBC core contract

            let port_id_on_starknet = PortId {
                port_id: ics20_port.to_string(),
            };

            let register_app = MsgRegisterApp {
                port_id: port_id_on_starknet.clone(),
                contract_address: ics20_contract_address,
            };

            let register_call_data = cairo_encoding.encode(&register_app)?;

            let call = Call {
                to: ibc_core_address,
                selector: selector!("bind_port_id"),
                calldata: register_call_data,
            };

            let message = StarknetMessage::new(call);

            let response = starknet_chain.send_message(message).await?;

            info!("register ics20 response: {:?}", response);
        }

        let starknet_connection_id_seq = starknet_connection_id
            .connection_id
            .strip_prefix("connection-")
            .unwrap()
            .parse::<u64>()?;

        let init_channel_options =
            CosmosInitChannelOptions::new(ConnectionId::new(starknet_connection_id_seq));

        let (starknet_channel_id, cosmos_channel_id) = starknet_to_cosmos_relay
            .bootstrap_channel(
                &ics20_port.clone(),
                &ics20_port.clone(),
                &init_channel_options,
            )
            .await?;

        info!("starknet_channel_id: {:?}", starknet_channel_id);
        info!("cosmos_channel_id: {:?}", cosmos_channel_id);

        // submit ics20 transfer to Cosmos

        let wallet_cosmos_a = &cosmos_chain_driver.user_wallet_a;
        let address_cosmos_a = &wallet_cosmos_a.address;
        let wallet_starknet_b = &starknet_chain_driver.user_wallet_b;
        let address_starknet_b = &wallet_starknet_b.account_address;
        let transfer_quantity = 1_000u128;
        let denom_cosmos = &cosmos_chain_driver.genesis_config.transfer_denom;

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

        // TODO(rano): how do I get the ics20 token contract address from starknet events
        cosmos_to_starknet_relay.relay_packet(&packet).await?;

        let balance_cosmos_a_step_1 = cosmos_chain
            .query_balance(address_cosmos_a, denom_cosmos)
            .await?;

        info!("cosmos balance after transfer: {}", balance_cosmos_a_step_1);

        assert_eq!(
            balance_cosmos_a_step_0.quantity,
            balance_cosmos_a_step_1.quantity + transfer_quantity
        );

        // TODO(rano): we should use the poseidon hash of the ibc denom to get the token address

        let ics20_token_address = {
            let output = starknet_chain
                .call_contract(
                    &ics20_contract_address,
                    &selector!("ibc_token_addresses"),
                    &vec![],
                )
                .await?;

            let addresses: Vec<Felt> = cairo_encoding.decode(&output)?;

            assert_eq!(addresses.len(), 1);

            addresses[0]
        };

        info!("ics20 token address: {:?}", ics20_token_address);

        let balance_starknet_b_step_1 = starknet_chain
            .query_token_balance(&ics20_token_address, address_starknet_b)
            .await?;

        info!(
            "starknet balance after transfer: {}",
            balance_starknet_b_step_1
        );

        assert_eq!(balance_starknet_b_step_1.quantity, transfer_quantity.into());

        // TODO(rano): send back the ics20 token to cosmos

        // create ibc transfer packet data

        let starknet_ic20_packet_data = {
            let denom = PrefixedDenom {
                trace_path: vec![TracePrefix {
                    port_id: ics20_port.to_string(),
                    channel_id: starknet_channel_id.channel_id.clone(),
                }],
                base: Denom::Hosted(denom_cosmos.to_string()),
            };

            let amount = transfer_quantity.into();

            let sender = Participant::Native(*address_starknet_b);

            let receiver = Participant::External(address_cosmos_a.clone());

            let memo = String::new();

            TransferPacketData {
                denom,
                amount,
                sender,
                receiver,
                memo,
            }
        };

        // create ibc transfer message

        let starknet_ics20_send_message = {
            let current_starknet_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs();

            MsgTransfer {
                port_id_on_a: PortId {
                    port_id: ics20_port.to_string(),
                },
                chan_id_on_a: starknet_channel_id.clone(),
                packet_data: starknet_ic20_packet_data,
                timeout_height_on_b: Height {
                    revision_number: 0,
                    revision_height: 0,
                },
                timeout_timestamp_on_b: Timestamp {
                    timestamp: current_starknet_time + 1800,
                },
            }
        };

        // submit to ics20 contract

        let (send_packet_event, send_ics20_event) = {
            let call_data = cairo_encoding.encode(&starknet_ics20_send_message)?;

            let call = Call {
                to: ics20_contract_address,
                selector: selector!("send_transfer"),
                calldata: call_data,
            };

            let message = StarknetMessage::new(call);

            let response = starknet_chain.send_message(message).await?;

            info!("ICS20 send packet response: {:?}", response);

            let mut ibc_packet_events: Vec<PacketRelayEvents> =
                event_encoding.filter_decode_events(&response.events)?;

            info!("IBC packet events: {:?}", ibc_packet_events);

            let mut ibc_transfer_events: Vec<IbcTransferEvent> =
                event_encoding.filter_decode_events(&response.events)?;

            info!("IBC transfer events: {:?}", ibc_transfer_events);

            assert_eq!(ibc_packet_events.len(), 1);
            assert_eq!(ibc_transfer_events.len(), 1);

            let Some(PacketRelayEvents::Send(send_packet_event)) = ibc_packet_events.pop() else {
                panic!("expected send packet event");
            };

            let Some(IbcTransferEvent::Send(send_ics20_event)) = ibc_transfer_events.pop() else {
                panic!("expected send ics20 event");
            };

            (send_packet_event, send_ics20_event)
        };

        // create ibc packet

        let starknet_ibc_packet = {
            let timeout_height_on_b = IbcHeight::new(
                send_packet_event.timeout_height_on_b.revision_number,
                send_packet_event.timeout_height_on_b.revision_height,
            )
            .map(TimeoutHeight::At)
            .unwrap_or_else(|_| TimeoutHeight::Never);

            let timeout_timestamp_on_b = (send_packet_event.timeout_timestamp_on_b.timestamp > 0)
                .then(|| {
                    TimeoutTimestamp::At(IbcTimestamp::from_nanoseconds(
                        send_packet_event.timeout_timestamp_on_b.timestamp * 1_000_000_000,
                    ))
                })
                .unwrap_or(TimeoutTimestamp::Never);

            let ibc_transfer_packet_data = PacketData {
                token: format!("{}{}", send_ics20_event.amount, send_ics20_event.denom).parse()?,
                sender: send_ics20_event.sender.to_string().into(),
                receiver: send_ics20_event.receiver.to_string().into(),
                memo: send_ics20_event.memo.into(),
            };

            IbcPacket {
                seq_on_a: send_packet_event.sequence_on_a.sequence.into(),
                port_id_on_a: send_packet_event.port_id_on_a.port_id.parse()?,
                chan_id_on_a: send_packet_event.channel_id_on_a.channel_id.parse()?,
                port_id_on_b: send_packet_event.port_id_on_b.port_id.parse()?,
                chan_id_on_b: send_packet_event.channel_id_on_b.channel_id.parse()?,
                data: serde_json::to_vec(&ibc_transfer_packet_data).unwrap(),
                timeout_height_on_b,
                timeout_timestamp_on_b,
            }
        };

        // relay the packet via starknet to cosmos relay

        starknet_to_cosmos_relay
            .relay_packet(&starknet_ibc_packet)
            .await?;

        let balance_cosmos_a_step_2 = cosmos_chain
            .query_balance(address_cosmos_a, denom_cosmos)
            .await?;

        info!(
            "cosmos balance after transfer back: {}",
            balance_cosmos_a_step_2
        );

        assert_eq!(
            balance_cosmos_a_step_0.quantity,
            balance_cosmos_a_step_2.quantity
        );

        let balance_starknet_b_step_2 = starknet_chain
            .query_token_balance(&ics20_token_address, address_starknet_b)
            .await?;

        info!(
            "starknet balance after transfer back: {}",
            balance_starknet_b_step_2
        );

        assert_eq!(balance_starknet_b_step_2.quantity, 0u64.into());

        // send starknet native token to cosmos

        let wallet_starknet_relayer = &starknet_chain_driver.relayer_wallet;
        let address_starknet_relayer = &wallet_starknet_relayer.account_address;

        let erc20_token_supply = 1_000_000_000u128;

        let erc20_token_address = {
            let deploy_message = DeployErc20TokenMessage {
                name: "token".into(),
                symbol: "token".into(),
                fixed_supply: erc20_token_supply.into(),
                recipient: *address_starknet_relayer,
                owner: *address_starknet_relayer,
            };

            let calldata = StarknetCairoEncoding.encode(&deploy_message)?;

            starknet_chain
                .deploy_contract(&erc20_class_hash, false, &calldata)
                .await?
        };

        info!(
            "deployed ERC20 contract to address: {:?}",
            erc20_token_address
        );

        let balance_starknet_relayer_step_0 = starknet_chain
            .query_token_balance(&erc20_token_address, address_starknet_relayer)
            .await?;

        info!("initial balance: {}", balance_starknet_relayer_step_0);

        assert_eq!(
            balance_starknet_relayer_step_0.quantity,
            erc20_token_supply.into()
        );

        // {
        //     let transfer_amount = transfer_quantity.into();

        //     let message = starknet_chain.build_transfer_token_message(
        //         address_starknet_b,
        //         &StarknetAmount::new(transfer_amount, erc20_token_address),
        //     )?;

        //     let response = starknet_chain.send_message(message).await?;

        //     info!("performed top-up of {transfer_quantity} tokens");

        //     info!("response: {:?}", response);

        //     let balance = starknet_chain
        //         .query_token_balance(&erc20_token_address, address_starknet_b)
        //         .await?;

        //     info!("top-up balance: {}", balance);

        //     assert_eq!(balance.quantity, transfer_quantity.into());
        // }

        {
            // approve ics20 contract to spend the tokens for address_starknet_b
            let call_data = cairo_encoding.encode(&product![
                ics20_contract_address,
                U256::from(transfer_quantity)
            ])?;

            let call = Call {
                to: erc20_token_address,
                selector: selector!("approve"),
                calldata: call_data,
            };

            let message = StarknetMessage::new(call);

            let response = starknet_chain.send_message(message).await?;

            info!("ERC20 approve response: {:?}", response);
        }

        // submit ics20 transfer from Starknet to Cosmos

        let starknet_ic20_packet_data = {
            let denom = PrefixedDenom {
                trace_path: vec![],
                base: Denom::Native(erc20_token_address),
            };

            let amount = transfer_quantity.into();

            let sender = Participant::Native(*address_starknet_relayer);

            let receiver = Participant::External(address_cosmos_a.clone());

            let memo = String::new();

            TransferPacketData {
                denom,
                amount,
                sender,
                receiver,
                memo,
            }
        };

        // create ibc transfer message

        let starknet_ics20_send_message = {
            let current_starknet_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs();

            MsgTransfer {
                port_id_on_a: PortId {
                    port_id: ics20_port.to_string(),
                },
                chan_id_on_a: starknet_channel_id.clone(),
                packet_data: starknet_ic20_packet_data,
                timeout_height_on_b: Height {
                    revision_number: 0,
                    revision_height: 0,
                },
                timeout_timestamp_on_b: Timestamp {
                    timestamp: current_starknet_time + 1800,
                },
            }
        };

        // submit to ics20 contract

        let (send_packet_event, send_ics20_event) = {
            let call_data = cairo_encoding.encode(&starknet_ics20_send_message)?;

            let call = Call {
                to: ics20_contract_address,
                selector: selector!("send_transfer"),
                calldata: call_data,
            };

            let message = StarknetMessage::new(call);

            let response = starknet_chain.send_message(message).await?;

            info!("ICS20 send packet response: {:?}", response);

            let mut ibc_packet_events: Vec<PacketRelayEvents> =
                event_encoding.filter_decode_events(&response.events)?;

            info!("IBC packet events: {:?}", ibc_packet_events);

            let mut ibc_transfer_events: Vec<IbcTransferEvent> =
                event_encoding.filter_decode_events(&response.events)?;

            info!("IBC transfer events: {:?}", ibc_transfer_events);

            assert_eq!(ibc_packet_events.len(), 1);
            assert_eq!(ibc_transfer_events.len(), 1);

            let Some(PacketRelayEvents::Send(send_packet_event)) = ibc_packet_events.pop() else {
                panic!("expected send packet event");
            };

            let Some(IbcTransferEvent::Send(send_ics20_event)) = ibc_transfer_events.pop() else {
                panic!("expected send ics20 event");
            };

            (send_packet_event, send_ics20_event)
        };

        // create ibc packet

        let starknet_ibc_packet = {
            let timeout_height_on_b = IbcHeight::new(
                send_packet_event.timeout_height_on_b.revision_number,
                send_packet_event.timeout_height_on_b.revision_height,
            )
            .map(TimeoutHeight::At)
            .unwrap_or_else(|_| TimeoutHeight::Never);

            let timeout_timestamp_on_b = (send_packet_event.timeout_timestamp_on_b.timestamp > 0)
                .then(|| {
                    TimeoutTimestamp::At(IbcTimestamp::from_nanoseconds(
                        send_packet_event.timeout_timestamp_on_b.timestamp * 1_000_000_000,
                    ))
                })
                .unwrap_or(TimeoutTimestamp::Never);

            let ibc_transfer_packet_data = PacketData {
                token: PrefixedCoin {
                    denom: send_ics20_event.denom.to_string().parse()?,
                    amount: send_ics20_event.amount.to_string().parse()?,
                },
                sender: send_ics20_event.sender.to_string().into(),
                receiver: send_ics20_event.receiver.to_string().into(),
                memo: send_ics20_event.memo.into(),
            };

            IbcPacket {
                seq_on_a: send_packet_event.sequence_on_a.sequence.into(),
                port_id_on_a: send_packet_event.port_id_on_a.port_id.parse()?,
                chan_id_on_a: send_packet_event.channel_id_on_a.channel_id.parse()?,
                port_id_on_b: send_packet_event.port_id_on_b.port_id.parse()?,
                chan_id_on_b: send_packet_event.channel_id_on_b.channel_id.parse()?,
                data: serde_json::to_vec(&ibc_transfer_packet_data).unwrap(),
                timeout_height_on_b,
                timeout_timestamp_on_b,
            }
        };

        // relay the packet via starknet to cosmos relay

        starknet_to_cosmos_relay
            .relay_packet(&starknet_ibc_packet)
            .await?;

        // query balances

        let cosmos_ibc_denom = derive_ibc_denom(
            &ics20_port,
            &cosmos_channel_id,
            &IbcDenom::base(&erc20_token_address.to_string()),
        )?;

        info!("cosmos ibc denom: {:?}", cosmos_ibc_denom);

        let balance_cosmos_a_step_3 = cosmos_chain
            .query_balance(address_cosmos_a, &cosmos_ibc_denom)
            .await?;

        info!(
            "cosmos balance after transfer from starknet: {}",
            balance_cosmos_a_step_3
        );

        assert_eq!(balance_cosmos_a_step_3.quantity, transfer_quantity);

        let balance_starknet_relayer_step_3 = starknet_chain
            .query_token_balance(&erc20_token_address, address_starknet_relayer)
            .await?;

        info!(
            "starknet balance after transfer from starknet: {}",
            balance_starknet_relayer_step_3
        );

        assert_eq!(
            balance_starknet_relayer_step_3.quantity,
            (erc20_token_supply - transfer_quantity).into()
        );

        // send the tokens back to starknet

        let packet = <CosmosChain as CanIbcTransferToken<StarknetChain>>::ibc_transfer_token(
            cosmos_chain,
            &cosmos_channel_id,
            &IbcPortId::transfer(),
            wallet_cosmos_a,
            address_starknet_relayer,
            &Amount::new(transfer_quantity, cosmos_ibc_denom.clone()),
            &None,
        )
        .await?;

        cosmos_to_starknet_relay.relay_packet(&packet).await?;

        let balance_cosmos_a_step_4 = cosmos_chain
            .query_balance(address_cosmos_a, &cosmos_ibc_denom)
            .await?;

        info!(
            "cosmos balance after transfer back to starknet: {}",
            balance_cosmos_a_step_4
        );

        assert_eq!(balance_cosmos_a_step_4.quantity, 0u64.into());

        let balance_starknet_relayer_step_4 = starknet_chain
            .query_token_balance(&erc20_token_address, address_starknet_relayer)
            .await?;

        info!(
            "starknet balance after transfer back to starknet: {}",
            balance_starknet_relayer_step_4
        );

        assert_eq!(
            balance_starknet_relayer_step_4.quantity,
            balance_starknet_relayer_step_0.quantity
        );

        Ok(())
    })
}
