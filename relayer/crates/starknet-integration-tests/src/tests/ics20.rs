use core::marker::PhantomData;
use core::time::Duration;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use cgp::extra::run::CanRun;
use cgp::prelude::*;
use flate2::write::GzEncoder;
use flate2::Compression;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainStatus;
use hermes_chain_components::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use hermes_chain_components::traits::types::chain_id::HasChainId;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_chain_components::types::config::gas::eip_type::EipQueryType;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::chain::impls::transfer::amount::derive_ibc_denom;
use hermes_cosmos_test_components::chain::types::amount::Amount;
use hermes_cosmos_test_components::chain::types::denom::Denom as IbcDenom;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::impls::types::message::StarknetMessage;
use hermes_starknet_chain_components::traits::contract::call::CanCallContract;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::traits::queries::token_balance::CanQueryTokenBalance;
use hermes_starknet_chain_components::types::amount::StarknetAmount;
use hermes_starknet_chain_components::types::cosmos::height::Height;
use hermes_starknet_chain_components::types::cosmos::timestamp::Timestamp;
use hermes_starknet_chain_components::types::messages::ibc::denom::{
    Denom, PrefixedDenom, TracePrefix,
};
use hermes_starknet_chain_components::types::messages::ibc::ibc_transfer::MsgTransfer;
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_components::types::register::{MsgRegisterApp, MsgRegisterClient};
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use hermes_starknet_chain_context::contexts::encoding::event::StarknetEventEncoding;
use hermes_starknet_chain_context::impls::error::SignError;
use hermes_starknet_relayer::contexts::cosmos_to_starknet_relay::CosmosToStarknetRelay;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;
use ibc::core::connection::types::version::Version as IbcConnectionVersion;
use ibc::core::host::types::identifiers::PortId as IbcPortId;
use poseidon::Poseidon3Hasher;
use sha2::{Digest, Sha256};
use starknet::accounts::{Account, AccountError, Call, ExecutionEncoding, SingleOwnerAccount};
use starknet::core::types::U256;
use starknet::macros::{selector, short_string};
use starknet::providers::Provider;
use starknet::signers::{LocalWallet, SigningKey};
use tracing::info;

