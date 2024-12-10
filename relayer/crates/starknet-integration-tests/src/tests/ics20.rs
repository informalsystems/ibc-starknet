use alloc::sync::Arc;
use std::path::PathBuf;
use std::time::SystemTime;

use cgp::prelude::*;
use eyre::eyre;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_wasm_relayer::context::cosmos_bootstrap::CosmosWithWasmClientBootstrap;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_starknet_chain_components::impls::encoding::events::CanFilterDecodeEvents;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::traits::queries::token_balance::CanQueryTokenBalance;
use hermes_starknet_chain_components::types::cosmos::height::Height;
use hermes_starknet_chain_components::types::events::ics20::IbcTransferEvent;
use hermes_starknet_chain_components::types::messages::ibc::denom::{Denom, PrefixedDenom};
use hermes_starknet_chain_components::types::messages::ibc::ibc_transfer::{
    IbcTransferMessage, Participant,
};
use hermes_starknet_chain_components::types::messages::ibc::packet::Packet;
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use hermes_starknet_chain_context::contexts::encoding::event::StarknetEventEncoding;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use sha2::{Digest, Sha256};
use starknet::accounts::Call;
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
            ibc_client_hashes: Default::default(),
        };

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

        let _starknet_to_cosmos_relay = StarknetToCosmosRelay {
            runtime: runtime.clone(),
            src_chain: starknet_chain.clone(),
            dst_chain: cosmos_chain.clone(),
            src_client_id: starknet_client_id.clone(),
            dst_client_id: cosmos_client_id.clone(),
        };

        let ics20_contract_address = {
            let owner_call_data =
                cairo_encoding.encode(&starknet_chain_driver.relayer_wallet.account_address)?;
            // // TODO(rano): when we are using ibc-core handler, ibc-core is the owner
            // let owner_call_data = cairo_encoding.encode(&ibc_core_address)?;
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

        {
            // register the ICS20 contract with the IBC core contract

            let register_call_data =
                cairo_encoding.encode(&product![b"transfer".to_vec(), ics20_contract_address])?;

            let message = Call {
                to: ibc_core_address,
                selector: selector!("bind_port_id"),
                calldata: register_call_data,
            };

            let response = starknet_chain.send_message(message).await?;

            info!("register ics20 response: {:?}", response);
        }

        // TODO(rano): connection open init

        // TODO(rano): connection open ack

        // TODO(rano): channel open init

        // TODO(rano): channel open ack

        // stub
        let sender_address = "cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng".to_string();

        let recipient_address = starknet_chain_driver.user_wallet_a.account_address;

        let amount = 99u32;

        let message = {
            let transfer_message = IbcTransferMessage {
                denom: PrefixedDenom {
                    trace_path: Vec::new(),
                    base: Denom::Hosted("uatom".into()),
                },
                amount: amount.into(),
                sender: Participant::External(sender_address.clone()),
                receiver: Participant::Native(recipient_address),
                memo: "".into(),
            };

            let packet_data = cairo_encoding.encode(&transfer_message)?;

            let packet = Packet {
                sequence: 1,
                src_port_id: "transfer".into(),
                src_channel_id: "channel-1".into(),
                dst_port_id: "transfer".into(),
                dst_channel_id: "channel-2".into(),
                data: packet_data,
                timeout_height: Height {
                    revision_number: 0,
                    revision_height: 100,
                },
                timeout_timestamp: 0,
            };

            let calldata = cairo_encoding.encode(&packet)?;

            Call {
                to: ics20_contract_address,
                selector: selector!("on_recv_packet"),
                calldata,
            }

            // // TODO(rano): when ibc-core can create channels, this will be the correct message

            // let msg_recv_packet = MsgRecvPacket {
            //     packet,
            //     proof_commitment_on_a: StateProof { proof: Vec::new() }, // stub
            //     proof_height_on_a: Height {
            //         // stub
            //         revision_number: 0,
            //         revision_height: 0,
            //     },
            // };

            // let calldata = cairo_encoding.encode(&msg_recv_packet)?;

            // Call {
            //     to: ibc_core_address,
            //     selector: selector!("recv_packet"),
            //     calldata,
            // }
        };

        let token_address = {
            let response = starknet_chain.send_message(message.clone()).await?;

            info!("IBC transfer response: {:?}", response);

            let ibc_transfer_events: Vec<IbcTransferEvent> =
                event_encoding.filter_decode_events(&response.events)?;

            info!("IBC transfer events: {:?}", ibc_transfer_events);

            {
                let receive_transfer_event = ibc_transfer_events
                    .iter()
                    .find_map(|event| {
                        if let IbcTransferEvent::Receive(event) = event {
                            Some(event)
                        } else {
                            None
                        }
                    })
                    .ok_or_else(|| eyre!("expect create token event"))?;

                assert_eq!(receive_transfer_event.amount, amount.into());

                assert_eq!(
                    receive_transfer_event.sender,
                    Participant::External(sender_address)
                );
                assert_eq!(
                    receive_transfer_event.receiver,
                    Participant::Native(recipient_address)
                );
            }

            let token_address = {
                let create_token_event = ibc_transfer_events
                    .iter()
                    .find_map(|event| {
                        if let IbcTransferEvent::CreateToken(event) = event {
                            Some(event)
                        } else {
                            None
                        }
                    })
                    .ok_or_else(|| eyre!("expect create token event"))?;

                assert_eq!(create_token_event.initial_supply, amount.into());

                let token_address = create_token_event.address;

                info!("created token address: {:?}", token_address);

                token_address
            };

            {
                let recipient_balance = starknet_chain
                    .query_token_balance(&token_address, &recipient_address)
                    .await?;

                info!("recipient balance after transfer: {}", recipient_balance);

                assert_eq!(recipient_balance.quantity, amount.into());
            }

            token_address
        };

        {
            // Send the same transfer message a second time
            let response = starknet_chain.send_message(message.clone()).await?;

            let ibc_transfer_events_2: Vec<IbcTransferEvent> =
                event_encoding.filter_decode_events(&response.events)?;

            info!("ibc_transfer_events 2: {:?}", ibc_transfer_events_2);

            {
                let recipient_balance = starknet_chain
                    .query_token_balance(&token_address, &recipient_address)
                    .await?;

                info!("recipient balance after transfer: {}", recipient_balance);

                assert_eq!(recipient_balance.quantity, (amount * 2).into(),);
            }
        }

        Ok(())
    })
}
