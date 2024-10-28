use std::time::SystemTime;

use eyre::eyre;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
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
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use hermes_starknet_chain_context::contexts::encoding::event::StarknetEventEncoding;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use starknet::accounts::Call;
use starknet::macros::selector;

use crate::contexts::bootstrap::StarknetBootstrap;

#[test]
fn test_starknet_ics20_contract() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let chain_command_path = std::env::var("STARKNET_BIN")
            .unwrap_or("starknet-devnet".into())
            .into();

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let bootstrap = StarknetBootstrap {
            runtime: runtime.clone(),
            chain_command_path,
            chain_store_dir: format!("./test-data/{timestamp}").into(),
        };

        let chain_driver = bootstrap.bootstrap_chain("starknet").await?;

        let chain = &chain_driver.chain;

        let erc20_class_hash = {
            let contract_path = std::env::var("ERC20_CONTRACT")?;

            let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

            let contract = serde_json::from_str(&contract_str)?;

            let class_hash = chain.declare_contract(&contract).await?;

            println!("declared ERC20 class: {:?}", class_hash);

            class_hash
        };

        let ics20_class_hash = {
            let contract_path = std::env::var("ICS20_CONTRACT")?;

            let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

            let contract = serde_json::from_str(&contract_str)?;

            let class_hash = chain.declare_contract(&contract).await?;

            println!("declared ICS20 class: {:?}", class_hash);

            class_hash
        };

        let cairo_encoding = StarknetCairoEncoding;

        let event_encoding = StarknetEventEncoding {
            erc20_hashes: [erc20_class_hash].into(),
            ics20_hashes: [ics20_class_hash].into(),
            ibc_client_hashes: Default::default(),
        };

        let ics20_contract_address = {
            let owner_call_data =
                cairo_encoding.encode(&chain_driver.relayer_wallet.account_address)?;
            let erc20_call_data = cairo_encoding.encode(&erc20_class_hash)?;

            let contract_address = chain
                .deploy_contract(
                    &ics20_class_hash,
                    false,
                    &[owner_call_data, erc20_call_data].concat(),
                )
                .await?;

            println!("deployed ICS20 contract to address: {:?}", contract_address);

            contract_address
        };

        // stub
        let sender_address =
            cairo_encoding.encode(&"cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng".to_string())?;

        let recipient_address = chain_driver.user_wallet_a.account_address;

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
        };

        let token_address = {
            let response = chain.send_message(message.clone()).await?;

            println!("IBC transfer response: {:?}", response);

            let ibc_transfer_events: Vec<IbcTransferEvent> =
                event_encoding.filter_decode_events(&response.events)?;

            println!("IBC transfer events: {:?}", ibc_transfer_events);

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

                println!("created token address: {:?}", token_address);

                token_address
            };

            {
                let recipient_balance = chain
                    .query_token_balance(&token_address, &recipient_address)
                    .await?;

                println!("recipient balance after transfer: {}", recipient_balance);

                assert_eq!(recipient_balance.quantity, amount.into());
            }

            token_address
        };

        {
            // Send the same transfer message a second time
            let response = chain.send_message(message.clone()).await?;

            let ibc_transfer_events_2: Vec<IbcTransferEvent> =
                event_encoding.filter_decode_events(&response.events)?;

            println!("ibc_transfer_events 2: {:?}", ibc_transfer_events_2);

            {
                let recipient_balance = chain
                    .query_token_balance(&token_address, &recipient_address)
                    .await?;

                println!("recipient balance after transfer: {}", recipient_balance);

                assert_eq!(recipient_balance.quantity, (amount * 2).into(),);
            }
        }

        Ok(())
    })
}