use crate::contexts::bootstrap::StarknetBootstrap;
use crate::contexts::osmosis_bootstrap::OsmosisBootstrap;

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

        let starknet_chain = {
            let mut fields = starknet_chain.fields.as_ref().clone();

            fields.ibc_core_contract_address = Some(ibc_core_address);
            fields.ibc_client_contract_address = Some(comet_client_address);

            let cairo_encoding = StarknetCairoEncoding;

            fields.event_encoding = StarknetEventEncoding {
                erc20_hashes: [erc20_class_hash].into(),
                ics20_hashes: [ics20_class_hash].into(),
                ibc_client_hashes: [comet_client_class_hash].into(),
                ibc_core_contract_addresses: [ibc_core_address].into(),
            };

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

        let starknet_client_id = StarknetToCosmosRelay::create_client(
            SourceTarget,
            &starknet_chain,
            cosmos_chain,
            &Default::default(),
            &(),
        )
        .await?;

        info!("created client on Starknet: {:?}", starknet_client_id);

        let cosmos_client_id = StarknetToCosmosRelay::create_client(
            DestinationTarget,
            cosmos_chain,
            &starknet_chain,
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

        {
            let starknet_to_cosmos_relay = starknet_to_cosmos_relay.clone();

            let cosmos_to_starknet_relay = cosmos_to_starknet_relay.clone();

            runtime.runtime.spawn(async move {
                let _ = starknet_to_cosmos_relay.run().await;
            });

            runtime.runtime.spawn(async move {
                let _ = cosmos_to_starknet_relay.run().await;
            });
        }

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
                to: *ibc_core_address,
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

        let denom = PrefixedDenom {
            trace_path: vec![TracePrefix {
                port_id: ics20_port.to_string(),
                channel_id: starknet_channel_id.to_string(),
            }],
            base: Denom::Hosted(denom_cosmos.to_string()),
        };

        let ics20_token_address: StarknetAddress = {
            let calldata = cairo_encoding.encode(&product![denom.clone()])?;

            let message = StarknetMessage {
                call: Call {
                    to: *ics20_contract_address,
                    selector: selector!("create_ibc_token"),
                    calldata,
                },
                counterparty_height: None,
            };

            let message_response = starknet_chain.send_message(message).await?;

            cairo_encoding.decode(&message_response.result)?
        };

        info!("ics20 token address: {:?}", ics20_token_address);

        let expected_ics20_token_address: StarknetAddress = {
            let ibc_prefixed_denom = PrefixedDenom {
                trace_path: vec![TracePrefix {
                    port_id: ics20_port.to_string(),
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

        assert_eq!(ics20_token_address, expected_ics20_token_address);

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

        // cosmos_to_starknet_relay.relay_packet(&packet).await?;

        let balance_cosmos_a_step_1 = cosmos_chain
            .query_balance(address_cosmos_a, denom_cosmos)
            .await?;

        info!("cosmos balance after transfer: {}", balance_cosmos_a_step_1);

        assert_eq!(
            balance_cosmos_a_step_0.quantity,
            balance_cosmos_a_step_1.quantity + transfer_quantity
        );

        let balance_starknet_b_step_1 =
            StarknetAmount::new(transfer_quantity.into(), ics20_token_address);

        starknet_chain
            .assert_eventual_amount(address_starknet_b, &balance_starknet_b_step_1)
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

            let execution = starknet_account_b.execute_v3(vec![call]);

            let tx_hash = execution
                .send()
                .await
                .map_err(<StarknetChain as CanRaiseError<AccountError<SignError>>>::raise_error)?
                .transaction_hash;

            starknet_chain.poll_tx_response(&tx_hash).await?;
        }

        // create ibc transfer message

        let starknet_ics20_send_message = {
            let current_starknet_time = starknet_chain.query_chain_status().await?.time;

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
                    u64::try_from(current_starknet_time.unix_timestamp() + 1800).unwrap()
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

            let execution = starknet_account_b.execute_v3(vec![call]);

            let tx_hash = execution
                .send()
                .await
                .map_err(<StarknetChain as CanRaiseError<AccountError<SignError>>>::raise_error)?
                .transaction_hash;

            starknet_chain.poll_tx_response(&tx_hash).await?;
        };

        cosmos_chain
            .assert_eventual_amount(address_cosmos_a, &balance_cosmos_a_step_0)
            .await?;

        let balance_starknet_b_step_2 = starknet_chain
            .query_token_balance(&ics20_token_address, address_starknet_b)
            .await?;

        info!(
            "starknet balance after transfer back: {}",
            balance_starknet_b_step_2
        );

        assert_eq!(balance_starknet_b_step_2.quantity, 0u64.into());

        // send starknet erc20 token to cosmos

        let erc20_token_address = &starknet_chain_driver.genesis_config.transfer_denom;

        info!("erc20 token address: {:?}", erc20_token_address);

        let balance_starknet_step_0 = starknet_chain
            .query_token_balance(erc20_token_address, address_starknet_b)
            .await?;

        info!("erc20 balance on starknet: {}", balance_starknet_step_0);

        {
            // approve ics20 contract to spend the tokens for address_starknet_b
            let call_data = cairo_encoding.encode(&product![
                ics20_contract_address,
                U256::from(transfer_quantity)
            ])?;

            let call = Call {
                to: **erc20_token_address,
                selector: selector!("approve"),
                calldata: call_data,
            };

            let execution = starknet_account_b.execute_v3(vec![call]);

            let tx_hash = execution
                .send()
                .await
                .map_err(<StarknetChain as CanRaiseError<AccountError<SignError>>>::raise_error)?
                .transaction_hash;

            starknet_chain.poll_tx_response(&tx_hash).await?;
        }

        // submit ics20 transfer from Starknet to Cosmos

        let starknet_ics20_send_message = {
            let current_starknet_time = starknet_chain.query_chain_status().await?.time;

            let denom = PrefixedDenom {
                trace_path: vec![],
                base: Denom::Native(*erc20_token_address),
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
                    u64::try_from(current_starknet_time.unix_timestamp() + 1800).unwrap()
                        * 1_000_000_000,
                ),
            }
        };

        {
            let call_data = cairo_encoding.encode(&starknet_ics20_send_message)?;

            let call = Call {
                to: *ics20_contract_address,
                selector: selector!("send_transfer"),
                calldata: call_data,
            };

            let execution = starknet_account_b.execute_v3(vec![call]);

            let tx_hash = execution
                .send()
                .await
                .map_err(<StarknetChain as CanRaiseError<AccountError<SignError>>>::raise_error)?
                .transaction_hash;

            starknet_chain.poll_tx_response(&tx_hash).await?;
        };

        let cosmos_ibc_denom = derive_ibc_denom(
            &ics20_port,
            &cosmos_channel_id,
            &IbcDenom::base(&erc20_token_address.to_string()),
        )?;

        info!("cosmos ibc denom: {:?}", cosmos_ibc_denom);

        cosmos_chain
            .assert_eventual_amount(
                address_cosmos_a,
                &Amount::new(transfer_quantity, cosmos_ibc_denom.clone()),
            )
            .await?;

        let balance_starknet_relayer_step_3 = starknet_chain
            .query_token_balance(erc20_token_address, address_starknet_b)
            .await?;

        info!(
            "starknet balance after transfer from starknet: {}",
            balance_starknet_relayer_step_3
        );

        assert_eq!(
            balance_starknet_relayer_step_3.quantity,
            balance_starknet_step_0.quantity - transfer_quantity.into()
        );

        // send the tokens back to starknet

        let _packet = <CosmosChain as CanIbcTransferToken<StarknetChain>>::ibc_transfer_token(
            cosmos_chain,
            &cosmos_channel_id,
            &IbcPortId::transfer(),
            wallet_cosmos_a,
            address_starknet_b,
            &Amount::new(transfer_quantity, cosmos_ibc_denom.clone()),
            &None,
        )
        .await?;

        let balance_cosmos_a_step_4 = cosmos_chain
            .query_balance(address_cosmos_a, &cosmos_ibc_denom)
            .await?;

        info!(
            "cosmos balance after transfer back to starknet: {}",
            balance_cosmos_a_step_4
        );

        assert_eq!(balance_cosmos_a_step_4.quantity, 0u64.into());

        starknet_chain
            .assert_eventual_amount(address_starknet_b, &balance_starknet_step_0)
            .await?;

        Ok(())
    })
}
