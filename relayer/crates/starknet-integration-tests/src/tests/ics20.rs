use alloc::sync::Arc;
use core::marker::PhantomData;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use eyre::eyre;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_components::traits::queries::client_state::CanQueryClientState;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_wasm_relayer::context::cosmos_bootstrap::CosmosWithWasmClientBootstrap;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_starknet_chain_components::components::starknet_to_cosmos;
use hermes_starknet_chain_components::impls::encoding::events::CanFilterDecodeEvents;
use hermes_starknet_chain_components::impls::types::message::StarknetMessage;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::traits::queries::token_balance::CanQueryTokenBalance;
use hermes_starknet_chain_components::types::client_id::ClientId;
use hermes_starknet_chain_components::types::cosmos::height::Height;
use hermes_starknet_chain_components::types::events::channel::ChannelHandshakeEvents;
use hermes_starknet_chain_components::types::events::connection::ConnectionHandshakeEvents;
use hermes_starknet_chain_components::types::events::ics20::IbcTransferEvent;
use hermes_starknet_chain_components::types::events::packet::PacketRelayEvents;
use hermes_starknet_chain_components::types::messages::ibc::channel::{
    AppVersion, ChannelOrdering, MsgChanOpenAck, MsgChanOpenConfirm, MsgChanOpenInit,
    MsgChanOpenTry, PortId,
};
use hermes_starknet_chain_components::types::messages::ibc::connection::{
    BasePrefix, ConnectionVersion, MsgConnOpenAck, MsgConnOpenConfirm, MsgConnOpenInit,
    MsgConnOpenTry,
};
use hermes_starknet_chain_components::types::messages::ibc::denom::{Denom, PrefixedDenom};
use hermes_starknet_chain_components::types::messages::ibc::ibc_transfer::{
    IbcTransferMessage, Participant,
};
use hermes_starknet_chain_components::types::messages::ibc::packet::{
    MsgRecvPacket, Packet, StateProof,
};
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_components::types::register::{MsgRegisterApp, MsgRegisterClient};
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use hermes_starknet_chain_context::contexts::encoding::event::StarknetEventEncoding;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use ibc::core::connection::types::version::Version as IbcConnectionVersion;
use sha2::{Digest, Sha256};
use starknet::accounts::Call;
use starknet::core::types::Felt;
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

        let starknet_client_state = {
            starknet_chain
                .query_client_state(
                    PhantomData::<CosmosChain>,
                    &starknet_client_id,
                    &starknet_chain.query_chain_height().await?,
                )
                .await?
        };

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

        let cosmos_client_id_as_cairo = {
            let cosmos_client_id_str = cosmos_client_id.to_string();
            let (client_type, sequence_str) = cosmos_client_id_str
                .rsplit_once('-')
                .ok_or_else(|| eyre!("malformatted client id"))?;

            ClientId {
                client_type: Felt::from_bytes_be_slice(client_type.as_bytes()),
                sequence: sequence_str.parse()?,
            }
        };

        // TODO(rano): connection handshake

        let starknet_to_cosmos_relay = StarknetToCosmosRelay::new(
            runtime.clone(),
            starknet_chain.clone(),
            cosmos_chain.clone(),
            starknet_client_id,
            cosmos_client_id,
        );

        let conn_init_option = CosmosInitConnectionOptions {
            delay_period: Duration::from_secs(0),
            connection_version: IbcConnectionVersion::compatibles().first().unwrap().clone(),
        };

        let (starknet_connection_id, cosmos_connection_id) = starknet_to_cosmos_relay
            .bootstrap_connection(&conn_init_option)
            .await?;

        info!("starknet_connection_id: {:?}", starknet_connection_id);
        info!("cosmos_connection_id: {:?}", cosmos_connection_id);

        // TODO(rano): channel handshake
        //
        // let (starknet_channel_id, cosmos_channel_id) = starknet_to_cosmos_relay
        //     .bootstrap_channel(&starknet_connection_id, &cosmos_connection_id)
        //     .await?;

        // info!("starknet_channel_id: {:?}", starknet_channel_id);
        // info!("cosmos_channel_id: {:?}", cosmos_channel_id);

        // let starknet_connection_id_1 = {
        //     let conn_open_init_msg = MsgConnOpenInit {
        //         client_id_on_a: starknet_client_id.clone(),
        //         client_id_on_b: cosmos_client_id_as_cairo.clone(),
        //         prefix_on_b: base_prefix.clone(),
        //         version: connection_version.clone(),
        //         delay_period: 0,
        //     };

        //     info!("selector: {:?}", selector!("conn_open_init"));

        //     let call = Call {
        //         to: ibc_core_address,
        //         selector: selector!("conn_open_init"),
        //         calldata: cairo_encoding.encode(&conn_open_init_msg)?,
        //     };

        //     let message = StarknetMessage::new(call);

        //     let response = starknet_chain.send_message(message).await?;

        //     info!("conn_open_init response: {:?}", response);

        //     let events: Vec<ConnectionHandshakeEvents> =
        //         event_encoding.filter_decode_events(&response.events)?;

        //     assert_eq!(events.len(), 1);

        //     let ConnectionHandshakeEvents::Init(ref conn_init_event) = events[0] else {
        //         panic!("expected a init event from conn_open_init");
        //     };

        //     info!("conn_init_event: {:?}", conn_init_event);

        //     assert_eq!(conn_init_event.client_id_on_a, starknet_client_id);
        //     assert_eq!(conn_init_event.client_id_on_b, cosmos_client_id_as_cairo);

        //     conn_init_event.connection_id_on_a.clone()
        // };

        // let starknet_connection_id_2 = {
        //     // conn_open_try at starknet; as if conn_init was done at cosmos
        //     // to check the connection handshake

        //     let conn_open_try_msg = MsgConnOpenTry {
        //         client_id_on_b: starknet_client_id.clone(),
        //         client_id_on_a: cosmos_client_id_as_cairo.clone(),
        //         conn_id_on_a: starknet_connection_id_1.clone(),
        //         prefix_on_a: base_prefix.clone(),
        //         version_on_a: connection_version.clone(),
        //         proof_conn_end_on_a: StateProof {
        //             proof: vec![Felt::ONE],
        //         },
        //         proof_height_on_a: starknet_client_state.latest_height.clone(),
        //         delay_period: 0,
        //     };

        //     let call = Call {
        //         to: ibc_core_address,
        //         selector: selector!("conn_open_try"),
        //         calldata: cairo_encoding.encode(&conn_open_try_msg)?,
        //     };

        //     let message = StarknetMessage::new(call);

        //     let response = starknet_chain.send_message(message).await?;

        //     info!("conn_open_try response: {:?}", response);

        //     let events: Vec<ConnectionHandshakeEvents> =
        //         event_encoding.filter_decode_events(&response.events)?;

        //     assert_eq!(events.len(), 1);

        //     let ConnectionHandshakeEvents::Try(ref conn_try_event) = events[0] else {
        //         panic!("expected a try event from conn_open_try");
        //     };

        //     info!("conn_try_event: {:?}", conn_try_event);

        //     assert_eq!(conn_try_event.client_id_on_a, cosmos_client_id_as_cairo);
        //     assert_eq!(conn_try_event.connection_id_on_a, starknet_connection_id_1);
        //     assert_eq!(conn_try_event.client_id_on_b, starknet_client_id);

        //     conn_try_event.connection_id_on_b.clone()
        // };

        // {
        //     let conn_open_ack_msg = MsgConnOpenAck {
        //         conn_id_on_a: starknet_connection_id_1.clone(),
        //         conn_id_on_b: starknet_connection_id_2.clone(),
        //         // empty proofs are not accepted
        //         proof_conn_end_on_b: StateProof {
        //             proof: vec![Felt::ONE],
        //         },
        //         proof_height_on_b: starknet_client_state.latest_height.clone(),
        //         version: connection_version.clone(),
        //     };

        //     let call = Call {
        //         to: ibc_core_address,
        //         selector: selector!("conn_open_ack"),
        //         calldata: cairo_encoding.encode(&conn_open_ack_msg)?,
        //     };

        //     let message = StarknetMessage::new(call);

        //     let response = starknet_chain.send_message(message).await?;

        //     info!("conn_open_ack response: {:?}", response);

        //     let events: Vec<ConnectionHandshakeEvents> =
        //         event_encoding.filter_decode_events(&response.events)?;

        //     assert_eq!(events.len(), 1);

        //     let ConnectionHandshakeEvents::Ack(ref conn_ack_event) = events[0] else {
        //         panic!("expected a ack event from conn_open_ack");
        //     };

        //     info!("conn_ack_event: {:?}", conn_ack_event);

        //     assert_eq!(conn_ack_event.client_id_on_a, starknet_client_id);
        //     assert_eq!(conn_ack_event.client_id_on_b, cosmos_client_id_as_cairo);
        //     assert_eq!(conn_ack_event.connection_id_on_a, starknet_connection_id_1);
        //     assert_eq!(conn_ack_event.connection_id_on_b, starknet_connection_id_2);
        // }

        // {
        //     // conn_open_confirm at starknet; as if conn_ack was done at cosmos

        //     let conn_open_confirm_msg = MsgConnOpenConfirm {
        //         conn_id_on_b: starknet_connection_id_2.clone(),
        //         proof_conn_end_on_a: StateProof {
        //             proof: vec![Felt::ONE],
        //         },
        //         proof_height_on_a: starknet_client_state.latest_height.clone(),
        //     };

        //     let call = Call {
        //         to: ibc_core_address,
        //         selector: selector!("conn_open_confirm"),
        //         calldata: cairo_encoding.encode(&conn_open_confirm_msg)?,
        //     };

        //     let message = StarknetMessage::new(call);

        //     let response = starknet_chain.send_message(message).await?;

        //     info!("conn_open_confirm response: {:?}", response);

        //     let events: Vec<ConnectionHandshakeEvents> =
        //         event_encoding.filter_decode_events(&response.events)?;

        //     assert_eq!(events.len(), 1);

        //     let ConnectionHandshakeEvents::Confirm(ref conn_confirm_event) = events[0] else {
        //         panic!("expected a confirm event from conn_open_confirm");
        //     };

        //     info!("conn_confirm_event: {:?}", conn_confirm_event);

        //     assert_eq!(conn_confirm_event.client_id_on_a, cosmos_client_id_as_cairo);
        //     assert_eq!(conn_confirm_event.client_id_on_b, starknet_client_id);
        //     assert_eq!(
        //         conn_confirm_event.connection_id_on_a,
        //         starknet_connection_id_1
        //     );
        //     assert_eq!(
        //         conn_confirm_event.connection_id_on_b,
        //         starknet_connection_id_2
        //     );
        // }

        // let port_id_on_starknet = PortId {
        //     port_id: "transfer".into(),
        // };

        // let port_id_on_cosmos = PortId {
        //     port_id: "transfer".into(),
        // };

        // let app_version = AppVersion {
        //     version: "ics20-1".into(),
        // };

        // {
        //     // register the ICS20 contract with the IBC core contract

        //     let register_app = MsgRegisterApp {
        //         port_id: port_id_on_starknet.clone(),
        //         contract_address: ics20_contract_address,
        //     };

        //     let register_call_data = cairo_encoding.encode(&register_app)?;

        //     let call = Call {
        //         to: ibc_core_address,
        //         selector: selector!("bind_port_id"),
        //         calldata: register_call_data,
        //     };

        //     let message = StarknetMessage::new(call);

        //     let response = starknet_chain.send_message(message).await?;

        //     info!("register ics20 response: {:?}", response);
        // }

        // let starknet_channel_id_1 = {
        //     let chan_open_init_msg = MsgChanOpenInit {
        //         port_id_on_a: port_id_on_cosmos.clone(),
        //         conn_id_on_a: starknet_connection_id_1.clone(),
        //         port_id_on_b: port_id_on_starknet.clone(),
        //         version_proposal: app_version.clone(),
        //         ordering: ChannelOrdering::Unordered,
        //     };

        //     let call = Call {
        //         to: ibc_core_address,
        //         selector: selector!("chan_open_init"),
        //         calldata: cairo_encoding.encode(&chan_open_init_msg)?,
        //     };

        //     let message = StarknetMessage::new(call);

        //     let response = starknet_chain.send_message(message).await?;

        //     info!("chan_open_init response: {:?}", response);

        //     let events: Vec<ChannelHandshakeEvents> =
        //         event_encoding.filter_decode_events(&response.events)?;

        //     assert_eq!(events.len(), 1);

        //     let ChannelHandshakeEvents::Init(ref chan_init_event) = events[0] else {
        //         panic!("expected a init event from chan_open_init");
        //     };

        //     info!("chan_init_event: {:?}", chan_init_event);

        //     assert_eq!(chan_init_event.port_id_on_a, port_id_on_starknet);
        //     assert_eq!(chan_init_event.port_id_on_b, port_id_on_cosmos);
        //     assert_eq!(chan_init_event.connection_id_on_a, starknet_connection_id_1);
        //     assert_eq!(chan_init_event.version_on_a, app_version);

        //     chan_init_event.channel_id_on_a.clone()
        // };

        // let starknet_channel_id_2 = {
        //     // chan_open_try at starknet; as if chan_init was done at cosmos
        //     // to check the channel handshake

        //     let chan_open_try_msg = MsgChanOpenTry {
        //         port_id_on_b: port_id_on_cosmos.clone(),
        //         conn_id_on_b: starknet_connection_id_2.clone(),
        //         port_id_on_a: port_id_on_starknet.clone(),
        //         chan_id_on_a: starknet_channel_id_1.clone(),
        //         version_on_a: app_version.clone(),
        //         proof_chan_end_on_a: StateProof {
        //             proof: vec![Felt::ONE],
        //         },
        //         proof_height_on_a: starknet_client_state.latest_height.clone(),
        //         ordering: ChannelOrdering::Unordered,
        //     };

        //     let call = Call {
        //         to: ibc_core_address,
        //         selector: selector!("chan_open_try"),
        //         calldata: cairo_encoding.encode(&chan_open_try_msg)?,
        //     };

        //     let message = StarknetMessage::new(call);

        //     let response = starknet_chain.send_message(message).await?;

        //     info!("chan_open_try response: {:?}", response);

        //     let events: Vec<ChannelHandshakeEvents> =
        //         event_encoding.filter_decode_events(&response.events)?;

        //     assert_eq!(events.len(), 1);

        //     let ChannelHandshakeEvents::Try(ref chan_try_event) = events[0] else {
        //         panic!("expected a try event from chan_open_try");
        //     };

        //     info!("chan_try_event: {:?}", chan_try_event);

        //     assert_eq!(chan_try_event.port_id_on_a, port_id_on_starknet);
        //     assert_eq!(chan_try_event.channel_id_on_a, starknet_channel_id_1);
        //     assert_eq!(chan_try_event.port_id_on_b, port_id_on_cosmos);
        //     assert_eq!(chan_try_event.connection_id_on_b, starknet_connection_id_2);

        //     chan_try_event.channel_id_on_b.clone()
        // };

        // {
        //     let chan_open_ack_msg = MsgChanOpenAck {
        //         port_id_on_a: port_id_on_starknet.clone(),
        //         chan_id_on_a: starknet_channel_id_1.clone(),
        //         chan_id_on_b: starknet_channel_id_2.clone(),
        //         version_on_b: app_version.clone(),
        //         proof_chan_end_on_b: StateProof {
        //             proof: vec![Felt::ONE],
        //         },
        //         proof_height_on_b: starknet_client_state.latest_height.clone(),
        //     };

        //     let call = Call {
        //         to: ibc_core_address,
        //         selector: selector!("chan_open_ack"),
        //         calldata: cairo_encoding.encode(&chan_open_ack_msg)?,
        //     };

        //     let message = StarknetMessage::new(call);

        //     let response = starknet_chain.send_message(message).await?;

        //     info!("chan_open_ack response: {:?}", response);

        //     let events: Vec<ChannelHandshakeEvents> =
        //         event_encoding.filter_decode_events(&response.events)?;

        //     assert_eq!(events.len(), 1);

        //     let ChannelHandshakeEvents::Ack(ref chan_ack_event) = events[0] else {
        //         panic!("expected a ack event from chan_open_ack");
        //     };

        //     info!("chan_ack_event: {:?}", chan_ack_event);

        //     assert_eq!(chan_ack_event.port_id_on_a, port_id_on_starknet);
        //     assert_eq!(chan_ack_event.port_id_on_b, port_id_on_cosmos);
        //     assert_eq!(chan_ack_event.channel_id_on_a, starknet_channel_id_1);
        //     assert_eq!(chan_ack_event.channel_id_on_b, starknet_channel_id_2);
        //     assert_eq!(chan_ack_event.connection_id_on_a, starknet_connection_id_1);
        // }

        // {
        //     // chan_open_confirm at starknet; as if chan_ack was done at cosmos

        //     let chan_open_confirm_msg = MsgChanOpenConfirm {
        //         port_id_on_b: port_id_on_cosmos.clone(),
        //         chan_id_on_b: starknet_channel_id_2.clone(),
        //         proof_chan_end_on_a: StateProof {
        //             proof: vec![Felt::ONE],
        //         },
        //         proof_height_on_a: starknet_client_state.latest_height.clone(),
        //     };

        //     let call = Call {
        //         to: ibc_core_address,
        //         selector: selector!("chan_open_confirm"),
        //         calldata: cairo_encoding.encode(&chan_open_confirm_msg)?,
        //     };

        //     let message = StarknetMessage::new(call);

        //     let response = starknet_chain.send_message(message).await?;

        //     info!("chan_open_confirm response: {:?}", response);

        //     let events: Vec<ChannelHandshakeEvents> =
        //         event_encoding.filter_decode_events(&response.events)?;

        //     assert_eq!(events.len(), 1);

        //     let ChannelHandshakeEvents::Confirm(ref chan_confirm_event) = events[0] else {
        //         panic!("expected a confirm event from chan_open_confirm");
        //     };

        //     info!("chan_confirm_event: {:?}", chan_confirm_event);

        //     assert_eq!(chan_confirm_event.port_id_on_a, port_id_on_starknet);
        //     assert_eq!(chan_confirm_event.port_id_on_b, port_id_on_cosmos);
        //     assert_eq!(chan_confirm_event.channel_id_on_a, starknet_channel_id_1);
        //     assert_eq!(chan_confirm_event.channel_id_on_b, starknet_channel_id_2);
        //     assert_eq!(
        //         chan_confirm_event.connection_id_on_b,
        //         starknet_connection_id_2
        //     );
        // }

        // // stub
        // let sender_address = "cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng".to_string();

        // let recipient_address = starknet_chain_driver.user_wallet_a.account_address;

        // let amount = 99u32;

        // let mut msg_recv_packet = {
        //     let transfer_message = IbcTransferMessage {
        //         denom: PrefixedDenom {
        //             trace_path: Vec::new(),
        //             base: Denom::Hosted("uatom".into()),
        //         },
        //         amount: amount.into(),
        //         sender: Participant::External(sender_address.clone()),
        //         receiver: Participant::Native(recipient_address),
        //         memo: "".into(),
        //     };

        //     let packet_data = cairo_encoding.encode(&transfer_message)?;

        //     let current_starknet_height = starknet_chain.query_chain_height().await?;
        //     let current_starknet_time = SystemTime::now()
        //         .duration_since(SystemTime::UNIX_EPOCH)?
        //         .as_secs();

        //     let packet = Packet {
        //         sequence: 1,
        //         src_port_id: port_id_on_cosmos.port_id.clone(),
        //         src_channel_id: starknet_channel_id_2.channel_id.clone(),
        //         dst_port_id: port_id_on_starknet.port_id.clone(),
        //         dst_channel_id: starknet_channel_id_1.channel_id.clone(),
        //         data: packet_data,
        //         // both timeout height and timestamp are checked
        //         timeout_height: Height {
        //             revision_number: 0,
        //             revision_height: current_starknet_height + 100,
        //         },
        //         timeout_timestamp: current_starknet_time + 100,
        //     };

        //     MsgRecvPacket {
        //         packet,
        //         proof_commitment_on_a: StateProof {
        //             proof: vec![Felt::ONE],
        //         }, // stub
        //         proof_height_on_a: starknet_client_state.latest_height.clone(),
        //     }
        // };

        // let token_address = {
        //     let calldata = cairo_encoding.encode(&msg_recv_packet)?;

        //     let call = Call {
        //         to: ibc_core_address,
        //         selector: selector!("recv_packet"),
        //         calldata,
        //     };

        //     let message = StarknetMessage::new(call);

        //     let response = starknet_chain.send_message(message.clone()).await?;

        //     info!("IBC transfer response: {:?}", response);

        //     let ibc_packet_events: Vec<PacketRelayEvents> =
        //         event_encoding.filter_decode_events(&response.events)?;

        //     info!("IBC packet events: {:?}", ibc_packet_events);

        //     let ibc_transfer_events: Vec<IbcTransferEvent> =
        //         event_encoding.filter_decode_events(&response.events)?;

        //     info!("IBC transfer events: {:?}", ibc_transfer_events);

        //     {
        //         let receive_transfer_event = ibc_transfer_events
        //             .iter()
        //             .find_map(|event| {
        //                 if let IbcTransferEvent::Receive(event) = event {
        //                     Some(event)
        //                 } else {
        //                     None
        //                 }
        //             })
        //             .ok_or_else(|| eyre!("expect create token event"))?;

        //         assert_eq!(receive_transfer_event.amount, amount.into());

        //         assert_eq!(
        //             receive_transfer_event.sender,
        //             Participant::External(sender_address)
        //         );
        //         assert_eq!(
        //             receive_transfer_event.receiver,
        //             Participant::Native(recipient_address)
        //         );
        //     }

        //     let token_address = {
        //         let create_token_event = ibc_transfer_events
        //             .iter()
        //             .find_map(|event| {
        //                 if let IbcTransferEvent::CreateToken(event) = event {
        //                     Some(event)
        //                 } else {
        //                     None
        //                 }
        //             })
        //             .ok_or_else(|| eyre!("expect create token event"))?;

        //         assert_eq!(create_token_event.initial_supply, amount.into());

        //         let token_address = create_token_event.address;

        //         info!("created token address: {:?}", token_address);

        //         token_address
        //     };

        //     {
        //         let recipient_balance = starknet_chain
        //             .query_token_balance(&token_address, &recipient_address)
        //             .await?;

        //         info!("recipient balance after transfer: {}", recipient_balance);

        //         assert_eq!(recipient_balance.quantity, amount.into());
        //     }

        //     token_address
        // };

        // {
        //     // Send the same transfer message a second time
        //     // but increase the packet sequence number
        //     msg_recv_packet.packet.sequence += 1;

        //     let calldata = cairo_encoding.encode(&msg_recv_packet)?;

        //     let call = Call {
        //         to: ibc_core_address,
        //         selector: selector!("recv_packet"),
        //         calldata,
        //     };

        //     let message = StarknetMessage::new(call);

        //     let response = starknet_chain.send_message(message.clone()).await?;

        //     let ibc_packet_events_2: Vec<PacketRelayEvents> =
        //         event_encoding.filter_decode_events(&response.events)?;

        //     info!("ibc_packet_events 2: {:?}", ibc_packet_events_2);

        //     let ibc_transfer_events_2: Vec<IbcTransferEvent> =
        //         event_encoding.filter_decode_events(&response.events)?;

        //     info!("ibc_transfer_events 2: {:?}", ibc_transfer_events_2);

        //     {
        //         let recipient_balance = starknet_chain
        //             .query_token_balance(&token_address, &recipient_address)
        //             .await?;

        //         info!("recipient balance after transfer: {}", recipient_balance);

        //         assert_eq!(recipient_balance.quantity, (amount * 2).into(),);
        //     }
        // }

        Ok(())
    })
}
