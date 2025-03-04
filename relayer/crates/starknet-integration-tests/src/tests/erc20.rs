use std::time::SystemTime;

use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_starknet_chain_components::impls::encoding::events::CanFilterDecodeEvents;
use hermes_starknet_chain_components::impls::types::message::StarknetMessage;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::traits::messages::transfer::CanBuildTransferTokenMessage;
use hermes_starknet_chain_components::traits::queries::token_balance::CanQueryTokenBalance;
use hermes_starknet_chain_components::types::amount::StarknetAmount;
use hermes_starknet_chain_components::types::events::erc20::Erc20Event;
use hermes_starknet_chain_components::types::messages::erc20::deploy::DeployErc20TokenMessage;
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use hermes_starknet_chain_context::contexts::encoding::event::StarknetEventEncoding;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use starknet::accounts::Call;
use starknet::core::types::U256;
use starknet::macros::selector;
use tracing::info;

use crate::contexts::bootstrap::StarknetBootstrap;

#[test]
fn test_erc20_transfer() -> Result<(), Error> {
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
            // Test deployment of ERC20 contract
            let contract_path = std::env::var("ERC20_CONTRACT")?;

            let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

            let contract = serde_json::from_str(&contract_str)?;

            let class_hash = chain.declare_contract(&contract).await?;

            info!("declared class: {:?}", class_hash);

            class_hash
        };

        let initial_supply = 1000u32;

        let token_address = {
            let relayer_address = chain_driver.relayer_wallet.account_address;

            let deploy_message = DeployErc20TokenMessage {
                name: "token".into(),
                symbol: "token".into(),
                decimals: 18,
                owner: relayer_address,
            };

            let calldata = StarknetCairoEncoding.encode(&deploy_message)?;

            let token_address = chain
                .deploy_contract(&erc20_class_hash, false, &calldata)
                .await?;

            info!("deployed ERC20 contract to address: {:?}", token_address);

            let calldata = StarknetCairoEncoding.encode(&U256::from(initial_supply))?;

            let call = Call {
                to: *token_address,
                selector: selector!("mint"),
                calldata,
            };

            chain.send_message(StarknetMessage::new(call)).await?;

            info!("Mint {initial_supply} initial supply to address: {relayer_address}");

            let balance = chain
                .query_token_balance(&token_address, &relayer_address)
                .await?;

            assert_eq!(balance.quantity, initial_supply.into());

            token_address
        };

        let event_encoding = StarknetEventEncoding {
            erc20_hashes: [erc20_class_hash].into(),
            ics20_hashes: Default::default(),
            ibc_client_hashes: Default::default(),
            ibc_core_contract_addresses: Default::default(),
        };

        {
            // Test local ERC20 token transfer
            let account_address = chain_driver.relayer_wallet.account_address;

            let recipient_address = chain_driver.user_wallet_a.account_address;

            info!("sender address: {:?}", account_address);
            info!("recipient address: {:?}", recipient_address);

            let sender_balance_a = chain
                .query_token_balance(&token_address, &account_address)
                .await?;

            info!("sender balance before: {}", sender_balance_a);

            let recipient_balance_a = chain
                .query_token_balance(&token_address, &recipient_address)
                .await?;

            info!("recipient balance before: {}", recipient_balance_a);

            let transfer_amount = 100u32.into();

            let message = chain.build_transfer_token_message(
                &recipient_address,
                &StarknetAmount::new(transfer_amount, token_address),
            )?;

            let response = chain.send_message(message).await?;

            info!("performed transfer of 100 tokens");

            info!("response: {:?}", response);

            let erc20_events: Vec<Erc20Event> =
                event_encoding.filter_decode_events(&response.events)?;

            info!(
                "events from sending transfer token message: {:?}",
                erc20_events
            );

            match &erc20_events[0] {
                Erc20Event::Transfer(transfer) => {
                    assert_eq!(transfer.from, account_address);
                    assert_eq!(transfer.to, recipient_address);
                    assert_eq!(transfer.value, transfer_amount);
                }
                _ => {
                    panic!("expected a Transfer event to be emitted");
                }
            }

            let sender_balance_b = chain
                .query_token_balance(&token_address, &account_address)
                .await?;

            info!("sender balance after transfer: {}", sender_balance_b);

            let recipient_balance_b = chain
                .query_token_balance(&token_address, &recipient_address)
                .await?;

            info!("recipient balance transfer: {}", recipient_balance_b);

            assert_eq!(
                sender_balance_b.quantity,
                sender_balance_a.quantity - transfer_amount
            );
            assert_eq!(
                recipient_balance_b.quantity,
                recipient_balance_a.quantity + transfer_amount
            );
        }

        Ok(())
    })
}
