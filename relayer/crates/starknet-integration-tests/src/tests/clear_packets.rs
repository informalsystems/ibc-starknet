use alloc::sync::Arc;
use core::marker::PhantomData;
use core::time::Duration;
use std::io::Write;
use std::path::PathBuf;
use std::time::SystemTime;

use flate2::write::GzEncoder;
use flate2::Compression;
use hermes_chain_components::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use hermes_chain_components::traits::queries::unreceived_packet_sequences::CanQueryUnreceivedPacketSequences;
use hermes_chain_components::traits::types::chain_id::HasChainId;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_chain_components::types::config::gas::eip_type::EipQueryType;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::chain::types::amount::Amount;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_starknet_chain_components::impls::types::message::StarknetMessage;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
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
use ibc::core::connection::types::version::Version as IbcConnectionVersion;
use ibc::core::host::types::identifiers::{PortId as IbcPortId, Sequence};
use sha2::{Digest, Sha256};
use starknet::accounts::{Call, ExecutionEncoding, SingleOwnerAccount};
use starknet::macros::{selector, short_string};
use starknet::providers::Provider;
use starknet::signers::{LocalWallet, SigningKey};
use tracing::info;

use crate::contexts::bootstrap::StarknetBootstrap;
use crate::contexts::osmosis_bootstrap::OsmosisBootstrap;

#[test]
fn test_query_unreceived_packets() -> Result<(), Error> {
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

        let wasm_client_byte_code_gzip = {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&wasm_client_byte_code)?;
            encoder.finish()?
        };

        let cosmos_bootstrap = Arc::new(OsmosisBootstrap {
            runtime: runtime.clone(),
            cosmos_builder,
            should_randomize_identifiers: true,
            chain_store_dir: format!("./test-data/{timestamp}/osmosis").into(),
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

        let mut starknet_chain_driver = starknet_bootstrap.bootstrap_chain("starknet").await?;

        let starknet_chain = &mut starknet_chain_driver.chain;

        info!(
            "started starknet chain at port {}",
            starknet_chain_driver.node_config.rpc_port
        );

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

        starknet_chain.ibc_core_contract_address = Some(ibc_core_address);
        starknet_chain.ibc_client_contract_address = Some(comet_client_address);

        let cairo_encoding = StarknetCairoEncoding;

        starknet_chain.event_encoding = StarknetEventEncoding {
            erc20_hashes: [erc20_class_hash].into(),
            ics20_hashes: [ics20_class_hash].into(),
            ibc_client_hashes: [comet_client_class_hash].into(),
            ibc_core_contract_addresses: [ibc_core_address].into(),
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

        assert_eq!(&client_state_on_starknet.chain_id, cosmos_chain.chain_id());

        let client_state_on_cosmos = cosmos_chain
            .query_client_state_with_latest_height(PhantomData::<StarknetChain>, &cosmos_client_id)
            .await?;

        info!("client state on Cosmos: {:?}", client_state_on_cosmos);

        assert_eq!(
            &client_state_on_cosmos.client_state.chain_id,
            starknet_chain.chain_id()
        );

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

            let register_app = MsgRegisterApp {
                port_id: ics20_port.clone(),
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
                // submit ics20 transfer to Cosmos

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
            wallet_starknet_b.account_address,
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

            // submit ics20 transfer to Cosmos

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
                wallet_starknet_b.account_address,
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

        let pending = <StarknetChain as CanQueryUnreceivedPacketSequences<CosmosChain>>::query_unreceived_packet_sequences(starknet_chain, &starknet_channel_id, &IbcPortId::transfer(), vec!(Sequence::from(1)).as_slice()).await?;

        info!(
            "unreceived sequences: {pending:#?}"
        );

        Ok(())
    })
}
